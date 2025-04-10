use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::try_join;

use crate::supabase_jwt_middleware::User;
use crate::AppState;
use serde_json::json;

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

//Task
pub async fn get_tasks(
    Path(account_id): Path<String>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[TASKS] Handling a get_tasks for account_id: {}",
        account_id
    );

    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let client = &state.anything_client;

    let get_count = async {
        client
            .from("tasks")
            .auth(&user.jwt)
            .eq("account_id", &account_id)
            .select("*")
            .exact_count()
            .execute()
            .await
    };

    let get_data = async {
        client
            .from("tasks")
            .auth(&user.jwt)
            .eq("account_id", &account_id)
            .select("*")
            .range(offset as usize, (offset + page_size) as usize)
            .order("created_at.desc,processing_order.desc")
            .execute()
            .await
    };

    let (count_response, data_response) = match try_join!(get_count, get_data) {
        Ok((count, data)) => (count, data),
        Err(err) => {
            println!("[TASKS] Failed to execute requests: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute requests",
            )
                .into_response();
        }
    };

    let total_count = count_response
        .headers()
        .get("content-range")
        .and_then(|h| h.to_str().ok())
        .and_then(|range| range.split('/').last())
        .and_then(|count| count.parse::<i64>().ok())
        .unwrap_or(0);

    println!("[TASKS] Total count from header: {}", total_count);

    if data_response.status() == 204 {
        println!("[TASKS] No content for account_id: {}", account_id);
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let data_body = match data_response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("[TASKS] Failed to read data response: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read data response",
            )
                .into_response();
        }
    };

    // println!("[TASKS] Data response body: {}", data_body);

    let items: Value = match serde_json::from_str(&data_body) {
        Ok(items) => items,
        Err(err) => {
            println!("[TASKS] Failed to parse data JSON: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse data JSON",
            )
                .into_response();
        }
    };

    let response_with_meta = json!({
        "data": if items.is_array() && items.as_array().unwrap().is_empty() {
            json!([])
        } else {
            items
        },
        "pagination": {
            "page": page,
            "page_size": page_size,
            "total": total_count
        }
    });

    Json(response_with_meta).into_response()
}

pub async fn get_task_by_workflow_id(
    Path((account_id, workflow_id)): Path<(String, String)>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[TASKS] Handling get_task_by_workflow_id for account_id: {}, workflow_id: {}",
        account_id, workflow_id
    );

    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let client = &state.anything_client;

    let get_count = async {
        client
            .from("tasks")
            .auth(&user.jwt)
            .eq("account_id", &account_id)
            .eq("flow_id", &workflow_id)    
            .select("*")
            .planned_count()
            .execute()
            .await
    };

    let get_data = async {
        client
            .from("tasks")
            .auth(&user.jwt)
            .eq("account_id", &account_id)
            .eq("flow_id", &workflow_id)
            .select("*")
            .range(offset as usize, (offset + page_size) as usize)
            .order("created_at.desc,processing_order.desc")
            .execute()
            .await
    };

    let (count_response, data_response) = match try_join!(get_count, get_data) {
        Ok((count, data)) => (count, data),
        Err(err) => {
            println!("[TASKS] Failed to execute requests: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute requests",
            )
                .into_response();
        }
    };

    let total_count = count_response
        .headers()
        .get("content-range")
        .and_then(|h| h.to_str().ok())
        .and_then(|range| range.split('/').last())
        .and_then(|count| count.parse::<i64>().ok())
        .unwrap_or(0);

    println!("[TASKS] Total count from header: {}", total_count);

    if data_response.status() == 204 {
        println!("[TASKS] No content for account_id: {}", account_id);
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let data_body = match data_response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("[TASKS] Failed to read data response: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read data response",
            )
                .into_response();
        }
    };

    println!("[TASKS] Data response body: {}", data_body);

    let items: Value = match serde_json::from_str(&data_body) {
        Ok(items) => items,
        Err(err) => {
            println!("[TASKS] Failed to parse data JSON: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse data JSON",
            )
                .into_response();
        }
    };

    let response_with_meta = json!({
        "data": if items.is_array() && items.as_array().unwrap().is_empty() {
            json!([])
        } else {
            items
        },
        "pagination": {
            "page": page,
            "page_size": page_size,
            "total": total_count
        }
    });

    Json(response_with_meta).into_response()
}

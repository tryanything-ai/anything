use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::custom_auth::User;
use crate::AppState;
use crate::entities::tasks;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, QuerySelect, PaginatorTrait};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    page_size: Option<i64>,
    search: Option<String>,
}

// Simplified get_tasks using SeaORM
pub async fn get_tasks(
    Path(account_id): Path<String>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TASKS] Handling get_tasks for account_id: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response();
        }
    };

    let page = pagination.page.unwrap_or(1).max(1);
    let page_size = pagination.page_size.unwrap_or(20).min(100);

    let mut query = tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid));

    // Add search filter if provided
    if let Some(search) = pagination.search {
        if !search.is_empty() {
            query = query.filter(tasks::Column::ActionLabel.contains(&search));
        }
    }

    // Get total count
    let total_count = match query.clone().count(&*state.db).await {
        Ok(count) => count,
        Err(err) => {
            println!("[TASKS] Failed to get count: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    if total_count == 0 {
        return Json(json!({
            "data": [],
            "total_count": 0,
            "page": page,
            "page_size": page_size
        })).into_response();
    }

    // Get paginated data
    let tasks_data = match query
        .order_by_desc(tasks::Column::CreatedAt)
        .paginate(&*state.db, page_size as u64)
        .fetch_page((page - 1) as u64)
        .await
    {
        Ok(data) => data,
        Err(err) => {
            println!("[TASKS] Failed to get tasks: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Convert to JSON response
    let tasks_json: Vec<Value> = tasks_data
        .into_iter()
        .map(|task| json!({
            "task_id": task.task_id,
            "account_id": task.account_id,
            "task_status": task.task_status,
            "flow_id": task.flow_id,
            "flow_version_id": task.flow_version_id,
            "action_label": task.action_label,
            "trigger_id": task.trigger_id,
            "created_at": task.created_at,
            "updated_at": task.updated_at,
            "started_at": task.started_at,
            "ended_at": task.ended_at,
            "stage": task.stage,
            "processing_order": task.processing_order
        }))
        .collect();

    Json(json!({
        "data": tasks_json,
        "total_count": total_count,
        "page": page,
        "page_size": page_size
    })).into_response()
}

// Simplified get_task_and_context using SeaORM  
pub async fn get_task_and_context(
    Path((account_id, task_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TASKS] Handling get_task_and_context for task_id: {}", task_id);

    let task_uuid = match Uuid::parse_str(&task_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid task ID").into_response();
        }
    };

    let task = match tasks::Entity::find_by_id(task_uuid)
        .one(&*state.db)
        .await
    {
        Ok(Some(task)) => task,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Task not found").into_response();
        }
        Err(err) => {
            println!("[TASKS] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let response = json!({
        "task_id": task.task_id,
        "account_id": task.account_id,
        "task_status": task.task_status,
        "flow_id": task.flow_id,
        "flow_version_id": task.flow_version_id,
        "action_label": task.action_label,
        "config": task.config,
        "context": task.context,
        "result": task.result,
        "debug_result": task.debug_result,
        "error": task.error,
        "created_at": task.created_at,
        "updated_at": task.updated_at,
        "started_at": task.started_at,
        "ended_at": task.ended_at
    });

    Json(response).into_response()
}

// Simplified get_account_tasks_count using SeaORM
pub async fn get_account_tasks_count(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TASKS] Handling get_account_tasks_count for account_id: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response();
        }
    };

    let total_count = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .count(&*state.db)
        .await
    {
        Ok(count) => count,
        Err(err) => {
            println!("[TASKS] Failed to get count: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    Json(json!({
        "total_count": total_count
    })).into_response()
}

// Simplified get_workflow_tasks using SeaORM
pub async fn get_workflow_tasks(
    Path((account_id, workflow_id)): Path<(String, String)>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TASKS] Handling get_workflow_tasks for workflow_id: {}", workflow_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response();
        }
    };

    let workflow_uuid = match Uuid::parse_str(&workflow_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid workflow ID").into_response();
        }
    };

    let tasks_data = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .filter(tasks::Column::FlowId.eq(workflow_uuid))
        .order_by_desc(tasks::Column::CreatedAt)
        .limit(100) // Reasonable limit
        .all(&*state.db)
        .await
    {
        Ok(data) => data,
        Err(err) => {
            println!("[TASKS] Failed to get workflow tasks: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let tasks_json: Vec<Value> = tasks_data
        .into_iter()
        .map(|task| json!({
            "task_id": task.task_id,
            "account_id": task.account_id,
            "task_status": task.task_status,
            "flow_id": task.flow_id,
            "action_label": task.action_label,
            "created_at": task.created_at,
            "updated_at": task.updated_at
        }))
        .collect();

    Json(tasks_json).into_response()
}

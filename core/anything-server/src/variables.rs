use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_auth_middleware::User;
use crate::AppState;

// Actions
pub async fn get_flow_version_variables(
    Path((account_id, workflow_id, workflow_version_id, action_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[VARIABLES] Handling get_flow_version_variables request for account: {}, workflow: {}, version: {}, action: {}", 
        account_id, workflow_id, workflow_version_id, action_id);

    let client = &state.anything_client;

    //TODO: we need to get what action_id's are done before the action_id that was sent.

    // Get last session
    println!("[VARIABLES] Fetching last task for workflow");
    let response = match client
        .from("tasks")
        .auth(user.jwt.clone())
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("created_at.desc")
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "[VARIABLES] Response from fetching last task: {:?}",
                response
            );
            response
        }
        Err(e) => {
            println!("[VARIABLES] Error fetching last task: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[VARIABLES] Error reading response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let task: Value = match serde_json::from_str::<Vec<Value>>(&body) {
        Ok(tasks) => {
            if tasks.is_empty() {
                println!("[VARIABLES] No tasks found");
                return (StatusCode::NOT_FOUND, "No tasks found").into_response();
            }
            println!("[VARIABLES] First task: {:?}", tasks[0]);
            tasks[0].clone()
        }
        Err(e) => {
            println!("[VARIABLES] Error parsing JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    let session_id = match task.get("flow_session_id") {
        Some(id) => id.as_str().unwrap_or("").to_string(),
        None => {
            println!("[VARIABLES] Failed to get flow_session_id from task");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get flow_session_id",
            )
                .into_response();
        }
    };

    println!("[VARIABLES] Found session_id: {}", session_id);
    println!("[VARIABLES] Fetching tasks for session");

    let response = match client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("flow_session_id", &session_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("processing_order.asc")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[VARIABLES] Error fetching tasks: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[VARIABLES] Error reading tasks response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => {
            println!("[VARIABLES] Parsed items: {:?}", items);
            items
        }
        Err(e) => {
            println!("[VARIABLES] Error parsing tasks JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    // Find the processing order of the target action
    let target_processing_order = items
        .as_array()
        .and_then(|tasks| {
            tasks
                .iter()
                .find(|task| task.get("action_id").and_then(|id| id.as_str()) == Some(&action_id))
        })
        .and_then(|task| task.get("processing_order"))
        .and_then(|order| order.as_i64());

    println!(
        "[VARIABLES] Found target processing order: {:?}",
        target_processing_order
    );

    // Filter tasks to only include those with lower processing order
    let filtered_items = match target_processing_order {
        Some(target_order) => Value::Array(
            items
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter(|task| {
                    task.get("processing_order")
                        .and_then(|order| order.as_i64())
                        .map_or(false, |order| order < target_order)
                })
                .cloned()
                .collect(),
        ),
        None => items,
    };

    let items = filtered_items;

    let result = serde_json::json!({
        "tasks": items
    });

    println!("[VARIABLES] Returning response");
    Json(result).into_response()
}

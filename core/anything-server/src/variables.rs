use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    bundler::bundle_cached_inputs,
    supabase_jwt_middleware::User,
    types::{
        task_types::Task,
        workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
    },
    AppState,
};

// Actions
pub async fn get_flow_version_results(
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

    let task: Task = match serde_json::from_str::<Vec<Task>>(&body) {
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

    let session_id = task.flow_session_id;

    println!("[VARIABLES] Found session_id: {}", session_id);
    println!("[VARIABLES] Fetching tasks for session");

    let response = match client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("flow_session_id", &session_id.to_string())
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

    let items: Vec<Task> = match serde_json::from_str(&body) {
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
        .iter()
        .find(|task| task.action_id == action_id)
        .map(|task| task.processing_order);

    println!(
        "[VARIABLES] Found target processing order: {:?}",
        target_processing_order
    );

    // Filter tasks to only include those with lower processing order
    let filtered_items = match target_processing_order {
        Some(target_order) => items
            .iter()
            .filter(|task| task.processing_order < target_order)
            .cloned()
            .collect(),
        None => items,
    };

    let items = filtered_items;

    let result = serde_json::json!({
        "tasks": items
    });

    println!("[VARIABLES] Returning response");
    Json(result).into_response()
}

// Inputs
pub async fn get_flow_version_inputs(
    Path((account_id, workflow_id, workflow_version_id, action_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[INPUTS] Handling get_flow_version_inputs request");

    let client = &state.anything_client;

    // First get the flow version and action - we need this regardless of task history
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .select("*")
        .eq("flow_version_id", &workflow_version_id)
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[INPUTS] Error fetching flow version: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch flow version",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[INPUTS] Error reading flow version response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read flow version response",
            )
                .into_response();
        }
    };

    let flow_version = match serde_json::from_str::<Vec<DatabaseFlowVersion>>(&body) {
        Ok(versions) => match versions.into_iter().next() {
            Some(version) => version,
            None => {
                return (StatusCode::NOT_FOUND, "Flow version not found").into_response();
            }
        },
        Err(e) => {
            println!("[INPUTS] Error parsing flow version JSON: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse flow version",
            )
                .into_response();
        }
    };

    // Parse the flow definition and find the action
    let workflow: WorkflowVersionDefinition = flow_version.flow_definition;
    let action = match workflow.actions.iter().find(|a| a.action_id == action_id) {
        Some(action) => action,
        None => {
            return (StatusCode::NOT_FOUND, "Action not found").into_response();
        }
    };

    // Start building our response with basic action input information
    let mut response_data = serde_json::json!({
        "action_id": action.action_id,
        "inputs": action.inputs,
        "inputs_schema": action.inputs_schema,
        "has_task_history": false
    });

    // Try to get the last task and enrich with actual values if available
    let task_response = client
        .from("tasks")
        .auth(user.jwt.clone())
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("created_at.desc")
        .limit(1)
        .execute()
        .await;

    if let Ok(task_response) = task_response {
        if let Ok(task_body) = task_response.text().await {
            if let Ok(tasks) = serde_json::from_str::<Vec<Value>>(&task_body) {
                if let Some(last_task) = tasks.first() {
                    response_data["has_task_history"] = json!(true);

                    // If we have a task, try to get rendered inputs
                    if let Some(session_id) =
                        last_task.get("flow_session_id").and_then(|v| v.as_str())
                    {
                        if let Some(inputs) = &action.inputs {
                            if let Some(inputs_schema) = &action.inputs_schema {
                                match bundle_cached_inputs(
                                    state.clone(),
                                    client,
                                    &account_id,
                                    session_id,
                                    Some(inputs),
                                    Some(inputs_schema),
                                    false,
                                )
                                .await
                                {
                                    Ok(rendered_vars) => {
                                        response_data["rendered_inputs"] = json!(rendered_vars);
                                        response_data["last_task"] = json!({
                                            "task_id": last_task.get("task_id"),
                                            "created_at": last_task.get("created_at"),
                                            "status": last_task.get("status"),
                                        });
                                    }
                                    Err(e) => {
                                        println!("[INPUTS] Error rendering inputs: {:?}", e);
                                        response_data["render_error"] = json!(e.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("[INPUTS] Returning response");
    Json(response_data).into_response()
}

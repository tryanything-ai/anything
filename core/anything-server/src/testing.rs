use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use chrono::Utc;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

use crate::{
    processor::processor::ProcessorMessage,
    supabase_jwt_middleware::User,
    types::{
        action_types::ActionType,
        task_types::{Stage, Task, TaskConfig, TaskStatus, TriggerSessionStatus},
        workflow_types::DatabaseFlowVersion,
    },
    AppState,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::error;

#[derive(Debug, Deserialize, Serialize)]
pub struct StartTestingWorkflowPayload {
    trigger_session_id: Uuid,
    flow_session_id: Uuid,
}

// #[axum::debug_handler]
pub async fn test_workflow(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    Json(payload): Json<StartTestingWorkflowPayload>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    println!("[TESTING] Handling test workflow request");

    // GET the workflow_version
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .eq("account_id", &account_id)
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            println!("[TESTING] Failed to execute request to get workflow version");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            println!("[TESTING] Failed to read response body");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let workflow_version: DatabaseFlowVersion = match serde_json::from_str(&body) {
        Ok(dbflowversion) => dbflowversion,
        Err(e) => {
            println!("[TESTING] Failed to parse workflow version JSON: {}", e);
            println!("[TESTING] Raw JSON body: {}", body);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse JSON: {}", e),
            )
                .into_response();
        }
    };

    println!("[TESTING] Successfully retrieved workflow version");

    // Find the trigger action
    let trigger_action = match workflow_version
        .flow_definition
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
    {
        Some(action) => action,
        None => {
            println!("[TESTING] No trigger action found in workflow");
            return (
                StatusCode::BAD_REQUEST,
                "No trigger action found in workflow",
            )
                .into_response();
        }
    };

    let task_config = TaskConfig {
        inputs: Some(serde_json::json!(trigger_action.inputs)),
        inputs_schema: Some(trigger_action.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_action.plugin_config.clone()),
        plugin_config_schema: Some(trigger_action.plugin_config_schema.clone()),
    };

    println!("[TESTING] Creating task input");
    let task = match Task::builder()
        .account_id(Uuid::parse_str(&account_id).unwrap())
        .flow_id(Uuid::parse_str(&workflow_id).unwrap())
        .flow_version_id(workflow_version.flow_version_id)
        .action_label(trigger_action.label.clone())
        .trigger_id(trigger_action.action_id.clone())
        .flow_session_id(payload.flow_session_id.clone())
        .action_id(trigger_action.action_id.clone())
        .r#type(ActionType::Trigger)
        .plugin_name(trigger_action.plugin_name.clone())
        .plugin_version(trigger_action.plugin_version.clone())
        .stage(Stage::Testing)
        .config(task_config)
        .result(json!({
            "message": format!("Successfully triggered task"),
            "created_at": Utc::now()
        }))
        .build()
    {
        Ok(task) => task,
        Err(e) => error!("Failed to build task: {}", e),
    };

    println!("[TESTING] Creating processor message");
    // Send message to processor
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version: workflow_version,
        flow_session_id: task.flow_session_id.clone(),
        trigger_session_id: task.trigger_session_id.clone(),
        trigger_task: Some(task.clone()),
    };

    println!("[TESTING] Initializing flow session data");

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TESTING] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[TESTING] Successfully initiated test workflow");
    Json(serde_json::json!({
        "flow_session_id": task.flow_session_id,
        "trigger_session_id": task.trigger_session_id
    }))
    .into_response()
}

// Actions
pub async fn get_test_session_results(
    Path((account_id, workflow_id, workflow_version_id, session_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TESTING] Handling get_test_session_results request");
    println!(
        "[TESTING] Getting results for session {} in workflow {} version {}",
        session_id, workflow_id, workflow_version_id
    );

    let client = &state.anything_client;

    println!("[TESTING] Querying tasks table for session results");
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
        Ok(response) => {
            println!("[TESTING] Successfully queried tasks table");
            response
        }
        Err(e) => {
            println!("[TESTING] Failed to execute request to get tasks: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[TESTING] Successfully read response body");
            body
        }
        Err(e) => {
            println!("[TESTING] Failed to read response body: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let tasks: Vec<Task> = match serde_json::from_str::<Vec<Task>>(&body) {
        Ok(tasks) => {
            println!("[TESTING] Successfully parsed {} tasks", tasks.len());
            tasks
        }
        Err(e) => {
            println!("[TESTING] Failed to parse tasks JSON: {}", e);
            println!("[TESTING] Raw JSON body: {}", body);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse tasks").into_response();
        }
    };

    println!("[TESTING] Checking completion status of tasks");
    //TODO: maybe use trigger status in some future where we can have subflows.
    let all_completed = !tasks.is_empty()
        && tasks.iter().all(|task| {
            matches!(
                task.trigger_session_status,
                TriggerSessionStatus::Completed | TriggerSessionStatus::Failed
            )
        });

    println!("[TESTING] Session completion status: {}", all_completed);
    let result = serde_json::json!({
        "tasks": tasks,
        "complete": all_completed
    });

    println!("[TESTING] Successfully returning test session results");
    Json(result).into_response()
}

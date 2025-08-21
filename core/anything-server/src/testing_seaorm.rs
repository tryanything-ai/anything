use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::custom_auth::User;
use crate::entities::{flow_versions, tasks};
use crate::AppState;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};

#[derive(Debug, Deserialize, Serialize)]
pub struct TestWorkflowRequest {
    pub test_input: Option<Value>,
    pub configuration: Option<Value>,
}

// Test workflow using SeaORM
pub async fn test_workflow(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<TestWorkflowRequest>,
) -> impl IntoResponse {
    println!(
        "Handling test_workflow with SeaORM for workflow: {}, version: {}",
        workflow_id, workflow_version_id
    );

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let workflow_uuid = match Uuid::parse_str(&workflow_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid workflow ID").into_response(),
    };

    let version_uuid = match Uuid::parse_str(&workflow_version_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid version ID").into_response(),
    };

    // Get the workflow version
    let workflow_version = match flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowVersionId.eq(version_uuid))
        .filter(flow_versions::Column::FlowId.eq(workflow_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(version)) => version,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Workflow version not found").into_response();
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Generate a test session ID
    let test_session_id = Uuid::new_v4();

    // TODO: Implement actual workflow testing logic
    // This would typically involve:
    // 1. Creating a test task
    // 2. Running the workflow processor
    // 3. Tracking execution progress
    // 4. Returning results

    let response = json!({
        "session_id": test_session_id,
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "status": "started",
        "message": "Test workflow initiated (SeaORM placeholder implementation)",
        "test_input": payload.test_input,
        "configuration": payload.configuration
    });

    println!("Successfully initiated test workflow");
    Json(response).into_response()
}

// Get test session results using SeaORM
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
    println!(
        "Handling get_test_session_results with SeaORM for session: {}",
        session_id
    );

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let workflow_uuid = match Uuid::parse_str(&workflow_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid workflow ID").into_response(),
    };

    let version_uuid = match Uuid::parse_str(&workflow_version_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid version ID").into_response(),
    };

    let session_uuid = match Uuid::parse_str(&session_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid session ID").into_response(),
    };

    // Get tasks for this test session
    let test_tasks = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .filter(tasks::Column::FlowId.eq(workflow_uuid))
        .filter(tasks::Column::FlowVersionId.eq(version_uuid))
        .filter(tasks::Column::FlowSessionId.eq(session_uuid))
        .order_by_desc(tasks::Column::CreatedAt)
        .all(&*state.db)
        .await
    {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    if test_tasks.is_empty() {
        return Json(json!({
            "session_id": session_id,
            "status": "not_found",
            "message": "No test results found for this session",
            "tasks": []
        })).into_response();
    }

    // Convert tasks to response format
    let task_results: Vec<Value> = test_tasks
        .into_iter()
        .map(|task| json!({
            "task_id": task.task_id,
            "action_label": task.action_label,
            "task_status": task.task_status,
            "result": task.result,
            "error": task.error,
            "debug_result": task.debug_result,
            "created_at": task.created_at,
            "started_at": task.started_at,
            "ended_at": task.ended_at,
            "stage": task.stage
        }))
        .collect();

    // Determine overall session status
    let overall_status = if task_results.iter().any(|task| {
        task.get("task_status")
            .and_then(|s| s.as_str())
            .map_or(false, |s| s == "failed" || s == "error")
    }) {
        "failed"
    } else if task_results.iter().all(|task| {
        task.get("task_status")
            .and_then(|s| s.as_str())
            .map_or(false, |s| s == "completed")
    }) {
        "completed"
    } else {
        "running"
    };

    let response = json!({
        "session_id": session_id,
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "status": overall_status,
        "tasks": task_results,
        "task_count": task_results.len()
    });

    println!("Successfully retrieved test session results with {} tasks", task_results.len());
    Json(response).into_response()
}

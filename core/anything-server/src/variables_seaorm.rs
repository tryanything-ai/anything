use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    bundler::bundle_cached_inputs,
    custom_auth::User,
    entities::{tasks, flow_versions},
    types::{
        task_types::Task,
        workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
    },
    AppState,
};

use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, QuerySelect};

// Get flow version results using SeaORM
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
    println!("[VARIABLES] Handling get_flow_version_results for account: {}, workflow: {}, version: {}, action: {}", 
        account_id, workflow_id, workflow_version_id, action_id);

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

    // Get last task for workflow using SeaORM
    println!("[VARIABLES] Fetching last task for workflow");
    let last_task = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .filter(tasks::Column::FlowId.eq(workflow_uuid))
        .filter(tasks::Column::FlowVersionId.eq(version_uuid))
        .order_by_desc(tasks::Column::CreatedAt)
        .one(&*state.db)
        .await
    {
        Ok(Some(task)) => task,
        Ok(None) => {
            println!("[VARIABLES] No tasks found for workflow");
            return Json(json!({
                "error": "No tasks found for this workflow version"
            })).into_response();
        }
        Err(err) => {
            println!("[VARIABLES] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Extract results from the task
    let results = match &last_task.result {
        Some(result_value) => result_value.clone(),
        None => {
            println!("[VARIABLES] No results found in task");
            return Json(json!({
                "error": "No results found for this task"
            })).into_response();
        }
    };

    println!("[VARIABLES] Successfully retrieved results");
    Json(results).into_response()
}

// Get flow version inputs using SeaORM
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
    println!("[VARIABLES] Handling get_flow_version_inputs for account: {}, workflow: {}, version: {}, action: {}", 
        account_id, workflow_id, workflow_version_id, action_id);

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

    // Get the workflow version using SeaORM
    println!("[VARIABLES] Fetching workflow version");
    let workflow_version = match flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowVersionId.eq(version_uuid))
        .filter(flow_versions::Column::FlowId.eq(workflow_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(version)) => version,
        Ok(None) => {
            println!("[VARIABLES] Workflow version not found");
            return (StatusCode::NOT_FOUND, "Workflow version not found").into_response();
        }
        Err(err) => {
            println!("[VARIABLES] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Parse the workflow definition
    let definition: WorkflowVersionDefinition = match serde_json::from_value(workflow_version.flow_definition.clone()) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("[VARIABLES] Failed to parse workflow definition: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid workflow definition").into_response();
        }
    };

    // Get last task for context
    let last_task = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .filter(tasks::Column::FlowId.eq(workflow_uuid))
        .filter(tasks::Column::FlowVersionId.eq(version_uuid))
        .order_by_desc(tasks::Column::CreatedAt)
        .one(&*state.db)
        .await
    {
        Ok(task_opt) => task_opt,
        Err(err) => {
            println!("[VARIABLES] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Extract context from the task if available
    let context = match last_task {
        Some(task) => task.context.unwrap_or_else(|| json!({})),
        None => json!({})
    };

    // TODO: Update bundle_cached_inputs to use SeaORM instead of Postgrest
    let bundled_inputs = json!({
        "message": "bundle_cached_inputs not yet updated for SeaORM",
        "context": context,
        "definition_actions": definition.actions.len(),
        "status": "placeholder"
    });

    println!("[VARIABLES] Successfully bundled inputs");
    Json(bundled_inputs).into_response()
}

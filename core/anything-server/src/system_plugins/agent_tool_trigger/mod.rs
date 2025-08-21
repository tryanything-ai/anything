use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

mod utils;

use std::time::Duration;

use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Order};

use crate::{processor::processor::ProcessorMessage, types::workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition}};
use crate::{
    types::{
        action_types::ActionType,
        task_types::{Stage, Task, TaskConfig},
    },
    entities::flow_versions,
    AppState, FlowCompletion,
};
use tracing::error;

use tokio::sync::oneshot;
use tokio::time::timeout;

use crate::system_plugins::webhook_trigger::webhook_trigger_utils::validate_required_input_and_response_plugins;

//One Minute
pub const WEBHOOK_TIMEOUT: u64 = 60;

pub async fn run_workflow_as_tool_call_and_respond(
    Path((agent_id, workflow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Json<Value>,
) -> impl IntoResponse {
    println!("[TOOL_CALL_API SEAORM] Handling run workflow and respond");

    println!("[TOOL_CALL_API SEAORM] Call Body: {:?}", body);
    println!("[TOOL_CALL_API SEAORM] Workflow ID: {}: ", workflow_id);

    let workflow_uuid = match Uuid::parse_str(&workflow_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid workflow ID").into_response(),
    };

    // Get flow version from database using SeaORM
    println!("[TOOL_CALL_API SEAORM] Fetching flow version from database");
    let flow_version = match flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowId.eq(workflow_uuid))
        .filter(flow_versions::Column::Published.eq(true))
        .order_by(flow_versions::Column::CreatedAt, Order::Desc)
        .one(&*state.db)
        .await
    {
        Ok(Some(version)) => version,
        Ok(None) => {
            println!("[TOOL_CALL_API SEAORM] No published workflow found");
            return (
                StatusCode::BAD_REQUEST,
                "Unpublished Workflow. To use this endpoint you must publish your workflow.",
            )
                .into_response();
        }
        Err(err) => {
            println!("[TOOL_CALL_API SEAORM] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    // Convert to the expected DatabaseFlowVersion format
    let workflow_definition: WorkflowVersionDefinition = match serde_json::from_value(flow_version.flow_definition) {
        Ok(def) => def,
        Err(err) => {
            println!("[TOOL_CALL_API SEAORM] Failed to parse workflow definition: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid workflow definition",
            )
                .into_response();
        }
    };

    let workflow_version = DatabaseFlowVersion {
        flow_version_id: flow_version.flow_version_id,
        flow_id: flow_version.flow_id,
        flow: None,
        published: flow_version.published,
        account_id: flow_version.account_id,
        flow_definition: workflow_definition.clone(),
    };

    // Get account_id from workflow_version
    let account_id = workflow_version.account_id;

    println!("[TOOL_CALL_API SEAORM] Workflow version: {:?}", workflow_version);
    // Parse the flow definition into a Workflow
    println!("[TOOL_CALL_API SEAORM] Parsing workflow definition");
    
    // Validate the tool has correct input and output nodes
    let (trigger_node, _output_node) = match validate_required_input_and_response_plugins(
        &workflow_version.flow_definition,
        "@anything/agent_tool_call".to_string(),
        "@anything/agent_tool_call_response".to_string(),
        true,
    ) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    println!("[TOOL_CALL_API SEAORM] Trigger node: {:?}", trigger_node);

    let task_config: TaskConfig = TaskConfig {
        inputs: Some(trigger_node.inputs.clone().unwrap()),
        inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_node.plugin_config.clone()),
        plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
    };

    // Parse the tool call request
    let (parsed_and_formatted_body, tool_call_id) = utils::parse_tool_call_request_to_result(body);

    // Create a task to initiate the flow
    println!("[TOOL_CALL_API SEAORM] Creating task for workflow execution");

    let task = match Task::builder()
        .account_id(account_id)
        .flow_id(workflow_uuid)
        .flow_version_id(workflow_version.flow_version_id)
        .action_label(trigger_node.label.clone())
        .trigger_id(trigger_node.action_id.clone())
        .action_id(trigger_node.action_id.clone())
        .r#type(ActionType::Trigger)
        .plugin_name(trigger_node.plugin_name.clone())
        .plugin_version(trigger_node.plugin_version.clone())
        .stage(if workflow_version.published {
            Stage::Production
        } else {
            Stage::Testing
        })
        .result(parsed_and_formatted_body)
        .config(task_config)
        .build()
    {
        Ok(task) => task,
        Err(e) => panic!("Failed to build task: {}", e),
    };

    println!("[TOOL_CALL_API SEAORM] Task to be created: {:?}", task);
    println!("[TOOL_CALL_API SEAORM] Creating processor message");

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    state.flow_completions.insert(
        task.flow_session_id.to_string(),
        FlowCompletion {
            sender: tx,
            needs_response: true,
        },
    );

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: workflow_uuid,
        workflow_version: workflow_version.clone(),
        workflow_definition: workflow_version.flow_definition.clone(),
        flow_session_id: task.flow_session_id,
        trigger_session_id: task.trigger_session_id,
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id),
        existing_tasks: HashMap::new(),
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TOOL_CALL_API SEAORM] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[TOOL_CALL_API SEAORM] Waiting for workflow completion");

    // Wait for the result with a timeout
    match timeout(Duration::from_secs(WEBHOOK_TIMEOUT), rx).await {
        Ok(Ok(flow_result)) => {
            println!(
                "[TOOL_CALL_API SEAORM] Received workflow result: {:?}",
                flow_result
            );
            utils::parse_tool_response_into_api_response(tool_call_id, Some(flow_result), None)
                .into_response()
        }
        Ok(Err(_)) => {
            println!("[TOOL_CALL_API SEAORM] Workflow channel closed unexpectedly");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Workflow execution channel closed unexpectedly",
                    "workflow_session_id": task.flow_session_id
                })),
            )
                .into_response()
        }
        Err(_) => {
            println!("[TOOL_CALL_API SEAORM] Workflow timed out after 60 seconds");
            // Remove the completion channel on timeout
            state
                .flow_completions
                .remove(&task.flow_session_id.to_string());
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error": "Workflow execution timed out",
                    "workflow_session_id": task.flow_session_id
                })),
            )
                .into_response()
        }
    }
}

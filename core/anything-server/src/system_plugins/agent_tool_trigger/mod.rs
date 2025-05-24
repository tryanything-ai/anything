use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

mod utils;

use std::time::Duration;

use dotenv::dotenv;
use serde_json::{json, Value};
use std::env;
use std::sync::Arc;
use uuid::Uuid;

use crate::{processor::processor::ProcessorMessage, types::workflow_types::DatabaseFlowVersion};
use crate::{
    types::{
        action_types::ActionType,
        task_types::{Stage, Task, TaskConfig},
    },
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
    println!("[TOOL_CALL_API] Handling run workflow and respond");

    println!("[TOOL_CALL_API] Call Body: {:?}", body);

    println!("[TOOL_CALL_API] Workflow ID: {}: ", workflow_id);

    //TODO:add tool calls to apent_tool_calls or something that allows us to trace this data
    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    println!("[TOOL_CALL_API] Fetching flow version from database");
    let response = match state
        .anything_client
        .from("flow_versions")
        .eq("flow_id", workflow_id.clone())
        .eq("published", "true")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("[TOOL_CALL_API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let response_body = match response.text().await {
        Ok(body) => {
            println!("[TOOL_CALL_API] Response body: {}", body);
            body
        }
        Err(err) => {
            println!("[TOOL_CALL_API] Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let workflow_version: DatabaseFlowVersion = match serde_json::from_str(&response_body) {
        Ok(version) => version,
        Err(_) => {
            println!("[TOOL_CALL_API] No published workflow found");
            return (
                StatusCode::BAD_REQUEST,
                "Unpublished Workflow. To use this endpoint you must publish your workflow.",
            )
                .into_response();
        }
    };

    // Get account_id from workflow_version
    let account_id = workflow_version.account_id.clone();

    println!("[TOOL_CALL_API] Workflow version: {:?}", workflow_version);
    // Parse the flow definition into a Workflow
    println!("[TOOL_CALL_API] Parsing workflow definition");
    // Validate the tool is has correct input and oupt nodes. Does not gurantee correct inputs ie rigth arguments
    let (trigger_node, _output_node) = match validate_required_input_and_response_plugins(
        &workflow_version.flow_definition,
        "@anything/agent_tool_call".to_string(),
        "@anything/agent_tool_call_response".to_string(),
        true,
    ) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    println!("[TOOL_CALL_API] Trigger node: {:?}", trigger_node);

    let task_config: TaskConfig = TaskConfig {
        inputs: Some(trigger_node.inputs.clone().unwrap()),
        inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_node.plugin_config.clone()),
        plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
    };

    //TODO: take the input style from here https://docs.vapi.ai/server-url/events
    //And convert and simplify it to create the correct "result";
    let (parsed_and_formatted_body, tool_call_id) = utils::parse_tool_call_request_to_result(body);

    // Create a task to initiate the flow
    println!("[TOOL_CALL_API] Creating task for workflow execution");

    let task = match Task::builder()
        .account_id(account_id)
        .flow_id(Uuid::parse_str(&workflow_id).unwrap())
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

    println!("[TOOL_CALL_API] Task to be created: {:?}", task);

    println!("[TOOL_CALL_API] Creating processor message");

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    {
        let mut completions = state.flow_completions.lock().await;
        completions.insert(
            task.flow_session_id.to_string(),
            FlowCompletion {
                sender: tx,
                needs_response: true,
            },
        );
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version: workflow_version,
        flow_session_id: task.flow_session_id.clone(),
        trigger_session_id: task.trigger_session_id.clone(),
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id), // Include task_id for tracing
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TEST WORKFLOW] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[TOOL_CALL_API] Waiting for workflow completion");

    // Wait for the result with a timeout
    match timeout(Duration::from_secs(WEBHOOK_TIMEOUT), rx).await {
        Ok(Ok(flow_result)) => {
            println!(
                "[TOOL_CALL_API] Received workflow result: {:?}",
                flow_result
            );
            //TODO: take this response and turn it into the correct tool_call_response needed for
            utils::parse_tool_response_into_api_response(tool_call_id, Some(flow_result), None)
                .into_response()
        }
        Ok(Err(_)) => {
            println!("[TOOL_CALL_API] Workflow channel closed unexpectedly");
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
            println!("[TOOL_CALL_API] Workflow timed out after 30 seconds");
            // Remove the completion channel on timeout
            state
                .flow_completions
                .lock()
                .await
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

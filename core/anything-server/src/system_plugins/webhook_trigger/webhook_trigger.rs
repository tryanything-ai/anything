use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    Json,
};

use std::time::Duration;

use dotenv::dotenv;
use serde_json::{json, Value};
use std::{collections::HashMap, env, sync::Arc};
use uuid::Uuid;

use crate::{
    bundler::bundle_context_from_parts,
    types::{
        action_types::ActionType,
        task_types::{Stage, Task, TaskConfig},
    },
    AppState, FlowCompletion,
};

use crate::{processor::processor::ProcessorMessage, types::workflow_types::DatabaseFlowVersion};

use tokio::sync::oneshot;
use tokio::time::timeout;

use tracing::error;

use super::webhook_trigger_utils::{
    convert_request_to_payload, parse_response_action_response_into_api_response,
    validate_request_method, validate_required_input_and_response_plugins, validate_security_model,
};

//One Minute
pub const WEBHOOK_TIMEOUT: u64 = 60;

pub async fn run_workflow_and_respond(
    method: Method,
    Path(workflow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    query: Option<Query<HashMap<String, String>>>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow and respond");
    // println!("[WEBHOOK API] Payload: {:?}", payload);
    println!("[WEBHOOK API] Workflow ID: {}: ", workflow_id);

    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    println!("[WEBHOOK API] Fetching flow version from database");
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
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let response_body = match response.text().await {
        Ok(body) => {
            println!("[WEBHOOK API] Response body: {}", body);
            body
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to read response body: {:?}", err);
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
            println!("[WEBHOOK API] No published workflow found");
            return (
                StatusCode::BAD_REQUEST,
                "Unpublished Workflow. To use this endpoint you must publish your workflow.",
            )
                .into_response();
        }
    };

    // Get account_id from workflow_version
    let account_id = workflow_version.account_id.clone();

    // Parse the flow definition into a Workflow
    println!("[WEBHOOK API] Parsing workflow definition");
    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_required_input_and_response_plugins(
        &workflow_version.flow_definition,
        "@anything/webhook".to_string(),
        "@anything/webhook_response".to_string(),
        true,
    ) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4();

    let task_config: TaskConfig = TaskConfig {
        inputs: Some(trigger_node.inputs.clone().unwrap()),
        inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_node.plugin_config.clone()),
        plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id.to_string(),
        Some(&trigger_node.inputs.clone().unwrap()),
        Some(&trigger_node.inputs_schema.clone().unwrap()),
        Some(&trigger_node.plugin_config.clone()),
        Some(&trigger_node.plugin_config_schema.clone()),
        false,
    )
    .await
    {
        Ok(context) => context,
        Err(e) => {
            println!("[WEBHOOK API] Failed to bundle context: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to bundle trigger context",
            )
                .into_response();
        }
    };

    println!("[WEBHOOK API] Bundled context: {:?}", rendered_inputs);

    //Validate security model
    if let Some(response) = validate_security_model(&rendered_inputs, &headers, state.clone()).await
    {
        return response.into_response();
    }

    // Validate request method
    if let Some(response) = validate_request_method(&rendered_inputs, &method.to_string()) {
        return response.into_response();
    }

    let processed_payload = convert_request_to_payload(method.clone(), query, body);

    // Create a task to initiate the flow
    println!("[WEBHOOK API] Creating task for workflow execution");

    let task = match Task::builder()
    .account_id(account_id)
        .flow_id(Uuid::parse_str(&workflow_id).unwrap())
    .flow_version_id(workflow_version.flow_version_id)
    .action_label(trigger_node.label.clone())
    .trigger_id(trigger_node.action_id.clone())
    .flow_session_id(flow_session_id)
    .action_id(trigger_node.action_id.clone())
    .r#type(ActionType::Trigger)
    .plugin_name(trigger_node.plugin_name.clone())
    .plugin_version(trigger_node.plugin_version.clone())
    .stage(if workflow_version.published {
            Stage::Production
        } else {
            Stage::Testing
        })
        .config(task_config)
        .result(json!({
                    "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
                    "body": processed_payload.clone(),
                    "method": method.to_string(),
                }))
        .build() {
            Ok(task) => task,
            Err(e) => panic!("Failed to build task: {}", e),
        };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    println!("[WEBHOOK API] Creating processor message");

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    {
        let mut completions = state.flow_completions.lock().await;
        completions.insert(
            flow_session_id.to_string(),
            FlowCompletion {
                sender: tx,
                needs_response: true,
            },
        );
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version: workflow_version.clone(),
        workflow_definition: workflow_version.flow_definition.clone(),
        flow_session_id: flow_session_id,
        trigger_session_id: task.trigger_session_id,
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id),    // Include task_id for tracing
        existing_tasks: HashMap::new(), // No existing tasks for new workflows
        workflow_graph: crate::processor::utils::create_workflow_graph(
            &workflow_version.flow_definition,
        ),
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TEST WORKFLOW] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[WEBHOOK API] Waiting for workflow completion");

    // Wait for the result with a timeout
    match timeout(Duration::from_secs(WEBHOOK_TIMEOUT), rx).await {
        Ok(Ok(result)) => {
            println!("[WEBHOOK API] Received workflow result");
            parse_response_action_response_into_api_response(result).into_response()
        }
        Ok(Err(_)) => {
            println!("[WEBHOOK API] Workflow channel closed unexpectedly");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Workflow execution channel closed unexpectedly",
                    "workflow_session_id": flow_session_id
                })),
            )
                .into_response()
        }
        Err(_) => {
            println!("[WEBHOOK API] Workflow timed out after 30 seconds");
            // Remove the completion channel on timeout
            state
                .flow_completions
                .lock()
                .await
                .remove(&flow_session_id.to_string());
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error": "Workflow execution timed out",
                    "workflow_session_id": flow_session_id
                })),
            )
                .into_response()
        }
    }
}

pub async fn run_workflow_version_and_respond(
    method: Method,
    Path((workflow_id, workflow_version_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    query: Option<Query<HashMap<String, String>>>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow and respond");
    println!("[WEBHOOK API] Payload: {:?}", body);

    println!("[WEBHOOK API] Workflow ID: {}: ", workflow_id);

    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    println!("[WEBHOOK API] Fetching flow version from database");
    let response = match state
        .anything_client
        .from("flow_versions")
        .eq("flow_id", workflow_id.clone())
        .eq("flow_version_id", workflow_version_id.clone())
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let response_body = match response.text().await {
        Ok(body) => {
            println!("[WEBHOOK API] Response body: {}", body);
            body
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to read response body: {:?}", err);
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
            println!("[WEBHOOK API] No published workflow found");
            return (
                StatusCode::BAD_REQUEST,
                "Unpublished Workflow. To use this endpoint you must publish your workflow.",
            )
                .into_response();
        }
    };

    // Get account_id from workflow_version
    let account_id = workflow_version.account_id.clone();

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_required_input_and_response_plugins(
        &workflow_version.flow_definition,
        "@anything/webhook".to_string(),
        "@anything/webhook_response".to_string(),
        true,
    ) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4();

    let task_config: TaskConfig = TaskConfig {
        inputs: Some(serde_json::to_value(&trigger_node.inputs).unwrap()),
        inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_node.plugin_config.clone()),
        plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id.to_string(),
        Some(&trigger_node.inputs.clone().unwrap()),
        Some(&trigger_node.inputs_schema.clone().unwrap()),
        Some(&trigger_node.plugin_config.clone()),
        Some(&trigger_node.plugin_config_schema.clone()),
        false,
    )
    .await
    {
        Ok(context) => context,
        Err(e) => {
            println!("[WEBHOOK API] Failed to bundle context: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to bundle trigger context",
            )
                .into_response();
        }
    };

    println!("[WEBHOOK API] Bundled context: {:?}", rendered_inputs);

    //Validate security model
    if let Some(response) = validate_security_model(&rendered_inputs, &headers, state.clone()).await
    {
        return response.into_response();
    }

    // Validate request method
    if let Some(response) = validate_request_method(&rendered_inputs, &method.to_string()) {
        return response.into_response();
    }

    let processed_payload = convert_request_to_payload(method.clone(), query, body);

    // Create a task to initiate the flow
    println!("[WEBHOOK API] Creating task for workflow execution");

    let task = match Task::builder()
    .account_id(account_id)
    .flow_id(Uuid::parse_str(&workflow_id).unwrap())
    .flow_version_id(workflow_version.flow_version_id)
    .action_label(trigger_node.label.clone())
    .trigger_id(trigger_node.action_id.clone())
    .flow_session_id(flow_session_id)
    .action_id(trigger_node.action_id.clone())
    .r#type(ActionType::Trigger)
    .plugin_name(trigger_node.plugin_name.clone())
    .plugin_version(trigger_node.plugin_version.clone())
    .stage(if workflow_version.published {
        Stage::Production
    } else {
        Stage::Testing
    })
    .config(task_config)
    .result(json!({
                "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
                "body": processed_payload.clone(),
                "method": method.to_string(),
            }))
    .build() {
        Ok(task) => task,
        Err(e) => panic!("Failed to build task: {}", e),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    {
        let mut completions = state.flow_completions.lock().await;
        completions.insert(
            flow_session_id.to_string(),
            FlowCompletion {
                sender: tx,
                needs_response: true,
            },
        );
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version: workflow_version.clone(),
        workflow_definition: workflow_version.flow_definition.clone(),
        flow_session_id: flow_session_id,
        trigger_session_id: task.trigger_session_id,
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id),    // Include task_id for tracing
        existing_tasks: HashMap::new(), // No existing tasks for new workflows
        workflow_graph: crate::processor::utils::create_workflow_graph(
            &workflow_version.flow_definition,
        ),
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TEST WORKFLOW] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[WEBHOOK API] Waiting for workflow completion");

    // Wait for the result with a timeout
    match timeout(Duration::from_secs(WEBHOOK_TIMEOUT), rx).await {
        Ok(Ok(result)) => {
            println!("[WEBHOOK API] Received workflow result");
            parse_response_action_response_into_api_response(result).into_response()
        }
        Ok(Err(_)) => {
            println!("[WEBHOOK API] Workflow channel closed unexpectedly");
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
            println!("[WEBHOOK API] Workflow timed out after 30 seconds");
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

pub async fn run_workflow(
    method: Method,
    Path(workflow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    query: Option<Query<HashMap<String, String>>>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow and respond");
    println!("[WEBHOOK API] Payload: {:?}", body);

    println!("[WEBHOOK API] Workflow ID: {}: ", workflow_id);

    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    println!("[WEBHOOK API] Fetching flow version from database");
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
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let response_body = match response.text().await {
        Ok(body) => {
            println!("[WEBHOOK API] Response body: {}", body);
            body
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to read response body: {:?}", err);
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
            println!("[WEBHOOK API] No published workflow found");
            return (
                StatusCode::BAD_REQUEST,
                "Unpublished Workflow. To use this endpoint you must publish your workflow.",
            )
                .into_response();
        }
    };

    // Get account_id from workflow_version
    let account_id = workflow_version.account_id.clone();

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_required_input_and_response_plugins(
        &workflow_version.flow_definition,
        "@anything/webhook".to_string(),
        "@anything/webhook_response".to_string(),
        false,
    ) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4();

    let task_config: TaskConfig = TaskConfig {
        inputs: Some(serde_json::to_value(&trigger_node.inputs).unwrap()),
        inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_node.plugin_config.clone()),
        plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id.to_string(),
        Some(&trigger_node.inputs.clone().unwrap()),
        Some(&trigger_node.inputs_schema.clone().unwrap()),
        Some(&trigger_node.plugin_config.clone()),
        Some(&trigger_node.plugin_config_schema.clone()),
        false,
    )
    .await
    {
        Ok(context) => context,
        Err(e) => {
            println!("[WEBHOOK API] Failed to bundle context: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to bundle trigger context",
            )
                .into_response();
        }
    };

    println!("[WEBHOOK API] Bundled context: {:?}", rendered_inputs);

    //Validate security model
    if let Some(response) = validate_security_model(&rendered_inputs, &headers, state.clone()).await
    {
        return response.into_response();
    }

    // Validate request method
    if let Some(response) = validate_request_method(&rendered_inputs, &method.to_string()) {
        return response.into_response();
    }

    let processed_payload = convert_request_to_payload(method.clone(), query, body);

    // Create a task to initiate the flow
    println!("[WEBHOOK API] Creating task for workflow execution");
    let task = match Task::builder()
    .account_id(account_id)
    .flow_id(Uuid::parse_str(&workflow_id).unwrap())
    .flow_version_id(workflow_version.flow_version_id)
    .action_label(trigger_node.label.clone())
    .trigger_id(trigger_node.action_id.clone())
    .flow_session_id(flow_session_id)
    .action_id(trigger_node.action_id.clone())
    .r#type(ActionType::Trigger)
    .plugin_name(trigger_node.plugin_name.clone())
    .plugin_version(trigger_node.plugin_version.clone())
    .stage(if workflow_version.published {
        Stage::Production
    } else {
        Stage::Testing
    })
    .config(task_config)
    .result(json!({
                "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
                "body": processed_payload.clone(),
                "method": method.to_string(),
            }))
    .build() {
        Ok(task) => task,
        Err(e) => panic!("Failed to build task: {}", e),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version: workflow_version.clone(),
        workflow_definition: workflow_version.flow_definition.clone(),
        flow_session_id: flow_session_id,
        trigger_session_id: task.trigger_session_id,
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id),    // Include task_id for tracing
        existing_tasks: HashMap::new(), // No existing tasks for new workflows
        workflow_graph: crate::processor::utils::create_workflow_graph(
            &workflow_version.flow_definition,
        ),
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TEST WORKFLOW] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[WEBHOOK API] Task created successfully");
    Json(serde_json::json!({
        "success": true,
        "message": "Workflow started!",
        "workflow_session_id": task.flow_session_id,
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version.flow_version_id
    }))
    .into_response()
}

pub async fn run_workflow_version(
    method: Method,
    Path((workflow_id, workflow_version_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    query: Option<Query<HashMap<String, String>>>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow and respond");
    println!("[WEBHOOK API] Payload: {:?}", body);

    println!("[WEBHOOK API] Workflow ID: {}: ", workflow_id);

    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    println!("[WEBHOOK API] Fetching flow version from database");
    let response = match state
        .anything_client
        .from("flow_versions")
        .eq("flow_id", workflow_id.clone())
        .eq("flow_version_id", workflow_version_id.clone())
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let response_body = match response.text().await {
        Ok(body) => {
            println!("[WEBHOOK API] Response body: {}", body);
            body
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to read response body: {:?}", err);
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
            println!("[WEBHOOK API] No published workflow found");
            return (
                StatusCode::BAD_REQUEST,
                "Unpublished Workflow. To use this endpoint you must publish your workflow.",
            )
                .into_response();
        }
    };

    // Get account_id from workflow_version
    let account_id = workflow_version.account_id.clone();

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_required_input_and_response_plugins(
        &workflow_version.flow_definition,
        "@anything/webhook".to_string(),
        "@anything/webhook_response".to_string(),
        false,
    ) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4();

    let task_config: TaskConfig = TaskConfig {
        inputs: Some(serde_json::to_value(&trigger_node.inputs).unwrap()),
        inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_node.plugin_config.clone()),
        plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id.to_string(),
        Some(&trigger_node.inputs.clone().unwrap()),
        Some(&trigger_node.inputs_schema.clone().unwrap()),
        Some(&trigger_node.plugin_config.clone()),
        Some(&trigger_node.plugin_config_schema.clone()),
        false,
    )
    .await
    {
        Ok(context) => context,
        Err(e) => {
            println!("[WEBHOOK API] Failed to bundle context: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to bundle trigger context",
            )
                .into_response();
        }
    };

    println!("[WEBHOOK API] Bundled context: {:?}", rendered_inputs);

    //Validate security model
    if let Some(response) = validate_security_model(&rendered_inputs, &headers, state.clone()).await
    {
        return response.into_response();
    }

    // Validate request method
    if let Some(response) = validate_request_method(&rendered_inputs, &method.to_string()) {
        return response.into_response();
    }

    let processed_payload = convert_request_to_payload(method.clone(), query, body);

    // Create a task to initiate the flow
    println!("[WEBHOOK API] Creating task for workflow execution");
    let task = match Task::builder()
    .account_id(account_id)
    .flow_id(Uuid::parse_str(&workflow_id).unwrap())
    .flow_version_id(workflow_version.flow_version_id)
    .action_label(trigger_node.label.clone())
    .trigger_id(trigger_node.action_id.clone())
    .flow_session_id(flow_session_id)
    .action_id(trigger_node.action_id.clone())
    .r#type(ActionType::Trigger)
    .plugin_name(trigger_node.plugin_name.clone())
    .plugin_version(trigger_node.plugin_version.clone())
    .stage(if workflow_version.published {
        Stage::Production
    } else {
        Stage::Testing
    })
    .config(task_config)
    .result(json!({
                "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
                "body": processed_payload.clone(),
                "method": method.to_string(),
            }))
    .build() {
        Ok(task) => task,
        Err(e) => panic!("Failed to build task: {}", e),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version: workflow_version.clone(),
        workflow_definition: workflow_version.flow_definition.clone(),
        flow_session_id: flow_session_id,
        trigger_session_id: task.trigger_session_id,
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id),    // Include task_id for tracing
        existing_tasks: HashMap::new(), // No existing tasks for new workflows
        workflow_graph: crate::processor::utils::create_workflow_graph(
            &workflow_version.flow_definition,
        ),
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TEST WORKFLOW] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[WEBHOOK API] Task created successfully");
    Json(serde_json::json!({
        "success": true,
        "message": "Workflow started!",
        "workflow_session_id": flow_session_id.to_string(),
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version.flow_version_id
    }))
    .into_response()
}

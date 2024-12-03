use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;

use std::time::Duration;

use dotenv::dotenv;
use serde_json::{json, Value};
use std::{collections::HashMap, env, sync::Arc};
use uuid::Uuid;

use crate::{
    bundler::bundle_context_from_parts,
    types::{
        action_types::ActionType,
        task_types::{
            CreateTaskInput, FlowSessionStatus, Stage, TaskConfig, TaskStatus, TriggerSessionStatus,
        },
    },
    AppState, FlowCompletion,
};

use crate::{
    processor::{flow_session_cache::FlowSessionData, processor::ProcessorMessage},
    types::workflow_types::DatabaseFlowVersion,
};

use tokio::sync::oneshot;
use tokio::time::timeout;

use super::webhook_trigger_utils::{
    convert_request_to_payload, parse_response_action_response_into_api_response,
    validate_request_method, validate_security_model, validate_webhook_input_and_response,
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
    let (trigger_node, _output_node) =
        match validate_webhook_input_and_response(&workflow_version.flow_definition, true) {
            Ok((trigger, output)) => (trigger, output),
            Err(response) => return response.into_response(),
        };

    let flow_session_id = Uuid::new_v4();

    let task_config: TaskConfig = TaskConfig {
        variables: Some(trigger_node.variables.clone()),
        variables_schema: Some(trigger_node.variables_schema.clone()),
        input: Some(trigger_node.input.clone()),
        input_schema: Some(trigger_node.input_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id.to_string(),
        Some(&trigger_node.variables.clone()),
        Some(&trigger_node.variables_schema.clone()),
        Some(&trigger_node.input.clone()),
        Some(&trigger_node.input_schema.clone()),
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
    let task = CreateTaskInput {
        account_id: account_id.to_string(),
        processing_order: 0,
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: flow_session_id.to_string(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: task_config,
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
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

    //Set the flow data in the cache of the processor so we don't do it again
    let flow_session_data = FlowSessionData {
        workflow: Some(workflow_version.clone()),
        tasks: HashMap::new(),
        flow_session_id: flow_session_id,
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version_id: Some(workflow_version.flow_version_id),
    };

    println!("[TEST WORKFLOW] Setting flow session data in cache");
    // Set the flow session data in cache
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(&flow_session_id, flow_session_data);
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        version_id: Some(workflow_version.flow_version_id),
        flow_session_id: flow_session_id,
        trigger_task: Some(task),
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
    let (trigger_node, _output_node) =
        match validate_webhook_input_and_response(&workflow_version.flow_definition, true) {
            Ok((trigger, output)) => (trigger, output),
            Err(response) => return response.into_response(),
        };

    let flow_session_id = Uuid::new_v4().to_string();

    let task_config: TaskConfig = TaskConfig {
        variables: Some(serde_json::to_value(&trigger_node.variables).unwrap()),
        variables_schema: Some(trigger_node.variables_schema.clone()),
        input: Some(trigger_node.input.clone()),
        input_schema: Some(trigger_node.input_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id,
        Some(&trigger_node.variables.clone()),
        Some(&trigger_node.variables_schema.clone()),
        Some(&trigger_node.input.clone()),
        Some(&trigger_node.input_schema.clone()),
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
    let task = CreateTaskInput {
        account_id: account_id.to_string(),
        processing_order: 0,
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: flow_session_id.clone(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: task_config,

        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    {
        let mut completions = state.flow_completions.lock().await;
        completions.insert(
            flow_session_id.clone(),
            FlowCompletion {
                sender: tx,
                needs_response: true,
            },
        );
    }

    //Set the flow data in the cache of the processor so we don't do it again
    let flow_session_data = FlowSessionData {
        workflow: Some(workflow_version.clone()),
        tasks: HashMap::new(),
        flow_session_id: Uuid::parse_str(&flow_session_id).unwrap(),
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version_id: Some(Uuid::parse_str(&workflow_version_id).unwrap()),
    };

    println!("[TEST WORKFLOW] Setting flow session data in cache");
    // Set the flow session data in cache
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(
            &Uuid::parse_str(&flow_session_id).unwrap(),
            flow_session_data,
        );
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        version_id: Some(Uuid::parse_str(&workflow_version_id).unwrap()),
        flow_session_id: Uuid::parse_str(&flow_session_id).unwrap(),
        trigger_task: Some(task.clone()),
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
                .remove(&task.flow_session_id);
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
    let (trigger_node, _output_node) =
        match validate_webhook_input_and_response(&workflow_version.flow_definition, false) {
            Ok((trigger, output)) => (trigger, output),
            Err(response) => return response.into_response(),
        };

    let flow_session_id = Uuid::new_v4().to_string();

    let task_config: TaskConfig = TaskConfig {
        variables: Some(serde_json::to_value(&trigger_node.variables).unwrap()),
        variables_schema: Some(trigger_node.variables_schema.clone()),
        input: Some(trigger_node.input.clone()),
        input_schema: Some(trigger_node.input_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id,
        Some(&trigger_node.variables.clone()),
        Some(&trigger_node.variables_schema.clone()),
        Some(&trigger_node.input.clone()),
        Some(&trigger_node.input_schema.clone()),
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
    let task = CreateTaskInput {
        account_id: account_id.to_string(),
        processing_order: 0,
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: flow_session_id.clone(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: task_config,
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    //Set the flow data in the cache of the processor so we don't do it again
    let flow_session_data = FlowSessionData {
        workflow: Some(workflow_version.clone()),
        tasks: HashMap::new(),
        flow_session_id: Uuid::parse_str(&flow_session_id).unwrap(),
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version_id: Some(workflow_version.flow_version_id),
    };

    println!("[TEST WORKFLOW] Setting flow session data in cache");
    // Set the flow session data in cache
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(
            &Uuid::parse_str(&flow_session_id).unwrap(),
            flow_session_data,
        );
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        version_id: Some(workflow_version.flow_version_id),
        flow_session_id: Uuid::parse_str(&flow_session_id).unwrap(),
        trigger_task: Some(task.clone()),
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
    let (trigger_node, _output_node) =
        match validate_webhook_input_and_response(&workflow_version.flow_definition, false) {
            Ok((trigger, output)) => (trigger, output),
            Err(response) => return response.into_response(),
        };

    let flow_session_id = Uuid::new_v4();

    let task_config: TaskConfig = TaskConfig {
        variables: Some(serde_json::to_value(&trigger_node.variables).unwrap()),
        variables_schema: Some(trigger_node.variables_schema.clone()),
        input: Some(trigger_node.input.clone()),
        input_schema: Some(trigger_node.input_schema.clone()),
    };

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context_from_parts(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id.to_string(),
        Some(&trigger_node.variables.clone()),
        Some(&trigger_node.variables_schema.clone()),
        Some(&trigger_node.input.clone()),
        Some(&trigger_node.input_schema.clone()),
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
    let task = CreateTaskInput {
        account_id: account_id.to_string(),
        processing_order: 0,
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: flow_session_id.to_string(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: task_config,
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    //Set the flow data in the cache of the processor so we don't do it again
    let flow_session_data = FlowSessionData {
        workflow: Some(workflow_version.clone()),
        tasks: HashMap::new(),
        flow_session_id: flow_session_id,
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version_id: Some(workflow_version.flow_version_id),
    };

    println!("[TEST WORKFLOW] Setting flow session data in cache");
    // Set the flow session data in cache
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(&flow_session_id, flow_session_data);
    }

    // Send message to processor to start the workflow
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        version_id: Some(workflow_version.flow_version_id),
        flow_session_id: flow_session_id,
        trigger_task: Some(task),
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

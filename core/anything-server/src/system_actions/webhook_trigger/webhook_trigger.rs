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

use crate::workflow_types::{CreateTaskInput, FlowVersion, WorkflowVersionDefinition};
use crate::AppState;
use crate::{
    bundler::bundle_context,
    task_types::{FlowSessionStatus, Stage, TaskStatus, TriggerSessionStatus},
};

use crate::{task_types::ActionType, FlowCompletion};

use tokio::sync::oneshot;
use tokio::time::timeout;

use super::webhook_trigger_utils::{
    convert_request_to_payload, parse_response_action_response_into_api_response,
    validate_request_method, validate_security_model, validate_webhook_input_and_response,
};

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

    let workflow_version: FlowVersion = match serde_json::from_str(&response_body) {
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
    let workflow: WorkflowVersionDefinition =
        match serde_json::from_value(workflow_version.flow_definition) {
            Ok(workflow) => workflow,
            Err(err) => {
                println!(
                    "[WEBHOOK API] Failed to parse workflow definition: {:?}",
                    err
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse workflow definition",
                )
                    .into_response();
            }
        };

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_webhook_input_and_response(&workflow, true) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4().to_string();

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id,
        Some(&serde_json::to_value(&trigger_node.variables).unwrap()),
        Some(&serde_json::to_value(&trigger_node.input).unwrap()),
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
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id,
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: json!({}),
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    let _create_task_response = match state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(serde_json::to_string(&task).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[WEBHOOK API] Response from task creation: {:?}", response);
            response
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    // If we need to wait for a response
    //    if needs_response {
    println!("[WEBHOOK API] Waiting for workflow completion");

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    state.flow_completions.lock().await.insert(
        task.flow_session_id.clone(),
        FlowCompletion {
            sender: tx,
            needs_response: true,
        },
    );

    // Send signal to task engine to process the new task
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("[WEBHOOK API] Failed to send task signal: {:?}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to initiate workflow"})),
        )
            .into_response();
    }

    // Wait for the result with a timeout
    match timeout(Duration::from_secs(30), rx).await {
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
                    "workflow_session_id": task.flow_session_id,
                    "message": "You can query the workflow status using the flow_session_id"
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

    let workflow_version: FlowVersion = match serde_json::from_str(&response_body) {
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
    let workflow: WorkflowVersionDefinition =
        match serde_json::from_value(workflow_version.flow_definition) {
            Ok(workflow) => workflow,
            Err(err) => {
                println!(
                    "[WEBHOOK API] Failed to parse workflow definition: {:?}",
                    err
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse workflow definition",
                )
                    .into_response();
            }
        };

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_webhook_input_and_response(&workflow, true) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4().to_string();

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id,
        Some(&serde_json::to_value(&trigger_node.variables).unwrap()),
        Some(&serde_json::to_value(&trigger_node.input).unwrap()),
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
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id,
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: json!({}),
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    let _create_task_response = match state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(serde_json::to_string(&task).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[WEBHOOK API] Response from task creation: {:?}", response);
            response
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    // If we need to wait for a response
    //    if needs_response {
    println!("[WEBHOOK API] Waiting for workflow completion");

    // Create a channel for receiving the completion result
    let (tx, rx) = oneshot::channel();

    // Store the sender in the state
    state.flow_completions.lock().await.insert(
        task.flow_session_id.clone(),
        FlowCompletion {
            sender: tx,
            needs_response: true,
        },
    );

    // Send signal to task engine to process the new task
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("[WEBHOOK API] Failed to send task signal: {:?}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to initiate workflow"})),
        )
            .into_response();
    }

    // Wait for the result with a timeout
    match timeout(Duration::from_secs(30), rx).await {
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
                    "workflow_session_id": task.flow_session_id,
                    "message": "You can query the workflow status using the flow_session_id"
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

    let workflow_version: FlowVersion = match serde_json::from_str(&response_body) {
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
    let workflow: WorkflowVersionDefinition =
        match serde_json::from_value(workflow_version.flow_definition) {
            Ok(workflow) => workflow,
            Err(err) => {
                println!(
                    "[WEBHOOK API] Failed to parse workflow definition: {:?}",
                    err
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse workflow definition",
                )
                    .into_response();
            }
        };

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_webhook_input_and_response(&workflow, false) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4().to_string();

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id,
        Some(&serde_json::to_value(&trigger_node.variables).unwrap()),
        Some(&serde_json::to_value(&trigger_node.input).unwrap()),
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
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id,
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: json!({}),
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    let _create_task_response = match state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(serde_json::to_string(&task).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[WEBHOOK API] Response from task creation: {:?}", response);
            response
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    // Send signal to task engine to process the new task
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("[WEBHOOK API] Failed to send task signal: {:?}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to initiate workflow"})),
        )
            .into_response();
    }

    println!("[WEBHOOK API] Task created successfully");
    Json(serde_json::json!({
        "success": true,
        "message": "Webhook was successfull",
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

    let workflow_version: FlowVersion = match serde_json::from_str(&response_body) {
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
    let workflow: WorkflowVersionDefinition =
        match serde_json::from_value(workflow_version.flow_definition) {
            Ok(workflow) => workflow,
            Err(err) => {
                println!(
                    "[WEBHOOK API] Failed to parse workflow definition: {:?}",
                    err
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse workflow definition",
                )
                    .into_response();
            }
        };

    // Validate the webhook trigger node and outputs
    let (trigger_node, _output_node) = match validate_webhook_input_and_response(&workflow, false) {
        Ok((trigger, output)) => (trigger, output),
        Err(response) => return response.into_response(),
    };

    let flow_session_id = Uuid::new_v4().to_string();

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context(
        state.clone(),
        &state.anything_client,
        &account_id.to_string(),
        &flow_session_id,
        Some(&serde_json::to_value(&trigger_node.variables).unwrap()),
        Some(&serde_json::to_value(&trigger_node.input).unwrap()),
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
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version.flow_version_id.to_string(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id,
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        action_id: trigger_node.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: json!({}),
        result: Some(json!({
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": processed_payload.clone(),
            "method": method.to_string(),
        })),
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    let _create_task_response = match state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(serde_json::to_string(&task).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[WEBHOOK API] Response from task creation: {:?}", response);
            response
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    // Send signal to task engine to process the new task
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("[WEBHOOK API] Failed to send task signal: {:?}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to initiate workflow"})),
        )
            .into_response();
    }

    println!("[WEBHOOK API] Task created successfully");
    Json(serde_json::json!({
        "success": true,
        "message": "Webhook was successfull",
        "workflow_session_id": task.flow_session_id,
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version.flow_version_id
    }))
    .into_response()
}

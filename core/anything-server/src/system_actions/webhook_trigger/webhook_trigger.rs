use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use std::time::Duration;

use dotenv::dotenv;
use serde_json::{json, Value};
use std::{collections::HashMap, env, sync::Arc};
use uuid::Uuid;

use crate::{
    bundler::bundle_context,
    task_types::{FlowSessionStatus, Stage, TaskStatus, TriggerSessionStatus},
};
use crate::workflow_types::{CreateTaskInput, FlowVersion, Workflow};
use crate::AppState;

use crate::{task_types::ActionType, FlowCompletion};

use tokio::sync::oneshot;
use tokio::time::timeout;

use super::webhook_trigger_utils::{validate_webhook_inputs_and_outputs, validate_security_model};

pub async fn run_workflow_and_respond(
    Path(workflow_id): Path<String>, // Changed to tuple extraction
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow and respond");
    println!("[WEBHOOK API] Payload: {:?}", payload);

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

    let body = match response.text().await {
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

    let workflow_version: FlowVersion = match serde_json::from_str(&body) {
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
    let workflow: Workflow = match serde_json::from_value(workflow_version.flow_definition) {
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
    let (trigger_node, _output_node) =
        match validate_webhook_inputs_and_outputs(&workflow, true).await {
            Ok((trigger, output)) => (trigger, output),
            Err(response) => return response.into_response(),
        };

    let flow_session_id = Uuid::new_v4().to_string();

    // Bundle the context for the trigger node
    println!("[WEBHOOK API] Bundling context for trigger node");
    let rendered_inputs = match bundle_context(
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
    validate_security_model(&rendered_inputs, &headers, state.clone()).await;

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
            "body": payload.clone(),
        })),
        test_config: None,
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
            (StatusCode::OK, Json(result)).into_response()
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
    Path(workflow_id): Path<String>, // Changed to tuple extraction
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow");
    println!("[WEBHOOK API] Payload: {:?}", payload);

    println!("[WEBHOOK API] Workflow ID: {}: ", workflow_id,);

    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get account_id for the workflow
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

    let body = match response.text().await {
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

    let workflow_version: FlowVersion = match serde_json::from_str(&body) {
        Ok(version) => version,
        Err(err) => {
            println!("[WEBHOOK API] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    let account_id = workflow_version.account_id.clone();
    println!("[WEBHOOK API] Account ID from flow version: {}", account_id);
    // Only proceed if we have an account_id
    if account_id == Uuid::nil() {
        println!("[WEBHOOK API] Account ID not found");
        return (StatusCode::BAD_REQUEST, "Account ID not found").into_response();
    }

    // Parse the flow definition into a Workflow
    println!("[WEBHOOK API] Parsing workflow definition");
    let workflow: Workflow = match serde_json::from_value(workflow_version.flow_definition) {
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

    // Find the trigger action in the workflow
    println!("[WEBHOOK API] Looking for trigger node in workflow");
    let trigger_node = match workflow
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
    {
        Some(trigger) => trigger,
        None => {
            println!("[WEBHOOK API] No trigger found in workflow");
            return (StatusCode::BAD_REQUEST, "No trigger found in workflow").into_response();
        }
    };

    // Check if trigger node has plugin_id of "webhook"
    if trigger_node.plugin_id != "webhook" {
        println!(
            "[WEBHOOK API] Invalid trigger type: {}",
            trigger_node.plugin_id
        );
        return (
            StatusCode::BAD_REQUEST,
            "Workflow trigger must be an webhook trigger to receive webhook",
        )
            .into_response();
    }

    //We need to use the action definition to generate the config
    //This has to take the incoming body and headers as an argument and parse them into the variables

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
        flow_session_id: Uuid::new_v4().to_string(),
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
            "body": payload.clone(),
        })),
        test_config: None,
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    let create_task_response = match state
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
    let body = match create_task_response.text().await {
        Ok(body) => {
            println!("[WEBHOOK API] Response body: {:?}", body);
            body
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to get response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get response body",
            )
                .into_response();
        }
    };

    let task_response: Value = match serde_json::from_str(&body) {
        Ok(json) => {
            println!("[WEBHOOK API] Parsed JSON: {:?}", json);
            json
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to parse response JSON: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse response",
            )
                .into_response();
        }
    };

    println!("[WEBHOOK API] Parsed task response: {:?}", task_response);

    // Send signal to task engine to process the new task for this webhook
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
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
    Path((workflow_id, workflow_version_id)): Path<(String, String)>, // Changed to tuple extraction
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow version");
    println!("[WEBHOOK API] Payload: {:?}", payload);

    println!(
        "[WEBHOOK API] Workflow ID: {}, Version ID: {}",
        workflow_id, workflow_version_id
    );

    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get account_id for the workflow
    println!("[WEBHOOK API] Fetching flow version from database");
    let response = match state
        .anything_client
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .eq("flow_version_id", workflow_version_id.clone())
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

    let body = match response.text().await {
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

    let workflow_version: FlowVersion = match serde_json::from_str(&body) {
        Ok(version) => version,
        Err(err) => {
            println!("[WEBHOOK API] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    let account_id = workflow_version.account_id.clone();
    println!("[WEBHOOK API] Account ID from flow version: {}", account_id);
    // Only proceed if we have an account_id
    if account_id == Uuid::nil() {
        println!("[WEBHOOK API] Account ID not found");
        return (StatusCode::BAD_REQUEST, "Account ID not found").into_response();
    }

    // Parse the flow definition into a Workflow
    println!("[WEBHOOK API] Parsing workflow definition");
    let workflow: Workflow = match serde_json::from_value(workflow_version.flow_definition) {
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

    // Find the trigger action in the workflow
    println!("[WEBHOOK API] Looking for trigger node in workflow");
    let trigger_node = match workflow
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
    {
        Some(trigger) => trigger,
        None => {
            println!("[WEBHOOK API] No trigger found in workflow");
            return (StatusCode::BAD_REQUEST, "No trigger found in workflow").into_response();
        }
    };

    // Check if trigger node has plugin_id of "input"
    if trigger_node.plugin_id != "webhook" {
        println!(
            "[WEBHOOK API] Invalid trigger type: {}",
            trigger_node.plugin_id
        );
        return (
            StatusCode::BAD_REQUEST,
            "Workflow trigger must be an webhook trigger to receive webhook",
        )
            .into_response();
    }

    //We need to use the action definition to generate the config
    //This has to take the incoming body and headers as an argument and parse them into the variables

    // Create a task to initiate the flow
    println!("[WEBHOOK API] Creating task for workflow execution");
    let task = CreateTaskInput {
        account_id: account_id.to_string(),
        processing_order: 0,
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version_id.clone(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id: Uuid::new_v4().to_string(),
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
            "body": payload.clone(),
        })),
        test_config: None,
    };

    println!("[WEBHOOK API] Task to be created: {:?}", task);

    let create_task_response = match state
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
    let body = match create_task_response.text().await {
        Ok(body) => {
            println!("[WEBHOOK API] Response body: {:?}", body);
            body
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to get response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get response body",
            )
                .into_response();
        }
    };

    let task_response: Value = match serde_json::from_str(&body) {
        Ok(json) => {
            println!("[WEBHOOK API] Parsed JSON: {:?}", json);
            json
        }
        Err(err) => {
            println!("[WEBHOOK API] Failed to parse response JSON: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse response",
            )
                .into_response();
        }
    };

    println!("[WEBHOOK API] Parsed task response: {:?}", task_response);

    // Send signal to task engine to process the new task for this webhook
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
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



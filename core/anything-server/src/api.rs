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
use crate::{
    secrets::get_secret_by_secret_value,
    workflow_types::{CreateTaskInput, FlowVersion, Workflow},
    CachedApiKey,
};
use crate::{workflow_types::Action, AppState};

use crate::{task_types::ActionType, FlowCompletion};

use tokio::sync::oneshot;
use tokio::time::timeout;

pub async fn run_workflow_and_respond(
    Path(workflow_id): Path<String>, // Changed to tuple extraction
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow and respond");
    println!("[WEBHOOK API] Payload: {:?}", payload);

    println!("[WEBHOOK API] Workflow ID: {}: ", workflow_id,);

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
        // .eq("account_id", account_id.clone())
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

pub async fn validate_webhook_inputs_and_outputs(
    workflow: &Workflow,
    require_output: bool,
) -> Result<(Box<&Action>, Option<Box<&Action>>), impl IntoResponse> {
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
            return Err((StatusCode::BAD_REQUEST, "No trigger found in workflow").into_response());
        }
    };

    // Check if trigger node has plugin_id of "webhook"
    if trigger_node.plugin_id != "webhook" {
        println!(
            "[WEBHOOK API] Invalid trigger type: {}",
            trigger_node.plugin_id
        );
        return Err((
            StatusCode::BAD_REQUEST,
            "Workflow trigger must be an webhook trigger to receive webhook",
        )
            .into_response());
    }

    let mut output_node = None;
    // Check for output node if required
    if require_output {
        println!("[WEBHOOK API] Looking for output node in workflow");
        output_node = match workflow
            .actions
            .iter()
            .find(|action| action.plugin_id == "output")
        {
            Some(output) => Some(Box::new(output)),
            None => {
                println!("[WEBHOOK API] No output node found in workflow");
                return Err(
                    (StatusCode::BAD_REQUEST, "No output node found in workflow").into_response(),
                );
            }
        };
    }

    Ok((Box::new(trigger_node), output_node))
}

async fn validate_security_model(
    rendered_inputs: &Value,
    headers: &HeaderMap,
    state: Arc<AppState>,
) -> impl IntoResponse {
    // Extract the security model from the rendered inputs
    println!("[WEBHOOK API] Extracting security model from rendered inputs");
    let security_model = rendered_inputs
        .get("security_model")
        .and_then(|v| v.as_str())
        .unwrap_or("none");

    println!(
        "[WEBHOOK API] Validating security with model: {}",
        security_model
    );

    match security_model {
        "none" => {
            println!("[WEBHOOK API] No security validation required");
            Ok(())
        }
        "basic_auth" => {
            println!("[WEBHOOK API] Validating Basic Auth");
            let expected_username = rendered_inputs.get("username").and_then(|v| v.as_str());
            let expected_password = rendered_inputs.get("password").and_then(|v| v.as_str());

            if expected_username.is_none() || expected_password.is_none() {
                println!("[WEBHOOK API] Missing username or password configuration");
                return Err((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
            }

            let auth_header = match headers.get("authorization") {
                Some(header) => header,
                None => {
                    println!("[WEBHOOK API] No Authorization header found");
                    return Err(
                        (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response()
                    );
                }
            };

            let auth_str = String::from_utf8_lossy(auth_header.as_bytes());
            if !auth_str.starts_with("Basic ") {
                println!("[WEBHOOK API] Invalid Authorization header format");
                return Err(
                    (StatusCode::UNAUTHORIZED, "Invalid Authorization header").into_response()
                );
            }

            let credentials = match base64::decode(&auth_str[6..]) {
                Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
                Err(_) => {
                    println!("[WEBHOOK API] Failed to decode Basic Auth credentials");
                    return Err((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
                }
            };

            let parts: Vec<&str> = credentials.split(':').collect();
            if parts.len() != 2
                || parts[0] != expected_username.unwrap()
                || parts[1] != expected_password.unwrap()
            {
                println!("[WEBHOOK API] Invalid Basic Auth credentials");
                return Err((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
            }
            Ok(())
        }
        "api_key" => {
            println!("[WEBHOOK API] Validating API Key");
            let api_key = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
                Some(header) if header.starts_with("Bearer ") => header[7..].to_string(),
                _ => {
                    return Err(
                        (StatusCode::UNAUTHORIZED, "Missing or invalid API key").into_response()
                    );
                }
            };

            // Validate the API key
            match validate_api_key(state, api_key.clone()).await {
                Ok(_account_id) => Ok(()),
                Err(status) => Err((status, "Invalid API key").into_response()),
            }
        }
        "custom_header" => {
            println!("[WEBHOOK API] Validating custom header");
            let header_name = match rendered_inputs
                .get("custom_header_name")
                .and_then(|v| v.as_str())
            {
                Some(name) => name,
                None => {
                    println!("[WEBHOOK API] No custom header name configured");
                    return Err(
                        (StatusCode::UNAUTHORIZED, "Invalid header configuration").into_response()
                    );
                }
            };

            let expected_value = match rendered_inputs
                .get("custom_header_value")
                .and_then(|v| v.as_str())
            {
                Some(value) => value,
                None => {
                    println!("[WEBHOOK API] No custom header value configured");
                    return Err(
                        (StatusCode::UNAUTHORIZED, "Invalid header configuration").into_response()
                    );
                }
            };

            let header_value = match headers.get(header_name) {
                Some(value) => String::from_utf8_lossy(value.as_bytes()),
                None => {
                    println!("[WEBHOOK API] Required custom header not found");
                    return Err(
                        (StatusCode::UNAUTHORIZED, "Missing required header").into_response()
                    );
                }
            };

            if header_value != expected_value {
                println!("[WEBHOOK API] Invalid custom header value");
                return Err((StatusCode::UNAUTHORIZED, "Invalid header value").into_response());
            }
            Ok(())
        }
        _ => {
            println!("[WEBHOOK API] Invalid security model specified");
            Err((StatusCode::BAD_REQUEST, "Invalid security model").into_response())
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

pub async fn validate_api_key(state: Arc<AppState>, api_key: String) -> Result<String, StatusCode> {
    println!("[VALIDATE API KEY] Starting API key validation");

    // Check cache first
    let cached_account = {
        println!("[VALIDATE API KEY] Checking cache for API key");
        let cache = state.api_key_cache.read().await;
        if let Some(cached) = cache.get(&api_key) {
            println!("[VALIDATE API KEY] Found cached API key");
            Some(cached.account_id.clone())
        } else {
            println!("[VALIDATE API KEY] API key not found in cache");
            None
        }
    };

    // Return early if we have a valid cached value
    if let Some(account_id) = cached_account {
        println!("[VALIDATE API KEY] Returning cached account ID");
        return Ok(account_id);
    }

    // Not in cache, check database
    println!("[VALIDATE API KEY] Checking database for API key");
    let secret = match get_secret_by_secret_value(state.clone(), api_key.clone()).await {
        Ok(secret) => {
            println!("[VALIDATE API KEY] Found secret in database");
            secret
        }
        Err(_) => {
            println!("[VALIDATE API KEY] Secret not found in database");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify this is an API key secret
    if !secret.anything_api_key {
        println!("[VALIDATE API KEY] Secret is not an API key");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update cache with new value
    {
        println!("[VALIDATE API KEY] Updating cache with new API key");
        let mut cache = state.api_key_cache.write().await;
        cache.insert(
            api_key,
            CachedApiKey {
                account_id: secret.account_id.clone(),
                secret_id: uuid::Uuid::parse_str(&secret.secret_id).unwrap(),
                secret_name: secret.secret_name.clone(),
            },
        );
    }

    println!("[VALIDATE API KEY] API key validation successful");
    Ok(secret.account_id)
}

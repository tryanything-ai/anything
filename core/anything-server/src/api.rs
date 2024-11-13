use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use dotenv::dotenv;
use serde_json::{json, Value};
use std::{collections::HashMap, env, sync::Arc};
use uuid::Uuid;

use crate::task_types::Stage;
use crate::workflow_types::{CreateTaskInput, FlowVersion, Workflow};
use crate::AppState;
use crate::{supabase_auth_middleware::User, task_types::ActionType};

pub async fn run_workflow(
    Path(params): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow");
    println!("[WEBHOOK API] Payload: {:?}", payload);

    // Split the path to extract workflow_id and optional respond path
    let parts: Vec<&str> = params.split('/').collect();
    let workflow_id = parts[0].to_string();
    let respond_path = if parts.len() > 1 {
        Some(parts[1..].join("/"))
    } else {
        None
    };

    println!("[WEBHOOK API] Workflow ID: {}", workflow_id);
    println!("[WEBHOOK API] Respond Path: {:?}", respond_path);

    Json(payload).into_response()
}

pub async fn run_workflow_version(
    Path((workflow_id, workflow_version_id)): Path<(String, String)>, // Changed to tuple extraction
    State(state): State<Arc<AppState>>,
    headers: HeaderMap, //TODO: use when we are doing HMAC secrets
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("[WEBHOOK API] Handling run workflow version");
    println!("[WEBHOOK API] Payload: {:?}", payload);

    // Split the path to extract workflow_id, version_id and optional respond path
    // let parts: Vec<&str> = params.split('/').collect();
    // let (workflow_id, workflow_version_id) = (parts[0].to_string(), parts[1].to_string());
    // println!(
    //     "[WEBHOOK API] Workflow ID: {}, Version ID: {}",
    //     workflow_id, workflow_version_id
    // );

    println!(
        "[WEBHOOK API] Workflow ID: {}, Version ID: {}",
        workflow_id, workflow_version_id
    );

    //TODO: add when we are making "responses" possible
    // let respond_path = if parts.len() > 2 {
    //     Some(parts[2..].join("/"))
    // } else {
    //     None
    // };

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

    let account_id = workflow_version.flow_id.to_string();
    println!("[WEBHOOK API] Account ID from flow version: {}", account_id);

    // Only proceed if we have an account_id
    if account_id.is_empty() {
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
        return (StatusCode::BAD_REQUEST, "Trigger must be an input trigger").into_response();
    }

    //We need to use the action definition to generate the config
    //This has to take the incoming body and headers as an argument and parse them into the variables

    // Create a task to initiate the flow
    println!("[WEBHOOK API] Creating task for workflow execution");
    let task = CreateTaskInput {
        account_id: account_id.to_string(),
        processing_order: 0,
        task_status: "PENDING".to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version_id.clone(),
        action_label: trigger_node.label.clone(),
        trigger_id: trigger_node.action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: "PENDING".to_string(),
        flow_session_id: Uuid::new_v4().to_string(),
        flow_session_status: "PENDING".to_string(),
        action_id: Uuid::new_v4().to_string(),
        r#type: ActionType::Trigger,
        plugin_id: trigger_node.plugin_id.clone(),
        stage: if workflow_version.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: json!({ //TODO: update with actual config?
            "headers": headers.iter().map(|(k,v)| (k.as_str(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect::<HashMap<_,_>>(),
            "body": serde_json::from_str(&body).unwrap_or(json!({}))
        }),
        test_config: None,
    };

    println!("[WEBHOOK API] Task created successfully");
    Json(task).into_response()
}

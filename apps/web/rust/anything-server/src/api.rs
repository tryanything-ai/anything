use axum::{
    extract::{Path, State, Extension},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::{auth::User, task_types::{ActionType, FlowSessionStatus, TaskStatus, TriggerSessionStatus}};
use crate::workflow_types::{Workflow, CreateTaskInput, TestConfig, TaskConfig};
use crate::AppState; 
use crate::task_types::Stage; 
use uuid::Uuid;

use dotenv::dotenv; 
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseFlowVersionInput {
    account_id: String,
    flow_id: String, 
    flow_version: String, 
    flow_definition: Value, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowHandleInput {
    flow_id: String,
    flow_name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowInput {
    flow_id: String,
    flow_name: String,
    account_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWorkflowInput {
    flow_id: String,
    flow_name: String
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_workflows(
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>
) -> impl IntoResponse {
    println!("Handling a get_workflows");

    let client = &state.client;

    let response = match client
        .from("flows")
        .auth(&user.jwt) // Pass a reference to the JWT
        // .eq("archived", "false")
        .select("*,flow_versions(*)")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response();
        },
    };

    if response.status() == 204 {
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to read response body: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response();
        },
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        },
    };

    Json(items).into_response()
}


pub async fn get_workflow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>
) -> impl IntoResponse {

    let client = &state.client;

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .select("*,flow_versions(*)")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(item).into_response()
}

pub async fn get_flow_versions(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>
) -> impl IntoResponse {

    let client = &state.client;

    let response = match client
        .from("flow_versions")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}

pub async fn create_workflow(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<CreateWorkflowHandleInput>,
) -> impl IntoResponse {

    println!("Handling a create_workflow");

    let client = &state.client;

    let input = CreateWorkflowInput {
        flow_id: payload.flow_id.clone(),
        flow_name: payload.flow_name.clone(),
        account_id: user.account_id.clone()
    }; 

    println!("Workflow: {:?}", input);

    let jwt = user.jwt.clone();
    // Create Flow
    let response = match client
        .from("flows")
        .auth(jwt)
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let version_input = BaseFlowVersionInput {
        account_id: user.account_id.clone(),
        flow_id: payload.flow_id.clone(),
        flow_version: "0.0.1".to_string(),
        flow_definition: serde_json::json!({
            "actions": [],
        })
    };

    // let clonedUser = user.clone();
    
    //Create Flow Version
    let version_response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&version_input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };


    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    Json(body).into_response()
}

pub async fn delete_workflow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>
) -> impl IntoResponse {

    let client = &state.client;

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .update("{\"archived\": true}")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    Json(body).into_response()
}

//TODO: validate schema. make sure its not a published flow
pub async fn update_workflow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<UpdateWorkflowInput>,  
) -> impl IntoResponse {
    
    let client = &state.client;

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("id", &flow_id)
        .update(serde_json::to_string(&payload).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    Json(body).into_response()
}

pub async fn update_workflow_version(
    Path((workflow_id, workflow_version_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {

    // let payload_json = serde_json::to_string(&payload).unwrap();
    let client = &state.client;

    let update_json = serde_json::json!({
        "flow_definition": payload,
    });
    
    let response = match client
        .from("flow_versions")
        .auth(user.jwt)
        .eq("flow_version_id", &workflow_version_id)
        .update(update_json.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    Json(body).into_response()
}


// Actions
pub async fn get_actions(
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>
) -> impl IntoResponse {
    println!("Handling a get_actions");

    let client = &state.client;

    let response = match client
        .from("action_templates")
        .auth(user.jwt)
        .eq("archived", "false")
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}


// Testing a workflow
pub async fn test_workflow(
    Path((workflow_id, workflow_version_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>
) -> impl IntoResponse {

    let client = &state.client;

    println!("Handling test workflow");

    // GET the workflow_version
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    // println!("Response from flow_versions: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    // println!("Body from flow_versions: {:?}", body);

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    // println!("Items from flow_versions: {:?}", items);

    let db_version_def = match items.get(0) {
        Some(item) => item,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get item zero").into_response(),
    };

    // println!("db_version_def: {:?}", db_version_def);

      // Parse response into Workflow type
    let flow_definition = match db_version_def.get("flow_definition") {
        Some(flow_definition) => flow_definition,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get flow_definition").into_response(),
    };

    // println!("flow_definition: {:?}", flow_definition);

    let workflow: Workflow = match serde_json::from_value(flow_definition.clone()) {
        Ok(workflow) => workflow,
        Err(err) => {
            println!("Failed to parse flow_definition into Workflow: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse flow_definition into Workflow: {:?}", err)).into_response();
        }
    };


    let task_config = TaskConfig {
        variables: serde_json::json!(workflow.actions[0].variables), 
        inputs: serde_json::json!(workflow.actions[0].input), 
    }; 

    let trigger_session_id = Uuid::new_v4().to_string();
    let flow_session_id = Uuid::new_v4().to_string();

    let input = CreateTaskInput {
        account_id: user.account_id.clone(),
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version_id.clone(),
        action_label: workflow.actions[0].label.clone(),
        trigger_id: workflow.actions[0].node_id.clone(),
        trigger_session_id: trigger_session_id.clone(),
        trigger_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        flow_session_id: flow_session_id.clone(),
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        node_id: workflow.actions[0].node_id.clone(),
        action_type: ActionType::Trigger,
        plugin_id: workflow.actions[0].plugin_id.clone(),
        stage: Stage::Testing.as_str().to_string(),
        config: serde_json::json!(task_config), 
        test_config: None,
        processing_order: 0,
    }; 

    // println!("Input: {:?}", input);

     //Get service_role priveledges by passing service_role in auth()
     dotenv().ok();
     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    // Signal the task processing loop and write error if it can't
    if let Err(err) = state.task_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
    }

    Json(serde_json::json!({
        "flow_session_id": flow_session_id,
        "trigger_session_id": trigger_session_id
    })).into_response()
}

//Just ask the user for dummy data and send it up when they do the call
// Testing a workflow
pub async fn test_action(
    Path((workflow_id, workflow_version_id, action_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>
) -> impl IntoResponse {

    println!("Handling test workflow action");

    let client = &state.client;

    // GET the workflow_version
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    // println!("Response from flow_versions: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    // println!("Body from flow_versions: {:?}", body);

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    // println!("Items from flow_versions: {:?}", items);

    let db_version_def = match items.get(0) {
        Some(item) => item,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get item zero").into_response(),
    };

    // println!("db_version_def: {:?}", db_version_def);

      // Parse response into Workflow type
    let flow_definition = match db_version_def.get("flow_definition") {
        Some(flow_definition) => flow_definition,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get flow_definition").into_response(),
    };

    println!("flow_definition: {:?}", flow_definition);

    let workflow: Workflow = match serde_json::from_value(flow_definition.clone()) {
        Ok(workflow) => workflow,
        Err(err) => {
            println!("Failed to parse flow_definition into Workflow: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse flow_definition into Workflow: {:?}", err)).into_response();
        }
    };

    // Use the `workflow` variable as needed
    // println!("Workflow Definition {:#?}", workflow);

    let task_config = TaskConfig {
        variables: serde_json::json!(workflow.actions[0].variables), 
        inputs: serde_json::json!(workflow.actions[0].input), 
    }; 

    let test_config = TestConfig {
        action_id: Some(action_id.clone()),
        variables: serde_json::json!({}), //TODO: we should take this from like a body as a one time argument for the action
        inputs: serde_json::json!({}),
    }; 

    let input = CreateTaskInput {
        account_id: user.account_id.clone(),
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version_id.clone(),
        action_label: workflow.actions[0].label.clone(),
        trigger_id: workflow.actions[0].node_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id: Uuid::new_v4().to_string(),
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        node_id: workflow.actions[0].node_id.clone(),
        action_type: workflow.actions[0].action_type.clone(),
        plugin_id: workflow.actions[0].plugin_id.clone(),
        stage: Stage::Testing.as_str().to_string(), 
        config: serde_json::json!(task_config), 
        test_config: Some(serde_json::json!(test_config)),
        processing_order: 0
    }; 

    // println!("Input: {:?}", input);

     //Get service_role priveledges by passing service_role in auth()
     dotenv().ok();
     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

     // Signal the task processing loop and write error if it can't
     // This is just a hint to the processing system. Processing is lazy sometimes to prevent using resources when not needed
     if let Err(err) = state.task_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
    }

    Json(items).into_response()
}

// Actions
pub async fn get_test_session_results(
    Path((workflow_id, workflow_version_id, session_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>, 
    Extension(user): Extension<User>
) -> impl IntoResponse {
    println!("Handling a get_test_session_results");

let client = &state.client;

    let response = match client
        .from("tasks")
        .auth(user.jwt)
        .eq("flow_session_id", &session_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("processing_order.asc")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    let all_completed = items.as_array().map_or(false, |tasks| {
        tasks.iter().all(|task| {
            task.get("flow_session_status") == Some(&Value::String("completed".to_string())) &&
            task.get("trigger_session_status") == Some(&Value::String("completed".to_string()))
        })
    });

    let result = serde_json::json!({
        "tasks": items,
        "complete": all_completed
    });

    Json(result).into_response()
}

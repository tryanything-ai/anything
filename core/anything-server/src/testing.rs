use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use chrono::Utc;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use crate::{
    processor::{flow_session_cache::FlowSessionData, processor::ProcessorMessage},
    supabase_jwt_middleware::User,
    types::{
        action_types::ActionType,
        task_types::{
            CreateTaskInput, FlowSessionStatus, Stage, Task, TaskConfig, TaskStatus, TestConfig,
            TriggerSessionStatus,
        },
        workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
    },
    AppState,
};
use uuid::Uuid;

use dotenv::dotenv;
use std::env;

// Testing a workflow
pub async fn test_workflow(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    println!("[TESTING] Handling test workflow request");

    // GET the workflow_version
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .eq("account_id", &account_id)
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            println!("[TESTING] Failed to execute request to get workflow version");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            println!("[TESTING] Failed to read response body");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let workflow_version: DatabaseFlowVersion = match serde_json::from_str(&body) {
        Ok(dbflowversion) => dbflowversion,
        Err(e) => {
            println!("[TESTING] Failed to parse workflow version JSON: {}", e);
            println!("[TESTING] Raw JSON body: {}", body);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse JSON: {}", e),
            )
                .into_response();
        }
    };

    println!("[TESTING] Successfully retrieved workflow version");

    // Find the trigger action
    let trigger_action = match workflow_version
        .flow_definition
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
    {
        Some(action) => action,
        None => {
            println!("[TESTING] No trigger action found in workflow");
            return (
                StatusCode::BAD_REQUEST,
                "No trigger action found in workflow",
            )
                .into_response();
        }
    };

    let task_config = TaskConfig {
        inputs: Some(serde_json::json!(trigger_action.inputs)),
        inputs_schema: Some(trigger_action.inputs_schema.clone().unwrap()),
        plugin_config: Some(trigger_action.plugin_config.clone()),
        plugin_config_schema: Some(trigger_action.plugin_config_schema.clone()),
    };

    let trigger_session_id = Uuid::new_v4().to_string();
    let flow_session_id = Uuid::new_v4().to_string();

    println!("[TESTING] Creating task input");
    let input = CreateTaskInput {
        account_id: account_id.clone(),
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version_id.clone(),
        action_label: trigger_action.label.clone(),
        trigger_id: trigger_action.action_id.clone(),
        trigger_session_id: trigger_session_id.clone(),
        trigger_session_status: FlowSessionStatus::Running.as_str().to_string(),
        flow_session_id: flow_session_id.clone(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: trigger_action.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_name: trigger_action.plugin_name.clone(),
        plugin_version: trigger_action.plugin_version.clone(),
        stage: Stage::Testing.as_str().to_string(),
        config: task_config,
        result: Some(serde_json::json!({
            "message": format!("Successfully triggered task"),
            "created_at": Utc::now()
        })),
        error: None,
        test_config: None,
        processing_order: 0,
        started_at: Some(Utc::now()),
    };

    println!("[TESTING] Creating processor message");
    // Send message to processor
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        version_id: Some(Uuid::parse_str(&workflow_version_id).unwrap()),
        flow_session_id: Uuid::parse_str(&flow_session_id).unwrap(),
        trigger_session_id: Uuid::parse_str(&trigger_session_id).unwrap(),
        trigger_task: Some(input),
    };

    println!("[TESTING] Initializing flow session data");
    // Initialize flow session data and set it in the cache
    //When we set it in the cache we don't need to fetch it again
    //But since testing is special we want to create our own task
    //To send to the processor
    let flow_session_data = FlowSessionData {
        workflow: Some(workflow_version),
        tasks: HashMap::new(),
        flow_session_id: Uuid::parse_str(&flow_session_id).unwrap(),
        workflow_id: Uuid::parse_str(&workflow_id).unwrap(),
        workflow_version_id: Some(Uuid::parse_str(&workflow_version_id).unwrap()),
    };

    println!("[TESTING] Setting flow session data in cache");
    // Set the flow session data in cache
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(
            &Uuid::parse_str(&flow_session_id).unwrap(),
            flow_session_data,
        );
    }

    if let Err(e) = state.processor_sender.send(processor_message).await {
        println!("[TESTING] Failed to send message to processor: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send message to processor: {}", e),
        )
            .into_response();
    }

    println!("[TESTING] Successfully initiated test workflow");
    Json(serde_json::json!({
        "flow_session_id": flow_session_id,
        "trigger_session_id": trigger_session_id
    }))
    .into_response()
}

//Just ask the user for dummy data and send it up when they do the call
// Testing a workflow
pub async fn test_action(
    Path((account_id, workflow_id, workflow_version_id, action_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling test workflow action");

    let client = &state.anything_client;

    // GET the workflow_version
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .eq("account_id", &account_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    // println!("Response from flow_versions: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    // println!("Body from flow_versions: {:?}", body);

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    // println!("Items from flow_versions: {:?}", items);

    let db_version_def = match items.get(0) {
        Some(item) => item,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get item zero").into_response()
        }
    };

    // println!("db_version_def: {:?}", db_version_def);

    // Parse response into Workflow type
    let flow_definition = match db_version_def.get("flow_definition") {
        Some(flow_definition) => flow_definition,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get flow_definition",
            )
                .into_response()
        }
    };

    println!("flow_definition: {:?}", flow_definition);

    let workflow: WorkflowVersionDefinition = match serde_json::from_value(flow_definition.clone())
    {
        Ok(workflow) => workflow,
        Err(err) => {
            println!("Failed to parse flow_definition into Workflow: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse flow_definition into Workflow: {:?}", err),
            )
                .into_response();
        }
    };

    // Use the `workflow` variable as needed
    // println!("Workflow Definition {:#?}", workflow);

    let task_config = TaskConfig {
        inputs: Some(workflow.actions[0].inputs.clone().unwrap()),
        inputs_schema: Some(workflow.actions[0].inputs_schema.clone().unwrap()),
        plugin_config: Some(workflow.actions[0].plugin_config.clone()),
        plugin_config_schema: Some(workflow.actions[0].plugin_config_schema.clone()),
    };

    let test_config = TestConfig {
        action_id: Some(action_id.clone()),
        variables: serde_json::json!({}), //TODO: we should take this from like a body as a one time argument for the action
        inputs: serde_json::json!({}),
    };

    let input = CreateTaskInput {
        account_id: account_id.clone(),
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: workflow_id.clone(),
        flow_version_id: workflow_version_id.clone(),
        action_label: workflow.actions[0].label.clone(),
        trigger_id: workflow.actions[0].action_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: Uuid::new_v4().to_string(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: workflow.actions[0].action_id.clone(),
        r#type: workflow.actions[0].r#type.clone(),
        plugin_name: workflow.actions[0].plugin_name.clone(),
        plugin_version: workflow.actions[0].plugin_version.clone(),
        stage: Stage::Testing.as_str().to_string(),
        config: task_config,
        result: None,
        error: None,
        test_config: Some(serde_json::json!(test_config)),
        processing_order: 0,
        started_at: Some(Utc::now()),
    };

    // println!("Input: {:?}", input);

    //Get service_role priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    // Signal the task processing loop and write error if it can't
    // This is just a hint to the processing system. Processing is lazy sometimes to prevent using resources when not needed

    Json(items).into_response()
}

// Actions
pub async fn get_test_session_results(
    Path((account_id, workflow_id, workflow_version_id, session_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TESTING] Handling get_test_session_results request");
    println!(
        "[TESTING] Getting results for session {} in workflow {} version {}",
        session_id, workflow_id, workflow_version_id
    );

    let client = &state.anything_client;

    println!("[TESTING] Querying tasks table for session results");
    let response = match client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("flow_session_id", &session_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("processing_order.asc")
        .execute()
        .await
    {
        Ok(response) => {
            println!("[TESTING] Successfully queried tasks table");
            response
        }
        Err(e) => {
            println!("[TESTING] Failed to execute request to get tasks: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[TESTING] Successfully read response body");
            body
        }
        Err(e) => {
            println!("[TESTING] Failed to read response body: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let tasks: Vec<Task> = match serde_json::from_str::<Vec<Task>>(&body) {
        Ok(tasks) => {
            println!("[TESTING] Successfully parsed {} tasks", tasks.len());
            tasks
        }
        Err(e) => {
            println!("[TESTING] Failed to parse tasks JSON: {}", e);
            println!("[TESTING] Raw JSON body: {}", body);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse tasks").into_response();
        }
    };

    println!("[TESTING] Checking completion status of tasks");
    let all_completed = !tasks.is_empty()
        && tasks.iter().all(|task| {
            matches!(
                task.trigger_session_status,
                TriggerSessionStatus::Completed | TriggerSessionStatus::Failed
            )
        });

    println!("[TESTING] Session completion status: {}", all_completed);
    let result = serde_json::json!({
        "tasks": tasks,
        "complete": all_completed
    });

    println!("[TESTING] Successfully returning test session results");
    Json(result).into_response()
}

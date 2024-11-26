use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::Value;
use std::sync::Arc;

use crate::{bundler::bundle_cached_variables, supabase_jwt_middleware::User, AppState};

use crate::workflow_types::{FlowVersion, WorkflowVersionDefinition};

// Actions
pub async fn get_flow_version_results(
    Path((account_id, workflow_id, workflow_version_id, action_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[VARIABLES] Handling get_flow_version_variables request for account: {}, workflow: {}, version: {}, action: {}", 
        account_id, workflow_id, workflow_version_id, action_id);

    let client = &state.anything_client;

    // Get last session
    println!("[VARIABLES] Fetching last task for workflow");
    let response = match client
        .from("tasks")
        .auth(user.jwt.clone())
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("created_at.desc")
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "[VARIABLES] Response from fetching last task: {:?}",
                response
            );
            response
        }
        Err(e) => {
            println!("[VARIABLES] Error fetching last task: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[VARIABLES] Error reading response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let task: Value = match serde_json::from_str::<Vec<Value>>(&body) {
        Ok(tasks) => {
            if tasks.is_empty() {
                println!("[VARIABLES] No tasks found");
                return (StatusCode::NOT_FOUND, "No tasks found").into_response();
            }
            println!("[VARIABLES] First task: {:?}", tasks[0]);
            tasks[0].clone()
        }
        Err(e) => {
            println!("[VARIABLES] Error parsing JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    let session_id = match task.get("flow_session_id") {
        Some(id) => id.as_str().unwrap_or("").to_string(),
        None => {
            println!("[VARIABLES] Failed to get flow_session_id from task");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get flow_session_id",
            )
                .into_response();
        }
    };

    println!("[VARIABLES] Found session_id: {}", session_id);
    println!("[VARIABLES] Fetching tasks for session");

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
        Ok(response) => response,
        Err(e) => {
            println!("[VARIABLES] Error fetching tasks: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[VARIABLES] Error reading tasks response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => {
            println!("[VARIABLES] Parsed items: {:?}", items);
            items
        }
        Err(e) => {
            println!("[VARIABLES] Error parsing tasks JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    // Find the processing order of the target action
    let target_processing_order = items
        .as_array()
        .and_then(|tasks| {
            tasks
                .iter()
                .find(|task| task.get("action_id").and_then(|id| id.as_str()) == Some(&action_id))
        })
        .and_then(|task| task.get("processing_order"))
        .and_then(|order| order.as_i64());

    println!(
        "[VARIABLES] Found target processing order: {:?}",
        target_processing_order
    );

    // Filter tasks to only include those with lower processing order
    let filtered_items = match target_processing_order {
        Some(target_order) => Value::Array(
            items
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter(|task| {
                    task.get("processing_order")
                        .and_then(|order| order.as_i64())
                        .map_or(false, |order| order < target_order)
                })
                .cloned()
                .collect(),
        ),
        None => items,
    };

    let items = filtered_items;

    let result = serde_json::json!({
        "tasks": items
    });

    println!("[VARIABLES] Returning response");
    Json(result).into_response()
}

// Variables
pub async fn get_flow_version_variables(
    Path((account_id, workflow_id, workflow_version_id, action_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[VARIABLES] Handling get_flow_version_variables request for account: {}, workflow: {}, version: {}, action: {}", 
        account_id, workflow_id, workflow_version_id, action_id);

    let client = &state.anything_client;

    // Get last session
    println!("[VARIABLES] Fetching last task for workflow");
    let response = match client
        .from("tasks")
        .auth(user.jwt.clone())
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .eq("flow_version_id", &workflow_version_id)
        .select("*")
        .order("created_at.desc")
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "[VARIABLES] Response from fetching last task: {:?}",
                response
            );
            response
        }
        Err(e) => {
            println!("[VARIABLES] Error fetching last task: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[VARIABLES] Error reading response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let task: Value = match serde_json::from_str::<Vec<Value>>(&body) {
        Ok(tasks) => {
            if tasks.is_empty() {
                println!("[VARIABLES] No tasks found");
                return (StatusCode::NOT_FOUND, "No tasks found").into_response();
            }
            println!("[VARIABLES] First task: {:?}", tasks[0]);
            tasks[0].clone()
        }
        Err(e) => {
            println!("[VARIABLES] Error parsing JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    let session_id = match task.get("flow_session_id") {
        Some(id) => id.as_str().unwrap_or("").to_string(),
        None => {
            println!("[VARIABLES] Failed to get flow_session_id from task");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get flow_session_id",
            )
                .into_response();
        }
    };

    println!("[VARIABLES] Found session_id: {}", session_id);

    // Fetch the current flow version from the database
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .select("*")
        .eq("flow_version_id", &workflow_version_id)
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[VARIABLES] Error fetching flow version: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch flow version",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[VARIABLES] Error reading flow version response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read flow version response",
            )
                .into_response();
        }
    };

    let flow_version = match serde_json::from_str::<Vec<FlowVersion>>(&body) {
        Ok(versions) => match versions.into_iter().next() {
            Some(version) => {
                print!("Found flow version: {:?}", version);
                version
            }
            None => {
                println!(
                    "[VARIABLES] No flow version found for id: {}",
                    workflow_version_id
                );
                return (StatusCode::NOT_FOUND, "Flow version not found").into_response();
            }
        },
        Err(e) => {
            println!("[VARIABLES] Error parsing flow version JSON: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse flow version",
            )
                .into_response();
        }
    };

    // Parse the flow definition into a Workflow struct
    let workflow: WorkflowVersionDefinition =
        match serde_json::from_value(flow_version.flow_definition) {
            Ok(workflow) => {
                print!("[VARIABLES] Parsed workflow: {:?}", workflow);
                workflow
            }
            Err(e) => {
                println!("[VARIABLES] Error parsing workflow definition: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse workflow definition",
                )
                    .into_response();
            }
        };

    // Find the action in the workflow and get its variables as a JSON object
    let variables = workflow
        .actions
        .iter()
        .find(|action| action.action_id == action_id)
        .map(|action| {
            print!(
                "[VARIABLES] Found action with correct action id: {:?}",
                action
            );
            serde_json::to_value(&action.variables.inner).unwrap_or(serde_json::json!({}))
        })
        .unwrap_or(serde_json::json!({}));

    println!("[VARIABLES] Found variables: {:?}", variables);

    //Run the templater over the variables and results from last session
    //Return the templated variables
    let rendered_variables = match bundle_cached_variables(
        state.clone(),
        client,
        &account_id,
        &session_id,
        Some(&variables),
        false,
    )
    .await
    {
        Ok(vars) => vars,
        Err(_e) => return Json(serde_json::Value::Null).into_response(),
    };

    //TODO: we could run bundled context on each key individually in case we have a key with a failed template render
    //we would still be able to return the other variables ( might be templater centric refactors that make more sense )
    //TODO: this will feel really awkward when you make a new workflow version that has no history.
    //would be good to imporove this in a way where we can show the data from past runs of the parent flow version vs making them run the thing
    //once to see any data. same is true for the results view
    //this may be a good reason to make "hashes" for actions when they publish so we can check things?
    //TODO: build some sort of tool that can "smell out" if the same api endpoint is being hit? like parse the urls to know its the same endpoint across users etc
    //Store this metadata somewhere usefull

    //Returning both so we can show the keys no matter what if the bundling fails we can still show top level keys
    let response = serde_json::json!({
        "variables": variables,
        "rendered_variables": rendered_variables
    });

    println!("[VARIABLES] Returning response");
    Json(response).into_response()
}

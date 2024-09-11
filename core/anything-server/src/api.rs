use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::task_types::Stage;
use crate::workflow_types::{CreateTaskInput, TaskConfig, TestConfig, Workflow};
use crate::AppState;
use crate::{
    supabase_auth_middleware::User,
    task_types::{ActionType, FlowSessionStatus, TaskStatus, TriggerSessionStatus},
};
use uuid::Uuid;

use dotenv::dotenv;
use std::env;

use chrono::Timelike;
use chrono::{DateTime, Datelike, Duration, Utc};
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseFlowVersionInput {
    account_id: String,
    flow_id: String,
    flow_definition: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowHandleInput {
    flow_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowInput {
    flow_id: String,
    flow_name: String,
    description: String,
    account_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWorkflowInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_workflows(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_workflows");

    let client = &state.anything_client;

    //Orde_with_options docs
    //https://github.com/supabase-community/postgrest-rs/blob/d740c1e739547d6c36482af61fc8673e23232fdd/src/builder.rs#L196
    let response = match client
        .from("flows")
        .auth(&user.jwt) // Pass a reference to the JWT
        // .eq("archived", "false")
        .select(
            "*,draft_workflow_versions:flow_versions(*), published_workflow_versions:flow_versions(*)",
        )
        .eq("archived", "false")
        .eq("account_id", &account_id)
        .eq("draft_workflow_versions.published", "false")
        .order_with_options("created_at", Some("draft_workflow_versions"), false, true)
        .foreign_table_limit(1, "draft_workflow_versions")
        .eq("published_workflow_versions.published", "true")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    if response.status() == 204 {
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(items).into_response()
}

pub async fn get_workflow(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .eq("account_id", &account_id)
        .select("*,flow_versions(*)")
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn get_flow_versions(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("flow_versions")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .eq("account_id", &account_id)
        .select("*")
        .order("created_at.desc")
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

    Json(items).into_response()
}

pub async fn get_flow_version(
    Path((account_id, flow_id, version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("flow_versions")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .eq("flow_version_id", &version_id)
        .eq("account_id", &account_id)
        .select("*")
        .single()
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn create_workflow(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<CreateWorkflowHandleInput>,
) -> impl IntoResponse {
    println!("Handling a create_workflow");

    let client = &state.anything_client;

    let input = CreateWorkflowInput {
        flow_id: payload.flow_id.clone(),
        flow_name: "New Default Flow".to_string(),
        description: "New Default Flow".to_string(),
        account_id: account_id.clone(),
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
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let version_input = BaseFlowVersionInput {
        account_id: account_id.clone(),
        flow_id: payload.flow_id.clone(),
        flow_definition: serde_json::json!(Workflow::default()),
    };
    // Create Flow Version
    let version_response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&version_input).unwrap())
        .single()
        .execute()
        .await
    {
        Ok(response) => {
            println!("Flow version creation response: {:?}", response);
            response
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let body = match version_response.json::<serde_json::Value>().await {
        Ok(body) => serde_json::json!({
            "workflow_id": payload.flow_id,
            "workflow_version_id": body["flow_version_id"].as_str().unwrap_or("")
        }),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    Json(body).into_response()
}

//TODO: we also need to set active to false
pub async fn delete_workflow(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .eq("account_id", &account_id)
        .update("{\"archived\": true, \"active\": false}")
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

    //Let trigger system be aware we deleted a workflow
    if let Err(err) = state.trigger_engine_signal.send(flow_id) {
        println!("Failed to send trigger signal: {:?}", err);
    }

    Json(body).into_response()
}

pub async fn update_workflow(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<UpdateWorkflowInput>,
) -> impl IntoResponse {
    println!("Handling a update_workflow");

    print!("Payload: {:?}", payload);

    let client = &state.anything_client;

    let payload_json = serde_json::to_value(&payload).unwrap();

    //If we are updating active we need to double check if their are any published worfklow versions
    //We don't allow people to make workflows active that do not have published versions.
    //We will let them turn them to not active though. This shouldnt happen but just in case
    if payload_json.get("active").is_some()
        && payload_json.get("active").unwrap().as_bool() == Some(true)
    {
        //TODO: we need to check if the flow has any published versions before we allow it to be made active
        //If it has no published flow_versions we should make an error
        let has_published_flow_version_resopnse = match client
            .from("flow_versions")
            .auth(user.jwt.clone())
            .eq("flow_id", &flow_id)
            .eq("account_id", &account_id)
            .eq("published", "true")
            .select("*")
            .execute()
            .await
        {
            Ok(response) => response,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to check if flow_version is published",
                )
                    .into_response()
            }
        };

        let check_body = match has_published_flow_version_resopnse.text().await {
            Ok(body) => body,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read check response body",
                )
                    .into_response()
            }
        };

        let has_published_flow_version: bool = match serde_json::from_str::<Value>(&check_body) {
            Ok(value) => value.as_array().map_or(false, |arr| !arr.is_empty()),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse check response JSON",
                )
                    .into_response()
            }
        };

        if !has_published_flow_version {
            return (
                StatusCode::BAD_REQUEST,
                "Cannot make flow active without published flow versions",
            )
                .into_response();
        }
    }

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("flow_id", &flow_id)
        .eq("account_id", &account_id)
        .update(serde_json::to_string(&payload).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error: {:?}", err);
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
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    // Signal the trigger processing loop that it needs to hydrate and manage new triggers.
    if payload_json.get("active").is_some() {
        if let Err(err) = state.trigger_engine_signal.send(flow_id) {
            println!("Failed to send trigger signal: {:?}", err);
        }
    }

    Json(body).into_response()
}

pub async fn update_workflow_version(
    Path((workflow_id, workflow_version_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    // Check if the flow_version is published
    let is_flow_version_published_resopnse = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .select("published")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to check if flow_version is published",
            )
                .into_response()
        }
    };

    let check_body = match is_flow_version_published_resopnse.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read check response body",
            )
                .into_response()
        }
    };

    let is_published: bool = match serde_json::from_str::<Value>(&check_body) {
        Ok(value) => value[0]["published"].as_bool().unwrap_or(false),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse check response JSON",
            )
                .into_response()
        }
    };

    //If it is published we need to create a new version to be the draft
    if is_published {
        // Create a new flow_version as a copy of the published one
        let copy_response = match client
            .from("flow_versions")
            .auth(user.jwt.clone())
            .eq("flow_version_id", &workflow_version_id)
            .select("*")
            .execute()
            .await
        {
            Ok(response) => response,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to fetch published flow_version",
                )
                    .into_response()
            }
        };

        let copy_body = match copy_response.text().await {
            Ok(body) => body,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read copy response body",
                )
                    .into_response()
            }
        };

        let mut new_flow_version: Value = match serde_json::from_str::<Vec<Value>>(&copy_body) {
            Ok(value) => value[0].clone(),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse copy response JSON",
                )
                    .into_response()
            }
        };

        // Update the new flow_version with the new payload and reset necessary fields
        new_flow_version["flow_version_id"] = serde_json::json!(Uuid::new_v4().to_string());
        new_flow_version["flow_definition"] = payload;
        new_flow_version["flow_id"] = serde_json::json!(workflow_id);
        new_flow_version["published"] = serde_json::json!(false);
        new_flow_version["published_at"] = serde_json::json!(null);
        new_flow_version["un_published"] = serde_json::json!(false);
        new_flow_version["un_published_at"] = serde_json::json!(null);
        new_flow_version["parent_flow_version_id"] = serde_json::json!(workflow_version_id);

        let insert_response = match client
            .from("flow_versions")
            .auth(user.jwt.clone())
            .insert(new_flow_version.to_string())
            .execute()
            .await
        {
            Ok(response) => response,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to insert new flow_version",
                )
                    .into_response()
            }
        };

        let insert_body = match insert_response.text().await {
            Ok(body) => body,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read insert response body",
                )
                    .into_response()
            }
        };

        return Json(insert_body).into_response();
    }

    //If its not published do the normal thing

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

    Json(body).into_response()
}

pub async fn publish_workflow_version(
    Path((workflow_id, workflow_version_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    println!("Handling publish workflow version");
    println!("workflow id: {}", workflow_id);
    println!("flow-version id: {}", workflow_version_id);

    let client = &state.anything_client;

    let unpublish_json = serde_json::json!({
        "published": false,
        "un_published": true,
        "un_published_at": Utc::now().to_rfc3339(),
    });

    println!("service_role_key: {:?}", &supabase_service_role_api_key);

    println!("workflow id: {:?}", &workflow_id);

    //Need to exclude this flow_version_id so that it doesn't unpublish itself if it gets called twice
    let un_publish_response = match client
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .eq("published", "true")
        .eq("flow_id", &workflow_id)
        .neq("flow_version_id", &workflow_version_id)
        .update(unpublish_json.to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response for un_publish_old: {:?}", response);
            response
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body_2 = match un_publish_response.text().await {
        Ok(body) => {
            println!("Response body: {}", body);
            body
        }
        Err(err) => {
            eprintln!("Error reading response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let update_json = serde_json::json!({
        "published": true,
        "published_at": Utc::now().to_rfc3339(),
    });

    //If called twice won't run because users are not allowed to make updates to flow_versions if published = true based on Database Permission Rules
    let response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .eq("flow_version_id", &workflow_version_id)
        .update(update_json.to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            response
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
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
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    // Signal the trigger processing loop that it needs to hydrate and manage new triggers.
    if let Err(err) = state.trigger_engine_signal.send(workflow_id) {
        println!("Failed to send trigger signal: {:?}", err);
    }

    Json(body).into_response()
}

// Actions
pub async fn get_actions(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_actions");

    let client = &state.anything_client;

    // Fetch data from the database
    println!("Fetching data from the database");
    let response = match client
        .from("action_templates")
        .auth(user.jwt)
        .eq("archived", "false")
        .select("*")
        .execute()
        .await
    {
        Ok(response) => {
            println!(
                "Successfully fetched data from the database: {:?}",
                response
            );
            response
        }
        Err(err) => {
            eprintln!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    println!("Reading response body");
    let body = match response.text().await {
        Ok(body) => {
            println!("Successfully read response body");
            body
        }
        Err(err) => {
            eprintln!("Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    println!("Parsing response body as JSON");
    let mut db_items: Value = match serde_json::from_str(&body) {
        Ok(items) => {
            println!("Successfully parsed JSON: {:?}", items);
            items
        }
        Err(err) => {
            eprintln!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    // Log the current working directory
    println!("Logging the current working directory");
    let current_dir = match env::current_dir() {
        Ok(path) => {
            println!("Current directory: {}", path.display());
            path
        }
        Err(e) => {
            println!("Failed to get current directory: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get current directory",
            )
                .into_response();
        }
    };

    // Load data from the JSON file
    let json_file_path = current_dir.join("action_db/action_templates.json");
    println!("Loading data from the JSON file at {:?}", json_file_path);
    let file = match File::open(&json_file_path) {
        Ok(file) => {
            println!("Successfully opened JSON file");
            file
        }
        Err(err) => {
            eprintln!("Failed to open JSON file: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to open JSON file",
            )
                .into_response();
        }
    };

    println!("Reading JSON file");
    let reader = BufReader::new(file);
    let json_items: Value = match serde_json::from_reader(reader) {
        Ok(items) => {
            println!("Successfully parsed JSON file: {:?}", items);
            items
        }
        Err(err) => {
            eprintln!("Failed to parse JSON file: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse JSON file",
            )
                .into_response();
        }
    };

    // Combine both database and JSON file items into a single array
    println!("Combining database and JSON file items");
    if let Some(db_array) = db_items.as_array_mut() {
        if let Some(json_array) = json_items.as_array() {
            db_array.extend(json_array.clone());
        }
    }

    Json(db_items).into_response()
}

// Testing a workflow
pub async fn test_workflow(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    println!("Handling test workflow");

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

    // println!("flow_definition: {:?}", flow_definition);

    let workflow: Workflow = match serde_json::from_value(flow_definition.clone()) {
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

    let task_config = TaskConfig {
        variables: serde_json::json!(workflow.actions[0].variables),
        inputs: serde_json::json!(workflow.actions[0].input),
    };

    let trigger_session_id = Uuid::new_v4().to_string();
    let flow_session_id = Uuid::new_v4().to_string();

    let input = CreateTaskInput {
        account_id: account_id.clone(),
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
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
    }

    Json(serde_json::json!({
        "flow_session_id": flow_session_id,
        "trigger_session_id": trigger_session_id
    }))
    .into_response()
}

//Just ask the user for dummy data and send it up when they do the call
// Testing a workflow
pub async fn test_action(
    Path((account_id, workflow_id, workflow_version_id, action_id)): Path<(String, String, String, String)>,
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

    let workflow: Workflow = match serde_json::from_value(flow_definition.clone()) {
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
        variables: serde_json::json!(workflow.actions[0].variables),
        inputs: serde_json::json!(workflow.actions[0].input),
    };

    let test_config = TestConfig {
        action_id: Some(action_id.clone()),
        variables: serde_json::json!({}), //TODO: we should take this from like a body as a one time argument for the action
        inputs: serde_json::json!({}),
    };

    let input = CreateTaskInput {
        account_id: account_id.clone(),
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
        processing_order: 0,
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
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
    }

    Json(items).into_response()
}

// Actions
pub async fn get_test_session_results(
    Path((workflow_id, workflow_version_id, session_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_test_session_results");

    let client = &state.anything_client;

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

    let all_completed = items.as_array().map_or(false, |tasks| {
        tasks.iter().all(|task| {
            let flow_status = task.get("flow_session_status");
            let trigger_status = task.get("trigger_session_status");
            let task_status = task.get("task_status");
            (flow_status == Some(&Value::String("completed".to_string()))
                || flow_status == Some(&Value::String("failed".to_string())))
                && (trigger_status == Some(&Value::String("completed".to_string()))
                    || trigger_status == Some(&Value::String("failed".to_string())))
                && (task_status == Some(&Value::String("completed".to_string()))
                    || task_status == Some(&Value::String("canceled".to_string()))
                    || task_status == Some(&Value::String("failed".to_string())))
        })
    });

    let result = serde_json::json!({
        "tasks": items,
        "complete": all_completed
    });

    Json(result).into_response()
}

//Task
pub async fn get_tasks(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get_tasks for account_id: {}", account_id);

    let client = &state.anything_client;

    let response = match client
        .from("tasks")
        .auth(&user.jwt)
        .eq("account_id", &account_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    if response.status() == 204 {
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(items).into_response()
}

pub async fn get_task_by_workflow_id(
    Path((account_id, workflow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .select("*")
        .order("created_at.desc,processing_order.desc")
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn get_auth_provider_by_name(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "Handling a get_auth_provider_by_name for  {:?}",
        provider_name
    );

    let client = &state.anything_client;

    let response = match client
        .from("auth_providers")
        .auth(user.jwt)
        .eq("provider_name", &provider_name)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            response
        }
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn get_auth_accounts_for_provider_name(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "Handling a get_auth_accounts_for_provider_name for  {:?}",
        provider_name
    );

    let client = &state.anything_client;

    let response = match client
        .from("account_auth_provider_accounts")
        .auth(user.jwt)
        .eq("auth_provider_id", &provider_name)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            response
        }
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn get_auth_accounts(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get auth accounts");

    let client = &state.anything_client;

    let response = match client
        .from("account_auth_provider_accounts")
        .auth(user.jwt)
        .select("*, auth_provider:auth_providers(*)")
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            response
        }
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn get_auth_providers(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling a get auth accounts");

    let client = &state.anything_client;

    let response = match client
        .from("auth_providers")
        .auth(user.jwt)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            response
        }
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

#[derive(Serialize)]
struct ChartDataPoint {
    date: String,
    #[serde(flatten)]
    status_counts: HashMap<String, i32>,
}

fn parse_date_or_default(date_str: &str) -> DateTime<Utc> {
    println!("Date Str: {:?}", date_str);
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

pub async fn get_task_status_counts_by_workflow_id(
    Path((account_id, workflow_id, start_date, end_date, time_unit)): Path<(String, String, String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let start = parse_date_or_default(&start_date);
    let end = parse_date_or_default(&end_date);

    println!("Start: {:?}, End: {:?}", start, end);

    let query = client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .select("task_status, created_at")
        .gte("created_at", start.to_rfc3339())
        .lte("created_at", end.to_rfc3339());

    let response = match query.execute().await {
        Ok(response) => {
            println!("Response from tasks w gte y lte: {:?}", response);
            response
        }
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

    let tasks: Vec<Value> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    let interval = match time_unit.as_str() {
        "minute" => Duration::minutes(1),
        "hour" => Duration::hours(1),
        "day" => Duration::days(1),
        "week" => Duration::weeks(1),
        "month" => Duration::days(30), // Approximation
        _ => return (StatusCode::BAD_REQUEST, "Invalid time unit").into_response(),
    };

    // Get all unique statuses from tasks
    let all_statuses: Vec<String> = tasks
        .iter()
        .filter_map(|task| task["task_status"].as_str())
        .map(|s| s.to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let mut date_status_counts: HashMap<DateTime<Utc>, HashMap<String, i32>> = HashMap::new();

    // Initialize all intervals with zero counts for all statuses
    let mut current = start;
    while current <= end {
        let mut status_counts = HashMap::new();
        for status in &all_statuses {
            status_counts.insert(status.clone(), 0);
        }
        date_status_counts.insert(current, status_counts);
        current += interval;
    }

    // println!("Date Status Counts: {:?}", date_status_counts);

    // Process tasks
    for task in tasks {
        let status = task["task_status"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        let created_at = task["created_at"].as_str().unwrap_or("");
        if let Ok(date) = DateTime::parse_from_rfc3339(created_at) {
            let date_utc = date.with_timezone(&Utc);
            let interval_start = match time_unit.as_str() {
                "month" => date_utc
                    .with_day(1)
                    .unwrap()
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
                "week" => {
                    let days_from_monday = date_utc.weekday().num_days_from_monday();
                    date_utc
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap()
                        - Duration::days(days_from_monday as i64)
                }
                _ => {
                    let interval_seconds = (date_utc - start).num_seconds()
                        / interval.num_seconds()
                        * interval.num_seconds();
                    start + Duration::seconds(interval_seconds)
                }
            };
            if let Some(date_counts) = date_status_counts.get_mut(&interval_start) {
                *date_counts.entry(status).or_insert(0) += 1;
            }
        }
    }

    // Convert to ChartDataPoint format
    let mut chart_data: Vec<ChartDataPoint> = date_status_counts
        .into_iter()
        .map(|(date, status_counts)| ChartDataPoint {
            date: format_date(&date, &time_unit),
            status_counts,
        })
        .collect();

    // Sort the chart_data by date
    chart_data.sort_by(|a, b| a.date.cmp(&b.date));

    Json(json!({ "chartData": chart_data })).into_response()
}

fn format_date(date: &DateTime<Utc>, time_unit: &str) -> String {
    match time_unit {
        "minute" => date.format("%Y-%m-%d %H:%M").to_string(),
        "hour" => date.format("%Y-%m-%d %H:00").to_string(),
        "day" => date.format("%Y-%m-%d").to_string(),
        "week" => date.format("%Y-%m-%d").to_string(), // Start of the week
        "month" => date.format("%Y-%m").to_string(),
        _ => date.to_rfc3339(),
    }
}

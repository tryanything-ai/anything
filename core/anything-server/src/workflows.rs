use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::types::workflow_types::WorkflowVersionDefinition;
use crate::AppState;
use uuid::Uuid;

use dotenv::dotenv;
use std::env;

use chrono::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseFlowVersionInput {
    account_id: String,
    flow_id: String,
    flow_definition: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowHandleInput {
    name: Option<String>,
    description: Option<String>,
    flow_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowFromJsonInput {
    flow_id: String,
    name: Option<String>,
    flow_template: Value,
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
        flow_name: payload.name.unwrap_or("New Workflow".to_string()),
        description: payload.description.unwrap_or("".to_string()),
        account_id: account_id.clone(),
    };

    println!("Workflow: {:?}", input);

    let jwt = user.jwt.clone();
    // Create Flow
    let _response = match client
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
        flow_definition: serde_json::json!(WorkflowVersionDefinition::default()),
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
        }
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

pub async fn create_workflow_from_json(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<CreateWorkflowFromJsonInput>,
) -> impl IntoResponse {
    println!("[WORKFLOW FROM JSON] Handling create_workflow_from_json request");

    // Extract and validate required fields
    let name = match payload.name {
        Some(name) if !name.trim().is_empty() => name,
        _ => {
            println!("[WORKFLOW FROM JSON] Name field validation failed - empty or missing name");
            return (StatusCode::BAD_REQUEST, "Name field is required").into_response()
        },
    };

    // Validate the flow template can be parsed
    println!("[WORKFLOW FROM JSON] Attempting to parse flow template");
    let flow_definition: Result<WorkflowVersionDefinition, _> =
        serde_json::from_value(payload.flow_template.clone());

    if let Err(e) = flow_definition {
        println!("[WORKFLOW FROM JSON] Flow template parsing failed: {}", e);
        return (
            StatusCode::BAD_REQUEST,
            format!("Invalid flow template format: {}", e),
        )
            .into_response();
    }
    println!("[WORKFLOW FROM JSON] Flow template successfully parsed");

    let flow_id = payload.flow_id;
    println!("[WORKFLOW FROM JSON] Using flow_id: {}", flow_id);

    let client = &state.anything_client;

    // Create the workflow
    let input = CreateWorkflowInput {
        flow_id: flow_id.clone(),
        flow_name: name.clone(),
        description: "".to_string(),
        account_id: account_id.clone(),
    };
    println!("[WORKFLOW FROM JSON] Creating workflow with name: {}", name);

    let _response = match client
        .from("flows")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[WORKFLOW FROM JSON] Successfully created workflow");
            response
        },
        Err(e) => {
            println!("[WORKFLOW FROM JSON] Failed to create workflow: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create workflow",
            )
                .into_response()
        }
    };

    // Create the flow version with the template
    println!("[WORKFLOW FROM JSON] Creating flow version");
    let version_input = BaseFlowVersionInput {
        account_id: account_id.clone(),
        flow_id: flow_id.clone(),
        flow_definition: payload.flow_template,
    };

    let version_response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&version_input).unwrap())
        .single()
        .execute()
        .await
    {
        Ok(response) => {
            println!("[WORKFLOW FROM JSON] Flow version creation response: {:?}", response);
            response
        }
        Err(e) => {
            println!("[WORKFLOW FROM JSON] Failed to create workflow version: {:?}", e);
            println!("[WORKFLOW FROM JSON] Attempting to cleanup failed workflow");
            // Delete the flow since version creation failed
            let _cleanup = client
                .from("flows")
                .auth(user.jwt.clone())
                .eq("flow_id", &flow_id)
                .eq("account_id", &account_id)
                .delete()
                .execute()
                .await;
            println!("[WORKFLOW FROM JSON] Cleanup completed");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create workflow version",
            )
                .into_response()
        }
    };

    let body = match version_response.json::<serde_json::Value>().await {
        Ok(body) => {
            println!("[WORKFLOW FROM JSON] Successfully parsed version response");
            serde_json::json!({
                "workflow_id": flow_id,
                "workflow_version_id": body["flow_version_id"].as_str().unwrap_or("")
            })
        },
        Err(e) => {
            println!("[WORKFLOW FROM JSON] Failed to parse version response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("[WORKFLOW FROM JSON] Successfully completed workflow creation");
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
    Path((_account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
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
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    println!("Handling publish workflow version");
    println!("account id: {}", account_id);
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

    let _body_2 = match un_publish_response.text().await {
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

    // Update the workflow to be active so it starts running automatically
    let update_workflow_json = serde_json::json!({
        "active": true
    });

    match client
        .from("flows")
        .auth(user.jwt.clone())
        .eq("flow_id", &workflow_id)
        .update(update_workflow_json.to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!("Workflow update response: {:?}", response);
            response
        }
        Err(err) => {
            eprintln!("Error updating workflow: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update workflow",
            )
                .into_response();
        }
    };
    // Signal the trigger processing loop that it needs to hydrate and manage new triggers.
    if let Err(err) = state.trigger_engine_signal.send(workflow_id) {
        println!("Failed to send trigger signal: {:?}", err);
    }

    Json(body).into_response()
}

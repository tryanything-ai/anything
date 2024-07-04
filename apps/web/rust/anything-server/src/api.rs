use axum::{
    extract::{Path, State, Extension},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use hyper::header::AUTHORIZATION;
use postgrest::Postgrest;

use crate::auth::User;

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
    State(client): State<Arc<Postgrest>>, 
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Handling a get_workflows");

    let response = match client
        .from("flows")
        .auth(user.jwt)
        .eq("archived", "false")
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

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}

pub async fn get_workflow(
    Path(flow_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {

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
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {

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
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Json(payload): Json<CreateWorkflowHandleInput>,
) -> impl IntoResponse {

    println!("Handling a create_workflow");

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

    let versionInput = BaseFlowVersionInput {
        account_id: user.account_id.clone(),
        flow_id: payload.flow_id.clone(),
        flow_version: "0.0.1".to_string(),
        flow_definition: serde_json::json!({
            "actions": [],
        })
    };

    let clonedUser = user.clone();
    
    //Create Flow Version
    let version_response = match client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&versionInput).unwrap())
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
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {

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
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Json(payload): Json<UpdateWorkflowInput>,  
) -> impl IntoResponse {
    
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
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Json(payload): Json<Value>,  
) -> impl IntoResponse {

    // let payload_json = serde_json::to_string(&payload).unwrap();

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
    State(client): State<Arc<Postgrest>>, 
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Handling a get_actions");

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
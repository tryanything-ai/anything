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


use slugify::slugify;

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

    // let clonedUser = user.clone();
    
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


// Secrets
pub async fn get_secrets(
    State(client): State<Arc<Postgrest>>, 
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Handling a get_actions");

    let response = match client
        .from("secrets")
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


pub async fn delete_secret(
    Path(secret_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {

    let response = match client
        .from("secrets")
        .auth(user.jwt)
        .eq("secret_id", &secret_id)
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



#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretPayload {
    secret_name: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretInput {
    name: String,
    secret: String,
    // description: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AnythingCreateSecretInput {
    secret_name: String,
    vault_secret_id: String,
    secret_description: String,
    account_id: String,
}

pub async fn create_secret(
    State(client): State<Arc<Postgrest>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Json(payload): Json<CreateSecretPayload>,
) -> impl IntoResponse {

    println!("create_secret Input?: {:?}", payload);

    let vault_secret_name = slugify!(format!("{}_{}", user.account_id.clone(), payload.secret_name.clone()).as_str(), separator = "_");

    println!("New Name: {}", vault_secret_name);

    let input = CreateSecretInput {
        name: vault_secret_name,
        secret: payload.secret_value.clone(),
        // description: payload.secret_description.clone(),
    }; 

    println!("insert_secret rpc Input?: {:?}", input);

    let jwt = user.jwt.clone();

    // Create Secret in Vault
    let response = match client
    // .insert_header("apikey", supabase_api_key.clone())
    // .rpc("insert_secret",)
    .rpc("insert_secret", serde_json::to_string(&input).unwrap())
        // .auth(jwt)
        // .insert(serde_json::to_string(&input).unwrap())
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

       // Extract the id from the response
    // Extract the id from the response
    // let vault_secret_id = match response.get("id").and_then(|id| id.as_str()) {
    //     Some(id) => id.to_string(),
    //     None => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get id from response").into_response(),
    // };

    // println!("Response from vault insert: {:?}", vault_secret_id);

    println!("Response from vault insert: {:?}", body);

    let cleaned_response = body.trim_matches('"');

    let anythingSecretInput = AnythingCreateSecretInput {
        secret_name: payload.secret_name.clone(),
        vault_secret_id: cleaned_response.to_string(), //vault_secret_id.clone(),
        secret_description: payload.secret_description.clone(),
        account_id: user.account_id.clone()
    };
    
    //Create Flow Version
    let db_secret_response = match client
        .from("secrets")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&anythingSecretInput).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    let db_secret_body = match db_secret_response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    println!("DB Secret Body: {:?}", db_secret_body);

    Json(db_secret_body).into_response()
}
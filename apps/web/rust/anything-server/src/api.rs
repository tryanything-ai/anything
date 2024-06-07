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
pub struct Workflow {
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

    println!("User in handler: {:?}", user);

    let response = match client
        .from("flows")
        .auth(user.jwt)
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
    headers: HeaderMap,
) -> impl IntoResponse {

    let response = match client
        .from("flows")
        // .auth(jwt)
        .eq("id", &flow_id)
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
    headers: HeaderMap,
) -> impl IntoResponse {
    

    let response = match client
        .from("flow_versions")
        // .auth(jwt)
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

//TODO: Validate against Schema. etc
pub async fn create_workflow(
    Path(flow_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    headers: HeaderMap,
    Json(payload): Json<Workflow>,
) -> impl IntoResponse {

    println!("Create Workflow in Rust");

    let jwt = match headers.get(AUTHORIZATION).and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    println!("JWT: {}", jwt);
    //TODO: get accound id from JWT

    let workflow = Workflow {
        ..payload
    };

    let response = match client
        .from("flows")
        // .auth(jwt)
        .insert(serde_json::to_string(&workflow).unwrap())
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

//TODO: change this to some sort of soft delete or archive
pub async fn delete_workflow(
    Path(flow_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    

    let response = match client
        .from("flows")
        // .auth(jwt)
        .eq("id", &flow_id)
        .delete()
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
    headers: HeaderMap,
    Json(payload): Json<Workflow>,  
) -> impl IntoResponse {
    

    let workflow = Workflow {
        flow_id: flow_id.clone(),
        ..payload
    };

    let response = match client
        .from("flows")
        // .auth(jwt)
        .eq("id", &flow_id)
        .update(serde_json::to_string(&workflow).unwrap())
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

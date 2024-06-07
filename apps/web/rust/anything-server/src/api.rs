use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use hyper::header::AUTHORIZATION;
use postgrest::Postgrest;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_workflows(State(client): State<Arc<Postgrest>>, headers: HeaderMap) -> impl IntoResponse {
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flows")
        .auth(jwt)
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
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flows")
        .auth(jwt)
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
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flow_versions")
        .auth(jwt)
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

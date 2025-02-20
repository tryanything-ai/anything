use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

pub async fn get_agent(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("agents")
        .auth(user.jwt)
        .eq("agent_id", &agent_id)
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

pub async fn get_agents(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("agents")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .select("*, agent_communication_channels(*)")
        .eq("archived", "false")
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

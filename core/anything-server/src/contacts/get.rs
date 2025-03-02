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

pub async fn get_contacts(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_contacts");

    let client = &state.anything_client;

    let response = match client
        .from("contacts")
        .auth(&user.jwt)
        .select("*")
        .eq("account_id", &account_id)
        .eq("archived", "false")
        .order("created_at.desc")
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

pub async fn get_contact(
    Path((account_id, contact_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_contact");

    let client = &state.anything_client;

    let response = match client
        .from("contacts")
        .auth(&user.jwt)
        .select("*")
        .eq("contact_id", &contact_id)
        .eq("account_id", &account_id)
        .eq("archived", "false")
        .single()
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
        return (StatusCode::NOT_FOUND, "Contact not found").into_response();
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(item).into_response()
}

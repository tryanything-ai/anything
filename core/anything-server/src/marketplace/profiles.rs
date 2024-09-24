use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::Value;
use std::sync::Arc;

use crate::AppState;

// Profiles
pub async fn get_profiles_from_marketplace(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let client = &state.marketplace_client;

    println!("[PROFILES] Fetching profiles");

    let response = match client
        .from("profiles")
        .select("*")
        .order("username.asc")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[PROFILES] Failed to execute request: {:?}", e);
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
            println!("[PROFILES] Failed to read response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(e) => {
            println!("[PROFILES] Failed to parse JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    println!("[PROFILES] Query result: {:?}", items);

    Json(items).into_response()
}

pub async fn get_marketplace_profile_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let client = &state.marketplace_client;

    println!("[MARKETPLACE] Fetching profile by slug: {}", username);

    let response = match client
        .from("profiles")
        .select("*")
        .eq("username", &username)
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[MARKETPLACE] Failed to execute request: {:?}", e);
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
            println!("[MARKETPLACE] Failed to read response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(e) => {
            println!("[MARKETPLACE] Failed to parse JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    if let Some(profile) = items.as_array().and_then(|arr| arr.first()) {
        println!("[MARKETPLACE] Found profile: {:?}", profile);
        Json(profile.clone()).into_response()
    } else {
        println!("[MARKETPLACE] No profile found for slug: {}", username);
        (StatusCode::NOT_FOUND, "Profile not found").into_response()
    }
}

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::Value;
use std::sync::Arc;

use crate::AppState;

// Profiles - placeholder implementation (use profiles_seaorm.rs for SeaORM version)
pub async fn get_profiles_from_marketplace(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[PROFILES] Fetching profiles (placeholder - migrating to SeaORM)");

    let placeholder_response = serde_json::json!({
        "message": "Marketplace profiles endpoint migrating to SeaORM - use profiles_seaorm.rs",
        "status": "placeholder",
        "data": []
    });

    Json(placeholder_response).into_response()
}

pub async fn get_marketplace_profile_by_username(
    State(_state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    println!("[MARKETPLACE] Fetching profile by username: {} (placeholder - migrating to SeaORM)", username);

    let placeholder_response = serde_json::json!({
        "message": "Marketplace profile lookup migrating to SeaORM - use profiles_seaorm.rs",
        "username": username,
        "status": "placeholder"
    });

    Json(placeholder_response).into_response()
}

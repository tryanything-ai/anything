use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::{Value, json};
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Order};
use uuid::Uuid;

use crate::AppState;
use crate::entities; // Assuming we have a profiles entity

// Note: We need to create a profiles entity first
// For now, I'll create a placeholder structure

// Profiles
pub async fn get_profiles_from_marketplace(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[PROFILES SEAORM] Fetching profiles");

    // TODO: Create profiles entity in entities/profiles.rs
    // For now, return empty array as placeholder
    println!("[PROFILES SEAORM] TODO: Implement profiles entity and SeaORM query");
    
    let placeholder_profiles: Vec<Value> = vec![
        json!({
            "message": "Profiles entity not yet implemented in SeaORM",
            "status": "placeholder"
        })
    ];

    Json(placeholder_profiles).into_response()
}

pub async fn get_marketplace_profile_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    println!("[MARKETPLACE SEAORM] Fetching profile by username: {}", username);

    // TODO: Create profiles entity in entities/profiles.rs
    // For now, return placeholder response
    println!("[MARKETPLACE SEAORM] TODO: Implement profiles entity and SeaORM query");
    
    let placeholder_profile = json!({
        "message": "Profile lookup not yet implemented in SeaORM",
        "username": username,
        "status": "placeholder"
    });

    Json(placeholder_profile).into_response()
}

// TODO: Once profiles entity is created, implement these functions:
/*
pub async fn get_profiles_from_marketplace_impl(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[PROFILES SEAORM] Fetching profiles");

    let profiles = match profiles::Entity::find()
        .order_by(profiles::Column::Username, Order::Asc)
        .all(&*state.db)
        .await
    {
        Ok(profiles) => profiles,
        Err(err) => {
            println!("[PROFILES SEAORM] Database error: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            )
                .into_response();
        }
    };

    // Convert to JSON format expected by frontend
    let profiles_json: Vec<Value> = profiles
        .into_iter()
        .map(|profile| {
            json!({
                "id": profile.id,
                "username": profile.username,
                "display_name": profile.display_name,
                "bio": profile.bio,
                "avatar_url": profile.avatar_url,
                "website": profile.website,
                "created_at": profile.created_at,
                "updated_at": profile.updated_at
            })
        })
        .collect();

    println!("[PROFILES SEAORM] Successfully retrieved {} profiles", profiles_json.len());
    Json(profiles_json).into_response()
}

pub async fn get_marketplace_profile_by_username_impl(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    println!("[MARKETPLACE SEAORM] Fetching profile by username: {}", username);

    let profile = match profiles::Entity::find()
        .filter(profiles::Column::Username.eq(username.clone()))
        .one(&*state.db)
        .await
    {
        Ok(Some(profile)) => profile,
        Ok(None) => {
            println!("[MARKETPLACE SEAORM] No profile found for username: {}", username);
            return (StatusCode::NOT_FOUND, "Profile not found").into_response();
        }
        Err(err) => {
            println!("[MARKETPLACE SEAORM] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let profile_json = json!({
        "id": profile.id,
        "username": profile.username,
        "display_name": profile.display_name,
        "bio": profile.bio,
        "avatar_url": profile.avatar_url,
        "website": profile.website,
        "created_at": profile.created_at,
        "updated_at": profile.updated_at
    });

    println!("[MARKETPLACE SEAORM] Found profile: {}", profile.username);
    Json(profile_json).into_response()
}
*/

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::custom_auth::User;
use crate::entities::{accounts, user_accounts};
use crate::AppState;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};

// Get auth accounts using SeaORM
pub async fn get_auth_accounts(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!("Handling get_auth_accounts for account_id: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // TODO: Implement auth provider accounts query with SeaORM
    // This would typically involve joining accounts with auth_provider_accounts
    // For now, returning placeholder data
    let auth_accounts = json!({
        "message": "get_auth_accounts not fully implemented with SeaORM",
        "account_id": account_id,
        "auth_accounts": [],
        "status": "placeholder"
    });

    Json(auth_accounts).into_response()
}

// Get auth accounts for provider using SeaORM
pub async fn get_auth_accounts_for_provider_name(
    Path((account_id, provider_name)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "Handling get_auth_accounts_for_provider_name for account: {}, provider: {}",
        account_id, provider_name
    );

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // TODO: Implement provider-specific auth accounts query with SeaORM
    let auth_accounts = json!({
        "message": "get_auth_accounts_for_provider_name not fully implemented with SeaORM",
        "account_id": account_id,
        "provider_name": provider_name,
        "auth_accounts": [],
        "status": "placeholder"
    });

    Json(auth_accounts).into_response()
}

// Get account by slug using SeaORM
pub async fn get_account_by_slug(
    Path((account_id, slug)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_account_by_slug for slug: {}", slug);

    // Find account by slug
    let account = match accounts::Entity::find()
        .filter(accounts::Column::Slug.eq(&slug))
        .filter(accounts::Column::Active.eq(true))
        .one(&*state.db)
        .await
    {
        Ok(Some(account)) => account,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Account not found").into_response();
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let response = json!({
        "account_id": account.account_id,
        "account_name": account.account_name,
        "slug": account.slug,
        "active": account.active,
        "created_at": account.created_at,
        "updated_at": account.updated_at
    });

    Json(response).into_response()
}

// Get account invitations using SeaORM
pub async fn get_account_invitations(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_account_invitations for account: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // TODO: Implement invitations query with SeaORM
    // This would involve an invitations entity
    let invitations = json!({
        "message": "get_account_invitations not fully implemented with SeaORM",
        "account_id": account_id,
        "invitations": [],
        "status": "placeholder"
    });

    Json(invitations).into_response()
}

// Get account members using SeaORM
pub async fn get_account_members(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_account_members for account: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Get all user_accounts for this account
    let user_accounts_list = match user_accounts::Entity::find()
        .filter(user_accounts::Column::AccountId.eq(account_uuid))
        .filter(user_accounts::Column::Active.eq(true))
        .all(&*state.db)
        .await
    {
        Ok(members) => members,
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Convert to response format
    let members: Vec<Value> = user_accounts_list
        .into_iter()
        .map(|ua| json!({
            "user_id": ua.user_id,
            "account_id": ua.account_id,
            "role": ua.role,
            "active": ua.active,
            "created_at": ua.created_at,
            "updated_at": ua.updated_at
        }))
        .collect();

    let response = json!({
        "account_id": account_id,
        "members": members,
        "member_count": members.len()
    });

    Json(response).into_response()
}

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::custom_auth::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretPayload {
    secret_name: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnythingApiKeyPayload {
    secret_name: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSecretPayload {
    secret_id: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecretByValueResponse {
    pub secret_id: String,
    pub account_id: String,
    pub secret_name: String,
    pub vault_secret_id: String,
    pub secret_description: String,
    pub anything_api_key: bool,
    pub updated_at: String,
    pub created_at: String,
    pub updated_by: String,
    pub created_by: String,
}

/// Create a regular secret using pgsodium encryption
pub async fn create_secret(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
    Json(payload): Json<CreateSecretPayload>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] Redirecting create_secret to pgsodium_secrets");
    
    // Convert the payload to match pgsodium expectations
    let pgsodium_payload = crate::pgsodium_secrets::handlers::CreateSecretRequest {
        secret_name: payload.secret_name,
        secret_value: payload.secret_value,
        description: Some(payload.secret_description),
        is_api_key: Some(false),
    };

    // Redirect to pgsodium implementation - extract Claims from Extension
    let claims = crate::custom_auth::jwt::Claims {
        sub: user.account_id.clone(),
        username: user.username.clone(),
        session_id: uuid::Uuid::new_v4().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let auth_claims = crate::custom_auth::extractors::AuthClaims(claims);
    
    crate::pgsodium_secrets::handlers::create_secret(
        State(state),
        Path(account_id),
        auth_claims,
        Json(pgsodium_payload),
    ).await
}

/// Create an API key using pgsodium encryption
pub async fn create_anything_api_key(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
    Json(payload): Json<CreateAnythingApiKeyPayload>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] Redirecting create_anything_api_key to pgsodium_secrets");

    // Generate a unique API key with a prefix for easy identification
    let api_key = format!("any_{}", uuid::Uuid::new_v4());
    
    // Convert the payload to match pgsodium expectations
    let pgsodium_payload = crate::pgsodium_secrets::handlers::CreateSecretRequest {
        secret_name: format!("api_key_{}", payload.secret_name),
        secret_value: api_key,
        description: Some(payload.secret_description),
        is_api_key: Some(true),
    };

    // Redirect to pgsodium implementation - extract Claims from Extension
    let claims = crate::custom_auth::jwt::Claims {
        sub: user.account_id.clone(),
        username: user.username.clone(),
        session_id: uuid::Uuid::new_v4().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let auth_claims = crate::custom_auth::extractors::AuthClaims(claims);
    
    crate::pgsodium_secrets::handlers::create_secret(
        State(state),
        Path(account_id),
        auth_claims,
        Json(pgsodium_payload),
    ).await
}

/// Get decrypted secrets using pgsodium
pub async fn get_decrypted_secrets(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] Redirecting get_decrypted_secrets to pgsodium_secrets");
    
    // Use dummy user for this public endpoint
    let dummy_user = User {
        id: uuid::Uuid::new_v4(),
        email: "system@localhost".to_string(),
        username: "system".to_string(),
        account_id: account_id.clone(),
        jwt: "system".to_string(),
    };

    // Redirect to pgsodium implementation - create AuthClaims from dummy user
    let claims = crate::custom_auth::jwt::Claims {
        sub: dummy_user.account_id.clone(),
        username: dummy_user.username.clone(),
        session_id: uuid::Uuid::new_v4().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let auth_claims = crate::custom_auth::extractors::AuthClaims(claims);
    
    crate::pgsodium_secrets::handlers::get_secrets(
        State(state),
        Path(account_id),
        auth_claims,
    ).await
}

/// Get decrypted API keys using pgsodium
pub async fn get_decrypted_anything_api_keys(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] get_decrypted_anything_api_keys using pgsodium_secrets");
    
    // TODO: Implement API key filtering in pgsodium_secrets
    // For now, return all secrets and let the frontend filter
    
    // Use dummy user for this public endpoint
    let dummy_user = User {
        id: uuid::Uuid::new_v4(),
        email: "system@localhost".to_string(),
        username: "system".to_string(),
        account_id: account_id.clone(),
        jwt: "system".to_string(),
    };

    // Redirect to pgsodium implementation - create AuthClaims from dummy user
    let claims = crate::custom_auth::jwt::Claims {
        sub: dummy_user.account_id.clone(),
        username: dummy_user.username.clone(),
        session_id: uuid::Uuid::new_v4().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let auth_claims = crate::custom_auth::extractors::AuthClaims(claims);
    
    crate::pgsodium_secrets::handlers::get_secrets(
        State(state),
        Path(account_id),
        auth_claims,
    ).await
}

/// Delete a secret using pgsodium
pub async fn delete_secret(
    Path((account_id, secret_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] Redirecting delete_secret to pgsodium_secrets");

    // Invalidate the bundler secrets cache for this account after deleting a secret
    if let Some(cache_entry) = state.bundler_secrets_cache.get(&account_id) {
        cache_entry.invalidate(&account_id);
    }

    // Redirect to pgsodium implementation - extract Claims from Extension
    let claims = crate::custom_auth::jwt::Claims {
        sub: user.account_id.clone(),
        username: user.username.clone(),
        session_id: uuid::Uuid::new_v4().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let auth_claims = crate::custom_auth::extractors::AuthClaims(claims);
    
    crate::pgsodium_secrets::handlers::delete_secret(
        State(state),
        Path((account_id, secret_id)),
        auth_claims,
    ).await
}

/// Delete an API key using pgsodium
pub async fn delete_api_key(
    Path((account_id, secret_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] Redirecting delete_api_key to pgsodium_secrets");

    // TODO: Implement specific API key cache removal for pgsodium
    // For now, just delegate to the regular delete operation
    delete_secret(
        Path((account_id, secret_id)),
        State(state),
        Extension(user),
    ).await
}

/// Get a secret by its value using pgsodium
pub async fn get_secret_by_secret_value(
    state: Arc<AppState>,
    secret_value: String,
) -> Result<SecretByValueResponse, StatusCode> {
    println!("[SECRETS SEAORM] get_secret_by_secret_value using pgsodium_secrets");
    
    // TODO: Implement get_secret_by_secret_value in pgsodium_secrets module
    // This would require searching through encrypted secrets and decrypting them to find a match
    // For now, return an error to indicate this functionality needs implementation
    
    println!("[SECRETS SEAORM] get_secret_by_secret_value not yet implemented for pgsodium");
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Update a secret using pgsodium
pub async fn update_secret(
    Path((account_id, secret_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateSecretPayload>,
) -> impl IntoResponse {
    println!("[SECRETS SEAORM] Redirecting update_secret to pgsodium_secrets");

    // Convert the payload to match pgsodium expectations
    let pgsodium_payload = crate::pgsodium_secrets::handlers::UpdateSecretRequest {
        secret_name: None,
        secret_value: Some(payload.secret_value),
        description: Some(payload.secret_description),
    };

    // Redirect to pgsodium implementation - extract Claims from Extension
    let claims = crate::custom_auth::jwt::Claims {
        sub: user.account_id.clone(),
        username: user.username.clone(),
        session_id: uuid::Uuid::new_v4().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let auth_claims = crate::custom_auth::extractors::AuthClaims(claims);
    
    crate::pgsodium_secrets::handlers::update_secret(
        State(state),
        Path((account_id, secret_id)),
        auth_claims,
        Json(pgsodium_payload),
    ).await
}
use crate::pgsodium_secrets::handlers::create_secret;
use crate::AppState;
use crate::entities::{auth_providers, accounts};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use serde_json::Value;

use chrono::{DateTime, Utc};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use slugify::slugify;
use std::sync::Arc;
use urlencoding;
use uuid::Uuid;

// Legacy types for compatibility
#[derive(Debug, Clone)]
pub struct AuthState {
    pub code_verifier: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthProvider {
    pub auth_provider_id: String,
    pub provider_name: String,
    pub provider_label: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub scopes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

use crate::auth::utils::{
    generate_code_challenge, generate_code_verifier, generate_random_string,
    generate_unique_account_slug,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountAuthProviderAccount {
    pub account_auth_provider_account_id: Uuid,
    pub account_id: Uuid,
    pub auth_provider_id: String,
    pub auth_provider: Option<Value>,
    pub account_auth_provider_account_label: String,
    pub account_auth_provider_account_slug: String,
    pub account_data: Option<Value>,
    pub access_token: String,
    pub access_token_vault_id: String,
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
    pub refresh_token_vault_id: String,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub failed_at: Option<DateTime<Utc>>,
    pub failed: bool,
    pub failed_reason: Option<String>,
    pub failure_retries: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthInitParams {
    pub account_id: String,
    pub provider_account_label: Option<String>,
}

// OAuth initialization endpoint
pub async fn init_oauth(
    Path(provider_name): Path<String>,
    Query(params): Query<OAuthInitParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[AUTH INIT SEAORM] Initializing OAuth for provider: {}", provider_name);

    // Get auth provider from database using SeaORM
    let auth_provider = match auth_providers::Entity::find()
        .filter(auth_providers::Column::ProviderName.eq(&provider_name))
        .one(&*state.db)
        .await
    {
        Ok(Some(provider)) => provider,
        Ok(None) => {
            println!("[AUTH INIT SEAORM] Provider not found: {}", provider_name);
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
        Err(err) => {
            println!("[AUTH INIT SEAORM] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Validate account exists
    let account_uuid = match Uuid::parse_str(&params.account_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response();
        }
    };

    let account_exists = match accounts::Entity::find()
        .filter(accounts::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(err) => {
            println!("[AUTH INIT SEAORM] Error checking account: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    if !account_exists {
        return (StatusCode::NOT_FOUND, "Account not found").into_response();
    }

    // Generate OAuth parameters
    let state_param = generate_random_string(32);
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    // Build OAuth authorization URL
    let auth_url = auth_provider.auth_url.unwrap_or_default();
    let scopes = auth_provider.scopes.unwrap_or_default();
    
    // TODO: Get client_id from vault using auth_provider.client_id_vault_id
    let client_id = "placeholder_client_id"; // Would need to decrypt from vault
    
    let redirect_uri = format!("{}/auth/oauth/callback/{}", 
        std::env::var("APP_URL").unwrap_or_default(), 
        provider_name
    );

    let oauth_url = format!(
        "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&state={}&code_challenge={}&code_challenge_method=S256",
        auth_url,
        urlencoding::encode(client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&scopes),
        urlencoding::encode(&state_param),
        urlencoding::encode(&code_challenge)
    );

    // TODO: Store OAuth state and code_verifier temporarily for callback validation
    // This would typically go in a temporary store (Redis, database temp table, etc.)

    println!("[AUTH INIT SEAORM] Generated OAuth URL for provider: {}", provider_name);

    // Return redirect response
    Html(format!(
        r#"
        <html>
            <head><title>OAuth Authorization</title></head>
            <body>
                <h1>Redirecting to {}</h1>
                <p>If you are not redirected automatically, <a href="{}">click here</a>.</p>
                <script>window.location.href = "{}";</script>
            </body>
        </html>
        "#,
        auth_provider.provider_label.unwrap_or(provider_name.clone()),
        oauth_url,
        oauth_url
    )).into_response()
}

// OAuth callback endpoint
pub async fn oauth_callback(
    Path(provider_name): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[AUTH INIT SEAORM] OAuth callback for provider: {}", provider_name);

    // Extract authorization code and state from callback
    let code = match params.get("code") {
        Some(code) => code,
        None => {
            println!("[AUTH INIT SEAORM] No authorization code in callback");
            return (StatusCode::BAD_REQUEST, "Missing authorization code").into_response();
        }
    };

    let state_param = params.get("state");
    
    // TODO: Validate state parameter against stored value
    // TODO: Exchange authorization code for access token
    // TODO: Store tokens in vault and create account_auth_provider_account record

    println!("[AUTH INIT SEAORM] OAuth callback processed for provider: {}", provider_name);
    
    Html(format!(
        r#"
        <html>
            <head><title>OAuth Success</title></head>
            <body>
                <h1>Authorization Successful</h1>
                <p>You have successfully authorized with {}.</p>
                <p>Authorization code: {}</p>
                <script>window.close();</script>
            </body>
        </html>
        "#,
        provider_name,
        code
    )).into_response()
}

// Helper function to get decrypted auth provider (simplified)
async fn get_decrypted_auth_provider_by_name(
    state: &Arc<AppState>,
    provider_name: &str,
) -> Result<auth_providers::Model, Box<dyn std::error::Error + Send + Sync>> {
    let provider = auth_providers::Entity::find()
        .filter(auth_providers::Column::ProviderName.eq(provider_name))
        .one(&*state.db)
        .await?
        .ok_or("Provider not found")?;

    // TODO: Decrypt client_id and client_secret from vault using:
    // - provider.client_id_vault_id
    // - provider.client_secret_vault_id

    Ok(provider)
}

// Simple health check for auth providers
pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[AUTH INIT SEAORM] Health check");
    
    // Count available auth providers
    let provider_count = match auth_providers::Entity::find()
        .count(&*state.db)
        .await
    {
        Ok(count) => count,
        Err(err) => {
            println!("[AUTH INIT SEAORM] Error counting providers: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    Json(json!({
        "status": "healthy",
        "auth_providers_count": provider_count,
        "timestamp": Utc::now()
    })).into_response()
}

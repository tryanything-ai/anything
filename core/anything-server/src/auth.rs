use crate::supabase_auth_middleware::User;
use crate::AppState;
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProvider {
    auth_provider_id: String,
    provider_name: String,
    provider_label: String,
    provider_icon: String,
    provider_description: String,
    provider_readme: String,
    auth_type: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
    client_id: String,
    client_secret: String,
    scopes: String,
    public: bool,
    updated_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_by: Option<Uuid>,
    created_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct InitiateAuthFlow {
    redirect_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountAuthProviderAccount {
    account_id: String,
    auth_provider_id: String,
    account_auth_provider_account_label: String,
    account_auth_provider_account_slug: String,
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

pub async fn handle_provider_callback(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Query(params): Query<OAuthCallbackParams>,
) -> impl IntoResponse {
    println!("Handling auth callback for provider: {:?}", provider_name);
    println!("Params: {:?}", params);

    let client = &state.anything_client;

    // Get Provider details
    let response = match client
        .from("auth_providers")
        .auth(user.jwt.clone())
        .eq("provider_name", &provider_name)
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

    println!("Response: {:?}", response);

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

    let auth_provider: AuthProvider = match serde_json::from_str(&body) {
        Ok(auth_provider) => auth_provider,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    // Verify state from the database
    // TODO: Implement state verification

    // Exchange code for token
    // let code_verifier = "your_code_verifier"; // You need to retrieve this value appropriately
    let token = match exchange_code_for_token(&auth_provider, &params.code).await {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to exchange code for token",
            )
                .into_response()
        }
    };

    println!("Token: {:?}", token);

    let input = CreateAccountAuthProviderAccount {
        account_id: user.account_id.clone(),
        auth_provider_id: auth_provider.auth_provider_id.clone(),
        account_auth_provider_account_label: auth_provider.provider_label.clone(), //TODO: update this to the real thing
        account_auth_provider_account_slug: auth_provider.provider_name.clone(), //TODO: update this to the real thing
        access_token: token.access_token.clone(),
        refresh_token: token.refresh_token.unwrap_or_default(),
        expires_at: token.expires_at.unwrap_or_default(),
    };

    // Store token in the database
    // TODO: Implement token storage and account creation
    let create_account_response = match client
        .from("account_auth_provider_accounts")
        .auth(user.jwt)
        .insert(serde_json::to_string(&input).unwrap())
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

    println!("Create Account Response: {:?}", create_account_response);
    // Return success response
    (
        StatusCode::OK,
        Json(serde_json::json!({"status": "success"})),
    )
        .into_response()
}

pub async fn exchange_code_for_token(
    provider: &AuthProvider,
    code: &str,
) -> Result<OAuthToken, StatusCode> {
    let client = Client::new();

    let response = client
        .post(&provider.token_url)
        .form(&[
            ("client_id", &provider.client_id),
            ("client_secret", &provider.client_secret),
            ("code", &code.to_string()),
            ("grant_type", &"authorization_code".to_string()),
        ])
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    response
        .json::<OAuthToken>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

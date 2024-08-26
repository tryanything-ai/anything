use crate::AppState;
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::supabase_auth_middleware::User;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, Utc};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use urlencoding;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub state: String,
    pub account_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
struct OAuthResponse {
    url: String,
}

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
    redirect_url: String, //TODO: add once we have it in the db
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
    pub code: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

pub async fn handle_provider_callback(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<OAuthCallbackParams>,
) -> impl IntoResponse {
    println!("Handling auth callback for provider: {:?}", provider_name);
    println!("Params: {:?}", params);

    //TODO: Implement state verification
    let client = &state.anything_client;

    // Get Provider details
    let response = match client
        .from("auth_providers")
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
    let token = match exchange_code_for_token(&auth_provider, &params.code.unwrap()).await {
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
        account_id: "".to_string(), //TODO: update this to the real thing
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
        // .auth(user.jwt)
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

pub async fn initiate_auth(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let auth_states = &state.auth_states;
    // Generate a unique state parameter
    let state_string = generate_random_string(32);

    // Replace with actual user ID or relevant data
    let account_id = user.account_id.clone();

    let auth_state = AuthState {
        state: state_string.clone(),
        account_id: account_id.clone(),
        created_at: Utc::now(),
    };

    println!("Auth State: {:?}", auth_state);

    // Store the state in memory
    let mut auth_states_lock = auth_states.write().await;
    auth_states_lock.insert(state_string.clone(), auth_state);

    //TODO: Implement state verification
    let client = &state.anything_client;

    // Get Provider details
    let response = match client
        .from("auth_providers")
        .auth(&user.jwt)
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
                "Failed to find that provider",
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

    println!("Body: {:?}", body);

    let auth_provider: AuthProvider = match serde_json::from_str(&body) {
        Ok(auth_provider) => auth_provider,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    println!("AuthProvider: {:?}", auth_provider);

    // Build the OAuth URL
    let client_id = auth_provider.client_id.clone();
    let redirect_uri = auth_provider.redirect_url.clone();
    let auth_url = auth_provider.auth_url.clone();
    let scope = "data.records:read data.records:write"; //TODO: Replace with your actual scopes
    let code_challenge = generate_code_challenge(&state_string).await; // Assuming you have a function to generate code challenge

    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        auth_url,
        client_id,
        urlencoding::encode(redirect_uri.as_str()),
        urlencoding::encode(scope),
        urlencoding::encode(&state_string),
        urlencoding::encode(&code_challenge)
    );

    println!("Auth URL: {}", auth_url);

    Json(OAuthResponse { url: auth_url }).into_response()
}

async fn generate_code_challenge(code_verifier: &str) -> String {
    let hash = Sha256::digest(code_verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(&hash) // Update this line
}

// Helper function to generate a random string
fn generate_random_string(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            charset
                .chars()
                .nth(rng.gen_range(0..charset.len()))
                .unwrap()
        })
        .collect()
}

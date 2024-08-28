use crate::AppState;
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use crate::supabase_auth_middleware::User;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use rand::Rng;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::sync::Arc;
use urlencoding;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub state: String,
    pub code_verifier: String,
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

    let client = &state.anything_client;
    let auth_states = &state.auth_states;

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
    // Retrieve the stored AuthState using the received state
    let auth_state = auth_states
        .read()
        .await
        .get(&params.state.unwrap())
        .cloned();

    let auth_state = match auth_state {
        Some(state) => state,
        None => return (StatusCode::BAD_REQUEST, "Invalid state").into_response(),
    };

    // Exchange code for token
    // Use the stored code_verifier in the token exchange
    let token = match exchange_code_for_token(
        &auth_provider,
        &params.code.as_deref().unwrap_or(""),
        &auth_provider.redirect_url,
        &auth_state.code_verifier, // Use the stored code_verifier here
    )
    .await
    {
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
        account_id: auth_state.account_id.clone(),
        auth_provider_id: auth_provider.auth_provider_id.clone(),
        account_auth_provider_account_label: auth_provider.provider_label.clone(), //TODO: update this to the real thing
        account_auth_provider_account_slug: auth_provider.provider_name.clone(), //TODO: update this to the real thing
        access_token: token.access_token.clone(),
        refresh_token: token.refresh_token.unwrap_or_default(),
        expires_at: token.expires_at.unwrap_or_default(),
    };

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Store token in the database
    let create_account_response = match client
        .from("account_auth_provider_accounts")
        .auth(supabase_service_role_api_key.clone())
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
    if create_account_response.status().is_success() {
        // Successful response
        let html = r#"
        <html>
        <body>
            <script>
            if (window.opener) {
                window.opener.postMessage('auth_success', '*');
                window.close();
            } else {
                document.body.innerHTML = 'Authentication successful. You can close this window.';
            }
            </script>
        </body>
        </html>
        "#;

        Html(html).into_response()
    } else {
        // Error response
        let html = r#"
        <html>
        <body>
            <h1>Authentication Failed</h1>
            <p>There was an error during authentication. Please try again.</p>
            <script>
            if (window.opener) {
                window.opener.postMessage('auth_failed', '*');
            }
            </script>
        </body>
        </html>
        "#;

        (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
    }
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
    error_description: String,
}

pub async fn exchange_code_for_token(
    provider: &AuthProvider,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> Result<OAuthToken, (StatusCode, String)> {
    let client = Client::new();

    let request = client
        .post(provider.token_url.clone())
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");

    // // Add Authorization header if client_secret is present
    // if let Some(client_secret) = &provider.client_secret {
    //     let credentials = format!("{}:{}", provider.client_id, client_secret);
    //     let encoded_credentials = URL_SAFE_NO_PAD.encode(credentials);
    //     request = request.header(
    //         header::AUTHORIZATION,
    //         format!("Basic {}", encoded_credentials),
    //     );
    // }

    let form_params = [
        ("code", code),
        ("client_id", &provider.client_id),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
        ("code_verifier", code_verifier),
    ];

    println!("token exchange form_params: {:?}", form_params);

    let response = request
        .form(&form_params)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status = response.status();
    println!("Response status: {:?}", status);

    let body = response.text().await.map_err(|e| {
        println!("Error reading response body: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    println!("Response body: {:?}", body);

    if status.is_success() {
        serde_json::from_str::<OAuthToken>(&body).map_err(|e| {
            println!("Failed to parse token response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse token response: {}", e),
            )
        })
    } else {
        let error: ErrorResponse = serde_json::from_str(&body).map_err(|e| {
            println!("Failed to parse error response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse error response: {}", e),
            )
        })?;
        // println!("Parsed error response: {:?}", error);

        let status_code = if error.error == "invalid_client" {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::BAD_REQUEST
        };

        println!(
            "Returning error with status code: {:?}, description: {:?}",
            status_code, error.error_description
        );
        Err((status_code, error.error_description))
    }
}

async fn generate_code_challenge(code_verifier: &str) -> String {
    let hash = Sha256::digest(code_verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(&hash)
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

fn generate_code_verifier() -> String {
    generate_random_string(43) // Between 43-128 characters
}

pub async fn initiate_auth(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let auth_states = &state.auth_states;
    // Generate a unique state parameter
    let state_string = generate_random_string(32);
    let code_verifier = generate_code_verifier();
    // Replace with actual user ID or relevant data
    let account_id = user.account_id.clone();

    let auth_state = AuthState {
        state: state_string.clone(),
        code_verifier: code_verifier.clone(),
        account_id: account_id.clone(),
        created_at: Utc::now(),
    };

    println!("Auth State: {:?}", auth_state);

    // Store the state in memory
    let mut auth_states_lock = auth_states.write().await;
    auth_states_lock.insert(state_string.clone(), auth_state);

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
    let code_challenge = generate_code_challenge(&code_verifier).await; // Assuming you have a function to generate code challenge

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

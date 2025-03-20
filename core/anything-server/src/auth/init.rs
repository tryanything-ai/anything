use crate::vault::insert_secret_to_vault;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

use serde_json::Value;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use slugify::slugify;
use std::env;
use std::sync::Arc;
use urlencoding;
use uuid::Uuid;

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
    pub last_failure_retry: Option<DateTime<Utc>>,
}

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
    pub auth_provider_id: String,
    pub provider_name: String,
    pub provider_label: String,
    pub provider_icon: String,
    pub provider_description: String,
    pub provider_readme: String,
    pub auth_type: String,
    pub auth_url: String,
    pub token_url: String,
    pub provider_data: Option<serde_json::Value>,
    pub access_token_lifetime_seconds: Option<String>,
    pub refresh_token_lifetime_seconds: Option<String>,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_id_vault_id: Uuid,
    pub client_secret_vault_id: Option<Uuid>,
    pub scopes: String,
    pub public: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountAuthProviderAccount {
    pub account_id: String,
    pub auth_provider_id: String,
    pub account_auth_provider_account_label: String,
    pub account_auth_provider_account_slug: String,
    pub access_token_vault_id: String,
    pub refresh_token_vault_id: String,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
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
    println!(
        "[OAUTH] Starting OAuth callback handler for provider: {}",
        provider_name
    );
    println!("[OAUTH] Received callback parameters: {:?}", params);

    let client = &state.anything_client;
    let auth_states = &state.auth_states;

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");
    println!("[OAUTH] Successfully loaded environment variables");

    // Get Provider details
    println!("[OAUTH] Fetching provider details for: {}", provider_name);
    let response = match client
        .rpc(
            "get_decrypted_auth_provider_by_name",
            json!({"provider_name_param": &provider_name}).to_string(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[OAUTH] Successfully retrieved provider details");
            response
        }
        Err(e) => {
            println!("[OAUTH] Failed to get provider details: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    println!("[OAUTH] Provider details response: {:?}", response);

    let body = match response.text().await {
        Ok(body) => {
            println!("[OAUTH] Successfully read response body");
            body
        }
        Err(e) => {
            println!("[OAUTH] Failed to read response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let auth_providers: Vec<AuthProvider> = match serde_json::from_str::<Vec<AuthProvider>>(&body) {
        Ok(providers) => {
            println!("[OAUTH] Successfully parsed auth providers");
            providers
        }
        Err(e) => {
            println!("[OAUTH] Failed to parse auth providers JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    let auth_provider = match auth_providers.into_iter().next() {
        Some(provider) => {
            println!("[OAUTH] Found auth provider: {}", provider.provider_name);
            provider
        }
        None => {
            println!("[OAUTH] No auth provider found");
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
    };

    // Verify state from the database
    println!("[OAUTH] Verifying state token");
    let auth_state = auth_states
        .read()
        .await
        .get(&params.state.unwrap())
        .cloned();

    let auth_state = match auth_state {
        Some(state) => {
            println!("[OAUTH] State verification successful");
            state
        }
        None => {
            println!("[OAUTH] Invalid state token received");
            return (StatusCode::BAD_REQUEST, "Invalid state").into_response();
        }
    };

    // Exchange code for token
    println!("[OAUTH] Exchanging authorization code for tokens");
    let token = match exchange_code_for_token(
        &auth_provider,
        &params.code.as_deref().unwrap_or(""),
        &auth_provider.redirect_url,
        &auth_state.code_verifier,
    )
    .await
    {
        Ok(token) => {
            println!("[OAUTH] Successfully exchanged code for tokens");
            token
        }
        Err(e) => {
            println!("[OAUTH] Failed to exchange code for tokens: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to exchange code for token",
            )
                .into_response();
        }
    };

    println!("[OAUTH] Generating unique account slug");
    let (account_slug, account_label) = generate_unique_account_slug(
        client,
        auth_provider.provider_label.as_str(),
        auth_state.account_id.as_str(),
    )
    .await;
    println!(
        "[OAUTH] Generated slug: {} and label: {}",
        account_slug, account_label
    );

    let refresh_token_expires_at = if let Some(refresh_token_lifetime) =
        auth_provider.refresh_token_lifetime_seconds.as_deref()
    {
        let refresh_token_lifetime: i64 = refresh_token_lifetime.parse().unwrap_or(0);
        println!(
            "[OAUTH] Refresh token lifetime: {} seconds",
            refresh_token_lifetime
        );
        Some(Utc::now() + chrono::Duration::seconds(refresh_token_lifetime))
    } else {
        println!("[OAUTH] No refresh token lifetime specified");
        None
    };

    let access_token_expires_at = if let Some(access_token_lifetime) =
        auth_provider.access_token_lifetime_seconds.as_deref()
    {
        let access_token_lifetime: i64 = access_token_lifetime.parse().unwrap_or(0);
        println!(
            "[OAUTH] Access token lifetime: {} seconds",
            access_token_lifetime
        );
        Some(Utc::now() + chrono::Duration::seconds(access_token_lifetime))
    } else {
        println!("[OAUTH] No access token lifetime specified");
        None
    };

    //Add access-token to vault
    println!("[OAUTH] Storing access token in vault");
    let vault_access_token_name = slugify!(
        format!(
            "access_token_for_{}_for_account_{}",
            account_slug.clone(),
            auth_state.account_id.clone()
        )
        .as_str(),
        separator = "_"
    );

    println!(
        "[OAUTH] Access Token Vault Name: {}",
        vault_access_token_name
    );

    let access_token_vault_id = insert_secret_to_vault(
        client,
        &vault_access_token_name,
        &token.access_token,
        &format!(
            "Access Token for {} for Account {}",
            auth_provider.auth_provider_id, auth_state.account_id
        ),
    )
    .await
    .unwrap();
    println!(
        "[OAUTH] Access token stored with vault ID: {}",
        access_token_vault_id
    );

    //Add refresh token secret in vault
    println!("[OAUTH] Storing refresh token in vault");
    let vault_refresh_token_name = slugify!(
        format!(
            "refresh_token_for_{}_for_account_{}",
            account_slug.clone(),
            auth_state.account_id.clone()
        )
        .as_str(),
        separator = "_"
    );

    println!(
        "[OAUTH] Refresh Token Vault Name: {}",
        vault_refresh_token_name
    );

    let refresh_token_vault_id = insert_secret_to_vault(
        client,
        &vault_refresh_token_name,
        &token.refresh_token.unwrap_or_default(),
        &format!(
            "Refresh Token for {} for Account {}",
            auth_provider.auth_provider_id, auth_state.account_id
        ),
    )
    .await
    .unwrap();
    println!(
        "[OAUTH] Refresh token stored with vault ID: {}",
        refresh_token_vault_id
    );

    let input = CreateAccountAuthProviderAccount {
        account_id: auth_state.account_id.clone(),
        auth_provider_id: auth_provider.auth_provider_id.clone(),
        account_auth_provider_account_label: account_label,
        account_auth_provider_account_slug: account_slug,
        access_token_vault_id: access_token_vault_id.to_string(),
        access_token_expires_at: access_token_expires_at.unwrap_or_else(Utc::now),
        refresh_token_vault_id: refresh_token_vault_id.to_string(),
        refresh_token_expires_at: refresh_token_expires_at,
    };

    println!(
        "[OAUTH] Creating account auth provider account with input: {:?}",
        input
    );

    // Store token in the database
    let create_account_response = match client
        .from("account_auth_provider_accounts")
        .auth(supabase_service_role_api_key.clone())
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[OAUTH] Successfully created account auth provider account");
            response
        }
        Err(e) => {
            println!(
                "[OAUTH] Failed to create account auth provider account: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    println!(
        "[OAUTH] Create Account Response: {:?}",
        create_account_response
    );

    // Invalidate the bundler secrets cache for this account after creating a new secret
    println!(
        "[OAUTH] Invalidating bundler secrets cache for account: {}",
        auth_state.account_id
    );
    {
        let mut cache = state.bundler_accounts_cache.write().await;
        cache.invalidate(&auth_state.account_id);
    }
    println!("[OAUTH] Cache invalidated successfully");

    // Return success response
    if create_account_response.status().is_success() {
        println!("[OAUTH] Authentication process completed successfully");
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
        println!("[OAUTH] Authentication process failed");
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
pub struct ErrorResponse {
    pub error: String,
    pub error_description: String,
}

pub async fn exchange_code_for_token(
    provider: &AuthProvider,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> Result<OAuthToken, (StatusCode, String)> {
    println!("[OAUTH] Starting code exchange for token");
    let client = Client::new();

    let mut form_params = vec![
        ("code", code),
        ("client_id", &provider.client_id),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
        ("code_verifier", code_verifier),
    ];

    // Add client_secret if present
    if let Some(client_secret) = &provider.client_secret {
        println!("[OAUTH] Adding client secret to token request");
        form_params.push(("client_secret", client_secret));
    }

    println!("[OAUTH] Token exchange form parameters: {:?}", form_params);
    println!(
        "[OAUTH] Making POST request to token URL: {}",
        provider.token_url
    );

    let response = client
        .post(provider.token_url.clone())
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&form_params)
        .send()
        .await
        .map_err(|e| {
            println!("[OAUTH] Token exchange request failed: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let status = response.status();
    println!("[OAUTH] Token exchange response status: {:?}", status);

    let body = response.text().await.map_err(|e| {
        println!(
            "[OAUTH] Error reading token exchange response body: {:?}",
            e
        );
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    println!("[OAUTH] Token exchange response body: {:?}", body);

    if status.is_success() {
        println!("[OAUTH] Token exchange successful, parsing response");
        serde_json::from_str::<OAuthToken>(&body).map_err(|e| {
            println!("[OAUTH] Failed to parse successful token response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse token response: {}", e),
            )
        })
    } else {
        println!("[OAUTH] Token exchange failed, parsing error response");
        let error: ErrorResponse = serde_json::from_str(&body).map_err(|e| {
            println!("[OAUTH] Failed to parse error response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse error response: {}", e),
            )
        })?;

        let status_code = if error.error == "invalid_client" {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::BAD_REQUEST
        };

        println!(
            "[OAUTH] Returning error response - Status: {:?}, Description: {:?}",
            status_code, error.error_description
        );
        Err((status_code, error.error_description))
    }
}

pub async fn generate_oauth_init_url_for_client(
    Path((account_id, provider_name)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!(
        "[OAUTH] Generating OAuth URL for account: {} and provider: {}",
        account_id, provider_name
    );

    let auth_states = &state.auth_states;
    // Generate a unique state parameter
    let state_string = generate_random_string(32);
    let code_verifier = generate_code_verifier();
    println!(
        "[OAUTH] Generated state: {} and code verifier",
        state_string
    );

    let auth_state = AuthState {
        state: state_string.clone(),
        code_verifier: code_verifier.clone(),
        account_id: account_id.clone(),
        created_at: Utc::now(),
    };

    println!("[OAUTH] Created auth state: {:?}", auth_state);

    // Store the state in memory
    let mut auth_states_lock = auth_states.write().await;
    auth_states_lock.insert(state_string.clone(), auth_state);
    println!("[OAUTH] Stored auth state in memory");

    let client = &state.anything_client;

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");
    println!("[OAUTH] Loaded environment variables");

    // Get Provider details
    println!("[OAUTH] Fetching provider details for: {}", provider_name);
    let response = match client
        .rpc(
            "get_decrypted_auth_provider_by_name",
            json!({"provider_name_param": &provider_name}).to_string(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[OAUTH] Successfully retrieved provider details");
            response
        }
        Err(e) => {
            println!("[OAUTH] Failed to find provider: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find that provider",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[OAUTH] Successfully read provider response body");
            body
        }
        Err(e) => {
            println!("[OAUTH] Failed to read provider response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let auth_providers: Vec<AuthProvider> = match serde_json::from_str::<Vec<AuthProvider>>(&body) {
        Ok(providers) => {
            println!(
                "[OAUTH] Successfully parsed {} auth providers",
                providers.len()
            );
            providers
        }
        Err(e) => {
            println!("[OAUTH] Failed to parse auth providers JSON: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse JSON for auth_providers",
            )
                .into_response();
        }
    };

    if auth_providers.is_empty() {
        println!("[OAUTH] No auth providers found");
        return (StatusCode::NOT_FOUND, "No auth providers found").into_response();
    }

    let auth_provider = match auth_providers.into_iter().next() {
        Some(provider) => {
            println!("[OAUTH] Selected auth provider: {}", provider.provider_name);
            provider
        }
        None => {
            println!("[OAUTH] No auth provider found after parsing");
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
    };

    // Build the OAuth URL
    println!("[OAUTH] Building OAuth URL");
    let client_id = auth_provider.client_id.clone();
    let redirect_uri = auth_provider.redirect_url.clone();
    let auth_url = auth_provider.auth_url.clone();
    let scope = auth_provider.scopes.clone();

    println!("[OAUTH] Generating code challenge from verifier");
    let code_challenge = generate_code_challenge(&code_verifier).await;

    //access_type=offline is for google to provide refresh_token
    //https://developers.google.com/identity/protocols/oauth2/web-server#httprest
    //prompt=consent is for google to show the consent screen
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256&access_type=offline&prompt=consent",
        auth_url,
        client_id,
        urlencoding::encode(redirect_uri.as_str()),
        urlencoding::encode(scope.as_str()),
        urlencoding::encode(&state_string),
        urlencoding::encode(&code_challenge)
    );

    println!("[OAUTH] Generated OAuth URL: {}", auth_url);

    Json(OAuthResponse { url: auth_url }).into_response()
}

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
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct AuthProvider {
    auth_provider_id: String,
    provider_name: String,
    provider_label: String,
    provider_icon: String,
    provider_description: String,
    provider_readme: String,
    auth_type: String,
    auth_url: String,
    token_url: String,
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
struct OAuthCallback {
    code: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
struct OAuthToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
struct InitiateAuthFlow {
    redirect_uri: String,
}

pub async fn handle_provider_callback(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Query(params): Query<OAuthCallback>,
) -> impl IntoResponse {
    println!("Handling auth callback for provider: {:?}", provider_name);

    let client = &state.anything_client;

    //Get Provider details
    let response = match client
        .from("auth_providers")
        .auth(user.jwt)
        .eq("provider_name", &provider_name)
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => {
            println!("Response: {:?}", response);
            response
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("Body: {:?}", body);
            body
        }
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

    // let pool = &state.db_pool;

    // Verify state from the database
    // TODO: Implement state verification

    // Exchange code for token
    let token = match exchange_code_for_token(&auth_provider, &params.code).await {
        Ok(token) => token,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to exchange code for token").into_response()
        }
    };

    // Store token in the database
    // match store_token_in_db(pool, &user.id, &provider_name, &token).await {
    //     Ok(_) => (),
    //     Err(_) => {
    //         return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to store token in database").into_response()
    //     }
    // };

    // // Redirect user or return success message
    // (
    //     StatusCode::OK,
    (
        StatusCode::OK,
        Json(serde_json::json!({"status": "success"})),
    )
        .into_response()
}
//     State(state): State<Arc<AppState>>,
//     Extension(user): Extension<User>,
//     Json(payload): Json<InitiateAuthFlow>,
// ) -> impl IntoResponse {
//     println!("Initiating auth flow for provider: {:?}", provider_name);

//     let client = &state.anything_client;
//     let pool = &state.db_pool;

//     // Generate state
//     let state = Uuid::new_v4().to_string();

//     // Store state in the database
//     store_state_in_db(pool, &state, &user.id, &provider_name).await?;

//     // Construct the authorization URL
//     let auth_url = construct_auth_url(&provider_name, &state, &payload.redirect_uri);

//     (StatusCode::OK, Json(serde_json::json!({"auth_url": auth_url})))
// }

async fn exchange_code_for_token(
    provider: &AuthProvider,
    code: &str,
) -> Result<OAuthToken, StatusCode> {
    //reqwest client
    let client = Client::new();

    // let token_url = format!("https://{}.com/oauth/token", provider);
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

// async fn store_token_in_db(
//     pool: &PgPool,
//     user_id: &Uuid,
//     provider: &str,
//     token: &OAuthToken,
// ) -> Result<(), StatusCode> {
//     sqlx::query!(
//         r#"
//         INSERT INTO anything.account_auth_provider_accounts
//         (account_id, auth_provider_id, account_auth_provider_account_label, account_auth_provider_account_slug, access_token, refresh_token, expires_at)
//         VALUES ($1, $2, $3, $4, $5, $6, $7)
//         "#,
//         user_id,
//         provider,
//         format!("{} Account", provider),
//         provider.to_lowercase(),
//         token.access_token,
//         token.refresh_token,
//         token.expires_at,
//     )
//     .execute(pool)
//     .await
//     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(())
// }

// async fn store_state_in_db(
//     pool: &PgPool,
//     state: &str,
//     user_id: &Uuid,
//     provider: &str,
// ) -> Result<(), StatusCode> {
//     // TODO: Implement state storage logic
//     // This might involve creating a new table for temporary state storage
//     Ok(())
// }

// fn construct_auth_url(provider: &str, state: &str, redirect_uri: &str) -> String {
//     // TODO: Implement proper URL construction for each provider
//     format!("https://{}.com/oauth/authorize?client_id=YOUR_CLIENT_ID&redirect_uri={}&state={}", provider, redirect_uri, state)
// }

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

use dotenv::dotenv;
use std::env;

pub async fn get_auth_accounts(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!(
        "Handling a get auth accounts for account_id: {}",
        account_id
    );

    let client = &state.anything_client;

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("account_auth_provider_accounts")
        .auth(supabase_service_role_api_key.clone())
        .eq("account_id", &account_id)
        .select("*, auth_provider:auth_providers(auth_provider_id, provider_name, provider_label, provider_icon, provider_description, provider_readme, auth_type, auth_url, token_url, access_token_lifetime_seconds, refresh_token_lifetime_seconds, scopes, public, updated_at, created_at)")
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
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

pub async fn get_auth_accounts_for_provider_name(
    Path((account_id, provider_name)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "Handling a get_auth_accounts_for_provider_name for account {:?} and provider {:?}",
        account_id, provider_name
    );

    let client = &state.anything_client;

    let response = match client
        .from("account_auth_provider_accounts")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("auth_provider_id", &provider_name)
        .select("*")
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
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}

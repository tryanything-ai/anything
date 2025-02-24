use crate::auth::utils::{insert_secret_to_vault, update_secret_in_vault};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use slugify::slugify;
use std::env;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SetAuthProviderClientIdPayload {
    client_id: String,
    cli_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAuthProviderClientIdPayload {
    client_id_vault_id: String,
    new_client_id: String,
    cli_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateVaultSecretInput {
    id: String,
    secret: String,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAuthProviderClientSecretPayload {
    client_secret_id: String,
    cli_secret: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateAuthProviderClientIdResopnse {
    auth_provider_id: String,
    message: String,
}

pub async fn get_auth_providers(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!(
        "Handling a get auth providers for account_id: {}",
        account_id
    );

    let client = &state.anything_client;
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("auth_providers")
        .auth(supabase_service_role_api_key.clone())
        .select("auth_provider_id, provider_name, provider_label, provider_icon, provider_description, provider_readme, auth_type, auth_url, token_url, access_token_lifetime_seconds, refresh_token_lifetime_seconds, scopes, public, updated_at, created_at")
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

pub async fn update_auth_provider_client_id(
    State(state): State<Arc<AppState>>,
    Path(auth_provider_name): Path<String>,
    Json(payload): Json<UpdateAuthProviderClientIdPayload>,
) -> impl IntoResponse {
    dotenv().ok();
    let cli_secret = env::var("CLI_SECRET").expect("CLI_SECRET must be set");
    let client = &state.anything_client;

    // Check if the user has the correct CLI_SECRET
    if payload.cli_secret != cli_secret {
        return (StatusCode::UNAUTHORIZED, "Invalid CLI_SECRET").into_response();
    }

    println!("[PROVIDER SECRETS] create_secret Input?: {:?}", payload);

    match update_secret_in_vault(client, &payload.client_id_vault_id, &payload.new_client_id).await
    {
        Ok(_) => {
            let response = UpdateAuthProviderClientIdResopnse {
                auth_provider_id: auth_provider_name,
                message: "Client ID updated successfully".to_string(),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update client ID: {}", e),
        )
            .into_response(),
    }
}

pub async fn set_auth_provider_client_id(
    State(state): State<Arc<AppState>>,
    Path(auth_provider_name): Path<String>,
    Json(payload): Json<SetAuthProviderClientIdPayload>,
) -> impl IntoResponse {
    dotenv().ok();
    let cli_secret = env::var("CLI_SECRET").expect("CLI_SECRET must be set");

    let client = &state.anything_client;

    // Check if the user has the correct CLI_SECRET
    if payload.cli_secret != cli_secret {
        return (StatusCode::UNAUTHORIZED, "Invalid CLI_SECRET").into_response();
    }

    println!("[PROVIDER SECRETS] create_secret Input?: {:?}", payload);

    let vault_client_id_name = slugify!(
        format!("providers_client_id_for_{}", auth_provider_name.clone()).as_str(),
        separator = "_"
    );

    // Insert client_id secret using the utility function
    let client_id_secret_vault_id = match insert_secret_to_vault(
        client,
        &vault_client_id_name,
        &payload.client_id,
        "Client ID for Auth Provider",
    )
    .await
    {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert client_id secret",
            )
                .into_response()
        }
    };

    let client = &state.anything_client;

    // Get Special Privileges by passing service_role in auth()
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Update the auth provider
    let response = match client
        .from("auth_providers")
        .auth(supabase_service_role_api_key)
        .eq("auth_provider_id", &auth_provider_name)
        .update(
            json!({
                "client_id_vault_id": client_id_secret_vault_id
            })
            .to_string(),
        )
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

    if response.status() == 200 || response.status() == 204 {
        let response_body = UpdateAuthProviderClientIdResopnse {
            auth_provider_id: auth_provider_name,
            message: "Auth provider updated successfully".to_string(),
        };
        Json(response_body).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update auth provider",
        )
            .into_response()
    }
}

pub async fn set_auth_provider_client_secret_id(
    State(state): State<Arc<AppState>>,
    Path(auth_provider_name): Path<String>,
    Json(payload): Json<UpdateAuthProviderClientSecretPayload>,
) -> impl IntoResponse {
    dotenv().ok();
    let cli_secret = env::var("CLI_SECRET").expect("CLI_SECRET must be set");

    let client = &state.anything_client;

    // Check if the user has the correct CLI_SECRET
    if payload.cli_secret != cli_secret {
        return (StatusCode::UNAUTHORIZED, "Invalid CLI_SECRET").into_response();
    }

    println!("[PROVIDER SECRETS] create_secret Input?: {:?}", payload);

    let vault_client_secret_id_name = slugify!(
        format!(
            "providers_client_secret_id_for_{}",
            auth_provider_name.clone()
        )
        .as_str(),
        separator = "_"
    );

    // Insert client_secret_id using the utility function
    let client_id_secret_vault_id = match insert_secret_to_vault(
        client,
        &vault_client_secret_id_name,
        &payload.client_secret_id,
        "Client Secret ID for Auth Provider",
    )
    .await
    {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert client_secret_id",
            )
                .into_response()
        }
    };

    let client = &state.anything_client;

    // Get Special Privileges by passing service_role in auth()
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Update the auth provider
    let response = match client
        .from("auth_providers")
        .auth(supabase_service_role_api_key)
        .eq("auth_provider_id", &auth_provider_name)
        .update(
            json!({
                "client_secret_vault_id": client_id_secret_vault_id
            })
            .to_string(),
        )
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

    if response.status() == 200 || response.status() == 204 {
        let response_body = UpdateAuthProviderClientIdResopnse {
            auth_provider_id: auth_provider_name,
            message: "Auth provider updated successfully".to_string(),
        };
        Json(response_body).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update auth provider",
        )
            .into_response()
    }
}

pub async fn get_auth_provider_by_name(
    Path((account_id, provider_name)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!(
        "Handling a get_auth_provider_by_name for account {:?} and provider {:?}",
        account_id, provider_name
    );

    let client = &state.anything_client;

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("auth_providers")
        .auth(supabase_service_role_api_key.clone())
        .eq("provider_name", &provider_name)
        .select("auth_provider_id, provider_name, provider_label, provider_icon, provider_description, provider_readme, auth_type, auth_url, token_url, access_token_lifetime_seconds, refresh_token_lifetime_seconds, scopes, public, updated_at, created_at)")
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

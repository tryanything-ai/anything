use crate::supabase_auth_middleware::User;
use crate::AppState;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use slugify::slugify;
use std::env;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UpdateAuthProviderClientIdPayload {
    client_id: String,
    cli_secret: String,
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

#[derive(Debug, Serialize)]
pub struct UpdateAuthProviderClientSecretResopnse {
    auth_provider_client_secret_id: String,
    message: String,
}

pub async fn update_auth_provider_client_id(
    State(state): State<Arc<AppState>>,
    Path(auth_provider_name): Path<String>,
    Json(payload): Json<UpdateAuthProviderClientIdPayload>,
) -> impl IntoResponse {
    dotenv().ok();
    let cli_secret = env::var("CLI_SECRET").expect("CLI_SECRET must be set");
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

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

    // Insert client_id secret
    let client_id_input = json!({
        "name": vault_client_id_name,
        "secret": payload.client_id,
        "description": "Client ID for Auth Provider",
    });

    let client_id_response = match client
        .rpc(
            "insert_secret",
            serde_json::to_string(&client_id_input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert client_id secret",
            )
                .into_response()
        }
    };

    let client_id_body = match client_id_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read client_id response body",
            )
                .into_response()
        }
    };

    println!(
        "[PROVIDER SECRETS] Response from vault insert (client_id): {:?}",
        client_id_body
    );

    let client_id_secret_vault_id = client_id_body.trim_matches('"');

    //TODO: set vault ids in table

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

pub async fn update_auth_provider_client_secret_id(
    State(state): State<Arc<AppState>>,
    Path(auth_provider_name): Path<String>,
    Json(payload): Json<UpdateAuthProviderClientSecretPayload>,
) -> impl IntoResponse {
    dotenv().ok();
    let cli_secret = env::var("CLI_SECRET").expect("CLI_SECRET must be set");
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

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

    // Insert client_id secret
    let client_secret_id_input = json!({
        "name": vault_client_secret_id_name,
        "secret": payload.client_secret_id,
        "description": "Client Secret ID for Auth Provider",
    });

    let client_id_response = match client
        .rpc(
            "insert_secret",
            serde_json::to_string(&client_secret_id_input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert client_id secret",
            )
                .into_response()
        }
    };

    let client_secret_id_body = match client_id_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read client_id response body",
            )
                .into_response()
        }
    };

    println!(
        "[PROVIDER SECRETS] Response from vault insert (client_secret_id): {:?}",
        client_secret_id_body
    );

    let client_id_secret_vault_id = client_secret_id_body.trim_matches('"');

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

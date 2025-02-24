use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

use dotenv::dotenv;
use slugify::slugify;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretPayload {
    secret_name: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretInput {
    name: String,
    secret: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnythingCreateSecretInput {
    secret_id: String,
    secret_name: String,
    vault_secret_id: String,
    secret_description: String,
    account_id: String,
}

pub async fn create_secret(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<CreateSecretPayload>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    println!("create_secret Input?: {:?}", payload);

    let vault_secret_name = slugify!(
        format!("{}_{}", account_id.clone(), payload.secret_name.clone()).as_str(),
        separator = "_"
    );

    println!("New Name: {}", vault_secret_name);

    let input = CreateSecretInput {
        name: vault_secret_name,
        secret: payload.secret_value.clone(),
        description: payload.secret_description.clone(),
    };

    // Create Secret in Vault using utility function
    let secret_vault_id = match crate::vault::insert_secret_to_vault(
        client,
        &input.name,
        &input.secret,
        &input.description,
    )
    .await
    {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create secret in vault",
            )
                .into_response()
        }
    };

    let anything_secret_input = AnythingCreateSecretInput {
        secret_id: secret_vault_id.clone(),
        secret_name: payload.secret_name.clone(),
        vault_secret_id: secret_vault_id,
        secret_description: payload.secret_description.clone(),
        account_id: account_id.clone(),
    };

    //Create Flow Version
    let db_secret_response = match client
        .from("secrets")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&anything_secret_input).unwrap())
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

    let db_secret_body = match db_secret_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("DB Secret Body: {:?}", db_secret_body);

    // Invalidate the bundler secrets cache for this account after creating a new secret
    // Only lock for the minimum time needed
    {
        let mut cache = state.bundler_secrets_cache.write().await;
        cache.invalidate(&account_id);
    }

    Json(db_secret_body).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnythingApiKeyPayload {
    secret_name: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnythingApiKeySecretInput {
    secret_id: String,
    secret_name: String,
    vault_secret_id: String,
    secret_description: String,
    account_id: String,
    anything_api_key: bool,
}

pub async fn create_anything_api_key(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<CreateAnythingApiKeyPayload>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    println!("create_secret Input?: {:?}", payload);

    let vault_secret_name = slugify!(
        format!(
            "api_key_{}_{}",
            account_id.clone(),
            payload.secret_name.clone()
        )
        .as_str(),
        separator = "_"
    );

    println!("New Name: {}", vault_secret_name);

    // Generate a unique API key with a prefix for easy identification
    let api_key = format!("any_{}", uuid::Uuid::new_v4());

    // Create Secret in Vault using utility function
    let secret_vault_id = match crate::vault::insert_secret_to_vault(
        client,
        &vault_secret_name,
        &api_key,
        &payload.secret_description,
    )
    .await
    {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create secret in vault",
            )
                .into_response()
        }
    };

    let anything_secret_input = CreateAnythingApiKeySecretInput {
        secret_id: secret_vault_id.clone(),
        secret_name: payload.secret_name.clone(),
        vault_secret_id: secret_vault_id,
        secret_description: payload.secret_description.clone(),
        account_id: account_id.clone(),
        anything_api_key: true,
    };

    //Create Flow Version
    let db_secret_response = match client
        .from("secrets")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&anything_secret_input).unwrap())
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

    let db_secret_body = match db_secret_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("DB Secret Body: {:?}", db_secret_body);

    Json(db_secret_body).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetDecryptedSecretsInput {
    team_account_id: String,
}

// Secrets
pub async fn get_decrypted_secrets(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!("Handling a get_decrypted_secrets");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let input = GetDecryptedSecretsInput {
        team_account_id: account_id,
    };

    println!("get_decrypted_secrets rpc Input?: {:?}", input);

    let client = &state.anything_client;

    let response = match client
        .rpc(
            "get_decrypted_secrets",
            serde_json::to_string(&input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone())
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

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(items).into_response()
}

// Secrets
pub async fn get_decrypted_anything_api_keys(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!("Handling a get_decrypted_secrets");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let input = GetDecryptedSecretsInput {
        team_account_id: account_id,
    };

    println!("get_decrypted_anything_api_keys rpc Input?: {:?}", input);

    let client = &state.anything_client;

    let response = match client
        .rpc(
            "get_decrypted_anything_api_keys",
            serde_json::to_string(&input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone())
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

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(items).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSecretPayload {
    secret_id: String,
    secret_value: String,
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateVaultSecretInput {
    id: String,
    secret: String,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAnythingSecretInput {
    secret_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadVaultSecretInput {
    secret_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadVaultDecryptedSecretInput {
    secret_uuid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteVaultSecretInput {
    secret_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSecretBySecretValueInput {
    secret_value: String,
}

pub async fn delete_secret(
    Path((account_id, secret_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!(
        "Delete Secret: {:?} for account: {:?}",
        secret_id, account_id
    );

    let client = &state.anything_client;

    // Delete in DB
    let response = match client
        .from("secrets")
        .auth(user.jwt)
        .eq("secret_id", &secret_id)
        .eq("account_id", &account_id)
        .delete()
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

    println!("Delete DB Secret Body: {:?}", body);

    //Delete in Vault
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    //If the user is allowed to delete the secret from the anything.secrets table the RLS policy means they are allowed to delete from vault.
    // It should fail if the user is not allowed to delete from the anything.secrets table
    // So this should be safe ( but i wish it was safer )
    //TODO: protect this more. right now its a little open.
    //TODO: protect this more. right now its a little open.
    let input = DeleteVaultSecretInput {
        secret_id: secret_id.clone(),
    };

    println!("delete secret rpc Input?: {:?}", input);

    let rpc_response = match client
        .rpc("delete_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone())
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

    let rpc_body = match rpc_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("Delete Vault Secret Body: {:?}", rpc_body);

    // Invalidate the bundler secrets cache for this account after creating a new secret
    // Only lock for the minimum time needed
    {
        let mut cache = state.bundler_secrets_cache.write().await;
        cache.invalidate(&account_id);
    }

    Json(body).into_response()
}

pub async fn delete_api_key(
    Path((account_id, secret_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!(
        "[DELETE API KEY] Deleting secret: {:?} for account: {:?}",
        secret_id, account_id
    );

    let client = &state.anything_client;

    // Get the API key value from vault before deleting
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");
    let get_secret_input = ReadVaultDecryptedSecretInput {
        secret_uuid: secret_id.clone(),
    };
    println!("[DELETE API KEY] Getting secret value from vault");
    let get_secret_response = match client
        .rpc(
            "get_decrypted_secret",
            serde_json::to_string(&get_secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[DELETE API KEY] Got response from vault: {:?}", response);
            response
        }
        Err(e) => {
            println!("[DELETE API KEY] Failed to get secret from vault: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let secret_body = match get_secret_response.text().await {
        Ok(body) => {
            println!("[DELETE API KEY] Got secret body from response: {:?}", body);
            body
        }
        Err(e) => {
            println!(
                "[DELETE API KEY] Failed to read secret value from response: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let secret_json: Value = match serde_json::from_str(&secret_body) {
        Ok(json) => json,
        Err(_) => {
            println!("[DELETE API KEY] Failed to parse secret JSON");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse secret response",
            )
                .into_response();
        }
    };

    // Extract secret value from the returned JSON array
    let secret_value = match secret_json[0]["secret_value"].as_str() {
        Some(value) => value.to_string(),
        None => {
            println!("[DELETE API KEY] Failed to get secret value from JSON");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get secret value",
            )
                .into_response();
        }
    };

    println!("[DELETE API KEY] Removing API key from cache");
    // Delete from API key cache
    {
        let mut cache = state.api_key_cache.write().await;
        let removed = cache.remove(&secret_value);
        println!(
            "[DELETE API KEY] Successfully removed from cache: {}",
            removed.is_some()
        );
    }

    println!("[DELETE API KEY] Deleting secret from database");
    // Delete in DB
    let response = match client
        .from("secrets")
        .auth(user.jwt)
        .eq("secret_id", &secret_id)
        .eq("account_id", &account_id)
        .delete()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            println!("[DELETE API KEY] Failed to delete from database");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            println!("[DELETE API KEY] Failed to read database response");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    println!("[DELETE API KEY] Database deletion response: {:?}", body);

    println!("[DELETE API KEY] Deleting secret from vault");
    //Delete in Vault
    let input = DeleteVaultSecretInput {
        secret_id: secret_id.clone(),
    };

    println!("[DELETE API KEY] Vault deletion input: {:?}", input);

    let rpc_response = match client
        .rpc("delete_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            println!("[DELETE API KEY] Failed to delete from vault");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let rpc_body = match rpc_response.text().await {
        Ok(body) => body,
        Err(_) => {
            println!("[DELETE API KEY] Failed to read vault deletion response");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    println!("[DELETE API KEY] Vault deletion response: {:?}", rpc_body);
    println!("[DELETE API KEY] API key deletion completed successfully");

    Json(body).into_response()
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

pub async fn get_secret_by_secret_value(
    state: Arc<AppState>,
    secret_value: String,
) -> Result<SecretByValueResponse, StatusCode> {
    println!("[GET SECRET BY SECRET VALUE] Starting get_secret_by_value");
    println!(
        "[GET SECRET BY SECRET VALUE] Secret Value: {:?}",
        secret_value
    );
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let input = GetSecretBySecretValueInput {
        secret_value: secret_value.clone(),
    };

    let client = &state.anything_client;

    println!("[GET SECRET BY SECRET VALUE] Making RPC call to get_secret_by_secret_value");
    let response = client
        .rpc(
            "get_secret_by_secret_value",
            serde_json::to_string(&input).unwrap(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!(
        "[GET SECRET BY SECRET VALUE] Got response from RPC call: {:?}",
        response
    );
    let body = response.text().await.map_err(|e| {
        println!(
            "[GET SECRET BY SECRET VALUE] Error getting response text: {:?}",
            e
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("[GET SECRET BY SECRET VALUE] Response body: {}", body);

    if body.contains("[]") {
        println!("[GET SECRET BY SECRET VALUE] No secret found - body was empty array");
        return Err(StatusCode::NOT_FOUND);
    }

    println!("[GET SECRET BY SECRET VALUE] Parsing response body");
    let mut secrets: Vec<SecretByValueResponse> =
        serde_json::from_str(&body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // We expect only one result since secret values should be unique
    println!("[GET SECRET BY SECRET VALUE] Returning secret");
    secrets.pop().ok_or(StatusCode::NOT_FOUND)
}

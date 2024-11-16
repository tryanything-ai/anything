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

use crate::supabase_auth_middleware::User;
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

    println!("insert_secret rpc Input?: {:?}", input);

    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Create Secret in Vault
    let response = match client
        .rpc("insert_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
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

    println!("Response from vault insert: {:?}", body);

    let secret_vault_id = body.trim_matches('"');

    let anything_secret_input = AnythingCreateSecretInput {
        secret_id: secret_vault_id.to_string(), //use the same id in vault and public secrets table for dx
        secret_name: payload.secret_name.clone(),
        vault_secret_id: secret_vault_id.to_string(), //vault_secret_id.clone(),
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

    let input = CreateSecretInput {
        name: vault_secret_name,
        secret: api_key,
        description: payload.secret_description.clone(),
    };

    println!("insert_secret rpc Input?: {:?}", input);

    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Create Secret in Vault
    let response = match client
        .rpc("insert_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
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

    println!("Response from vault insert: {:?}", body);

    let secret_vault_id = body.trim_matches('"');

    let anything_secret_input = CreateAnythingApiKeySecretInput {
        secret_id: secret_vault_id.to_string(), //use the same id in vault and public secrets table for dx
        secret_name: payload.secret_name.clone(),
        vault_secret_id: secret_vault_id.to_string(), //vault_secret_id.clone(),
        secret_description: payload.secret_description.clone(),
        account_id: account_id.clone(),
        anything_api_key: true, //Important not to leak keys into the public secrets table
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

pub async fn update_secret(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<UpdateSecretPayload>,
) -> impl IntoResponse {
    let read_secret_input = ReadVaultSecretInput {
        secret_id: payload.secret_id.clone(),
    };

    //Get Special Priveledges by passing service_role in auth()
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let client = &state.anything_client;

    // Read Secret in Vault
    let response = match client
        .rpc(
            "read_secret",
            serde_json::to_string(&read_secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
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

    let vault_secret_body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("Vault Secret Body: {:?}", vault_secret_body);

    let vault_secret_json: serde_json::Value = serde_json::from_str(&vault_secret_body).unwrap();
    let secret_name = vault_secret_json[0]["name"].as_str().unwrap_or_default();

    println!("Secret Name: {:?}", secret_name);

    println!("update_secret Input?: {:?}", payload);

    let input = UpdateVaultSecretInput {
        id: payload.secret_id.clone(),
        secret: payload.secret_value.clone(),
        name: secret_name.to_string(),
        description: payload.secret_description.clone(),
    };

    println!("update_secret rpc Input?: {:?}", input);

    // Create Secret in Vault
    let response = match client
        .rpc("update_secret", serde_json::to_string(&input).unwrap())
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
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

    let update_secret_body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    println!("update_secret_body: {:?}", update_secret_body);

    let anything_secret_input = UpdateAnythingSecretInput {
        secret_description: payload.secret_description.clone(),
    };

    //Update Secret
    let db_secret_response = match client
        .from("secrets")
        .auth(user.jwt.clone())
        .eq("secret_id", &payload.secret_id.clone())
        .eq("account_id", &account_id)
        .update(serde_json::to_string(&anything_secret_input).unwrap())
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

    println!("Update DB Secret Body: {:?}", db_secret_body);

    Json(db_secret_body).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteVaultSecretInput {
    secret_id: String,
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

    Json(body).into_response()
}

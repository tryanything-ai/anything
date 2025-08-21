use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use std::sync::Arc;

use crate::entities::{secrets, users};
use crate::custom_auth::{jwt::Claims, extractors::AuthClaims};
use crate::AppState;
use super::encryption::{encrypt_secret, decrypt_secret};

#[derive(Deserialize)]
pub struct CreateSecretRequest {
    pub secret_name: String,
    pub secret_value: String,
    pub description: Option<String>,
    pub is_api_key: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateSecretRequest {
    pub secret_name: Option<String>,
    pub secret_value: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct SecretResponse {
    pub secret_id: String,
    pub account_id: String,
    pub secret_name: String,
    pub secret_value: String, // Only included in get requests
    pub description: Option<String>,
    pub is_api_key: bool,
    pub archived: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize)]
pub struct SecretListResponse {
    pub secret_id: String,
    pub account_id: String,
    pub secret_name: String,
    pub description: Option<String>,
    pub is_api_key: bool,
    pub archived: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    // Note: secret_value is NOT included in list responses for security
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Create a new secret
pub async fn create_secret(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
    AuthClaims(claims): AuthClaims, // Extracted by middleware
    Json(request): Json<CreateSecretRequest>,
) -> Result<ResponseJson<SecretResponse>, StatusCode> {
    let account_uuid = Uuid::parse_str(&account_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: Verify user has access to this account
    // This would check the user_accounts table

    // Check if secret name already exists for this account
    let existing_secret = secrets::Entity::find()
        .filter(secrets::Column::AccountId.eq(account_uuid))
        .filter(secrets::Column::SecretName.eq(&request.secret_name))
        .filter(secrets::Column::Archived.eq(false))
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_secret.is_some() {
        return Err(StatusCode::CONFLICT); // Secret name already exists
    }

    // Encrypt the secret value
    let (encrypted_data, nonce) = encrypt_secret(&*state.db, &request.secret_value)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Create the secret
    let secret_id = Uuid::new_v4();
    let new_secret = secrets::ActiveModel {
        secret_id: Set(secret_id),
        account_id: Set(account_uuid),
        secret_name: Set(request.secret_name.clone()),
        secret_value_encrypted: Set(encrypted_data),
        nonce: Set(nonce),
        description: Set(request.description.clone()),
        is_api_key: Set(request.is_api_key.unwrap_or(false)),
        archived: Set(false),
        created_by: Set(Some(user_id)),
        updated_by: Set(Some(user_id)),
        ..Default::default()
    };

    let secret = new_secret.insert(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = SecretResponse {
        secret_id: secret.secret_id.to_string(),
        account_id: secret.account_id.to_string(),
        secret_name: secret.secret_name,
        secret_value: request.secret_value, // Return the original value
        description: secret.description,
        is_api_key: secret.is_api_key,
        archived: secret.archived,
        created_at: secret.created_at.map(|dt| dt.to_rfc3339()),
        updated_at: secret.updated_at.map(|dt| dt.to_rfc3339()),
    };

    Ok(ResponseJson(response))
}

/// Get all secrets for an account (without values)
pub async fn get_secrets(
    State(state): State<Arc<AppState>>,
    Path(account_id): Path<String>,
    AuthClaims(_claims): AuthClaims, // Extracted by middleware
) -> Result<ResponseJson<Vec<SecretListResponse>>, StatusCode> {
    let account_uuid = Uuid::parse_str(&account_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: Verify user has access to this account

    let secrets_list = secrets::Entity::find()
        .filter(secrets::Column::AccountId.eq(account_uuid))
        .filter(secrets::Column::Archived.eq(false))
        .all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<SecretListResponse> = secrets_list
        .into_iter()
        .map(|secret| SecretListResponse {
            secret_id: secret.secret_id.to_string(),
            account_id: secret.account_id.to_string(),
            secret_name: secret.secret_name,
            description: secret.description,
            is_api_key: secret.is_api_key,
            archived: secret.archived,
            created_at: secret.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: secret.updated_at.map(|dt| dt.to_rfc3339()),
        })
        .collect();

    Ok(ResponseJson(response))
}

/// Get a specific secret (with decrypted value)
pub async fn get_secret(
    State(state): State<Arc<AppState>>,
    Path((account_id, secret_id)): Path<(String, String)>,
    AuthClaims(_claims): AuthClaims, // Extracted by middleware
) -> Result<ResponseJson<SecretResponse>, StatusCode> {
    let account_uuid = Uuid::parse_str(&account_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let secret_uuid = Uuid::parse_str(&secret_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: Verify user has access to this account

    let secret = secrets::Entity::find_by_id(secret_uuid)
        .filter(secrets::Column::AccountId.eq(account_uuid))
        .filter(secrets::Column::Archived.eq(false))
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Decrypt the secret value
    let decrypted_value = decrypt_secret(&*state.db, &secret.secret_value_encrypted, &secret.nonce)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = SecretResponse {
        secret_id: secret.secret_id.to_string(),
        account_id: secret.account_id.to_string(),
        secret_name: secret.secret_name,
        secret_value: decrypted_value,
        description: secret.description,
        is_api_key: secret.is_api_key,
        archived: secret.archived,
        created_at: secret.created_at.map(|dt| dt.to_rfc3339()),
        updated_at: secret.updated_at.map(|dt| dt.to_rfc3339()),
    };

    Ok(ResponseJson(response))
}

/// Update a secret
pub async fn update_secret(
    State(state): State<Arc<AppState>>,
    Path((account_id, secret_id)): Path<(String, String)>,
    AuthClaims(claims): AuthClaims, // Extracted by middleware
    Json(request): Json<UpdateSecretRequest>,
) -> Result<ResponseJson<SecretResponse>, StatusCode> {
    let account_uuid = Uuid::parse_str(&account_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let secret_uuid = Uuid::parse_str(&secret_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: Verify user has access to this account

    let secret = secrets::Entity::find_by_id(secret_uuid)
        .filter(secrets::Column::AccountId.eq(account_uuid))
        .filter(secrets::Column::Archived.eq(false))
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut secret_update: secrets::ActiveModel = secret.clone().into();
    let mut updated_value = None;

    // Update secret name if provided
    if let Some(new_name) = &request.secret_name {
        // Check if new name conflicts with existing secrets
        let existing_secret = secrets::Entity::find()
            .filter(secrets::Column::AccountId.eq(account_uuid))
            .filter(secrets::Column::SecretName.eq(new_name))
            .filter(secrets::Column::SecretId.ne(secret_uuid))
            .filter(secrets::Column::Archived.eq(false))
            .one(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if existing_secret.is_some() {
            return Err(StatusCode::CONFLICT);
        }

        secret_update.secret_name = Set(new_name.clone());
    }

    // Update secret value if provided
    if let Some(new_value) = &request.secret_value {
        let (encrypted_data, nonce) = encrypt_secret(&*state.db, new_value)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        secret_update.secret_value_encrypted = Set(encrypted_data);
        secret_update.nonce = Set(nonce);
        updated_value = Some(new_value.clone());
    }

    // Update description if provided
    if let Some(new_description) = &request.description {
        secret_update.description = Set(Some(new_description.clone()));
    }

    secret_update.updated_by = Set(Some(user_id));

    let updated_secret = secret_update.update(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get the actual secret value to return
    let secret_value = if let Some(value) = updated_value {
        value
    } else {
        decrypt_secret(&*state.db, &updated_secret.secret_value_encrypted, &updated_secret.nonce)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let response = SecretResponse {
        secret_id: updated_secret.secret_id.to_string(),
        account_id: updated_secret.account_id.to_string(),
        secret_name: updated_secret.secret_name,
        secret_value,
        description: updated_secret.description,
        is_api_key: updated_secret.is_api_key,
        archived: updated_secret.archived,
        created_at: updated_secret.created_at.map(|dt| dt.to_rfc3339()),
        updated_at: updated_secret.updated_at.map(|dt| dt.to_rfc3339()),
    };

    Ok(ResponseJson(response))
}

/// Delete a secret (mark as archived)
pub async fn delete_secret(
    State(state): State<Arc<AppState>>,
    Path((account_id, secret_id)): Path<(String, String)>,
    AuthClaims(claims): AuthClaims, // Extracted by middleware
) -> Result<ResponseJson<MessageResponse>, StatusCode> {
    let account_uuid = Uuid::parse_str(&account_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let secret_uuid = Uuid::parse_str(&secret_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: Verify user has access to this account

    let secret = secrets::Entity::find_by_id(secret_uuid)
        .filter(secrets::Column::AccountId.eq(account_uuid))
        .filter(secrets::Column::Archived.eq(false))
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Mark as archived
    let mut secret_update: secrets::ActiveModel = secret.into();
    secret_update.archived = Set(true);
    secret_update.updated_by = Set(Some(user_id));

    secret_update.update(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(MessageResponse {
        message: "Secret deleted successfully".to_string(),
    }))
}

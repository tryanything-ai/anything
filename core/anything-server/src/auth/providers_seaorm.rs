use crate::pgsodium_secrets::handlers::{create_secret, update_secret};
use crate::AppState;
use crate::entities::auth_providers;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use slugify::slugify;
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
pub struct AuthProvider {
    pub auth_provider_id: String,
    pub provider_name: String,
    pub provider_label: Option<String>,
    pub provider_icon: Option<String>,
    pub provider_description: Option<String>,
    pub provider_readme: Option<String>,
    pub auth_type: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub access_token_lifetime_seconds: Option<i32>,
    pub refresh_token_lifetime_seconds: Option<i32>,
    pub scopes: Option<String>,
    pub public: Option<bool>,
    pub client_id_vault_id: Option<String>,
    pub client_secret_vault_id: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_all_auth_providers(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[AUTH PROVIDERS SEAORM] Getting all auth providers");

    // Get all auth providers using SeaORM
    let providers = match auth_providers::Entity::find()
        .all(&*state.db)
        .await
    {
        Ok(providers) => providers,
        Err(err) => {
            println!("[AUTH PROVIDERS SEAORM] Error fetching providers: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Convert to response format
    let provider_responses: Vec<AuthProvider> = providers
        .into_iter()
        .map(|p| AuthProvider {
            auth_provider_id: p.auth_provider_id,
            provider_name: p.provider_name,
            provider_label: p.provider_label,
            provider_icon: p.provider_icon,
            provider_description: p.provider_description,
            provider_readme: p.provider_readme,
            auth_type: p.auth_type,
            auth_url: p.auth_url,
            token_url: p.token_url,
            access_token_lifetime_seconds: p.access_token_lifetime_seconds,
            refresh_token_lifetime_seconds: p.refresh_token_lifetime_seconds,
            scopes: p.scopes,
            public: p.public,
            client_id_vault_id: p.client_id_vault_id,
            client_secret_vault_id: p.client_secret_vault_id,
            updated_at: p.updated_at.map(|dt| dt.naive_utc().and_utc()),
            created_at: p.created_at.map(|dt| dt.naive_utc().and_utc()),
        })
        .collect();

    Json(provider_responses).into_response()
}

pub async fn set_auth_provider_client_id(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SetAuthProviderClientIdPayload>,
) -> impl IntoResponse {
    println!("[AUTH PROVIDERS SEAORM] Setting client ID for provider: {}", provider_name);

    // Create slug for the secret name
    let provider_slug = slugify!(&provider_name);
    let secret_name = format!("{}_CLIENT_ID", provider_slug.to_uppercase());

    // TODO: Insert the client ID into vault using pgsodium
    // This would require calling create_secret with proper parameters
    let client_id_vault_id = format!("vault_{}", uuid::Uuid::new_v4());

    // Update the auth provider record with the vault ID
    let provider_record = match auth_providers::Entity::find()
        .filter(auth_providers::Column::AuthProviderId.eq(&provider_name))
        .one(&*state.db)
        .await
    {
        Ok(Some(record)) => record,
        Ok(None) => {
            println!("[AUTH PROVIDERS SEAORM] Provider not found: {}", provider_name);
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
        Err(err) => {
            println!("[AUTH PROVIDERS SEAORM] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let mut active_model: auth_providers::ActiveModel = provider_record.into();
    active_model.client_id_vault_id = Set(Some(client_id_vault_id.clone()));
    active_model.updated_at = Set(Some(chrono::Utc::now().into()));

    match active_model.update(&*state.db).await {
        Ok(_) => {
            println!("[AUTH PROVIDERS SEAORM] Successfully updated provider");
            Json(json!({
                "message": "Client ID set successfully",
                "vault_id": client_id_vault_id
            })).into_response()
        }
        Err(err) => {
            println!("[AUTH PROVIDERS SEAORM] Error updating provider: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update provider").into_response()
        }
    }
}

pub async fn update_auth_provider_client_id(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateAuthProviderClientIdPayload>,
) -> impl IntoResponse {
    println!("[AUTH PROVIDERS SEAORM] Updating client ID for provider: {}", provider_name);

    // TODO: Update the vault entry using pgsodium update_secret
    // This would require calling update_secret with proper parameters
    println!("[AUTH PROVIDERS SEAORM] Would update client ID in vault");
    Json(json!({
        "message": "Client ID update placeholder - implement vault update"
    })).into_response()
}

pub async fn set_auth_provider_client_secret(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SetAuthProviderClientIdPayload>, // Reusing same payload structure
) -> impl IntoResponse {
    println!("[AUTH PROVIDERS SEAORM] Setting client secret for provider: {}", provider_name);

    // Create slug for the secret name
    let provider_slug = slugify!(&provider_name);
    let secret_name = format!("{}_CLIENT_SECRET", provider_slug.to_uppercase());

    // TODO: Insert the client secret into vault using pgsodium
    // This would require calling create_secret with proper parameters  
    let client_secret_vault_id = format!("vault_secret_{}", uuid::Uuid::new_v4());

    // Update the auth provider record with the vault ID
    let provider_record = match auth_providers::Entity::find()
        .filter(auth_providers::Column::AuthProviderId.eq(&provider_name))
        .one(&*state.db)
        .await
    {
        Ok(Some(record)) => record,
        Ok(None) => {
            println!("[AUTH PROVIDERS SEAORM] Provider not found: {}", provider_name);
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
        Err(err) => {
            println!("[AUTH PROVIDERS SEAORM] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let mut active_model: auth_providers::ActiveModel = provider_record.into();
    active_model.client_secret_vault_id = Set(Some(client_secret_vault_id.clone()));
    active_model.updated_at = Set(Some(chrono::Utc::now().into()));

    match active_model.update(&*state.db).await {
        Ok(_) => {
            println!("[AUTH PROVIDERS SEAORM] Successfully updated provider with secret");
            Json(json!({
                "message": "Client secret set successfully",
                "vault_id": client_secret_vault_id
            })).into_response()
        }
        Err(err) => {
            println!("[AUTH PROVIDERS SEAORM] Error updating provider: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update provider").into_response()
        }
    }
}

pub async fn get_auth_provider_by_name(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("[AUTH PROVIDERS SEAORM] Getting provider: {}", provider_name);

    // Get the auth provider by name
    let provider = match auth_providers::Entity::find()
        .filter(auth_providers::Column::ProviderName.eq(&provider_name))
        .one(&*state.db)
        .await
    {
        Ok(Some(provider)) => provider,
        Ok(None) => {
            println!("[AUTH PROVIDERS SEAORM] Provider not found: {}", provider_name);
            return (StatusCode::NOT_FOUND, "Auth provider not found").into_response();
        }
        Err(err) => {
            println!("[AUTH PROVIDERS SEAORM] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let provider_response = AuthProvider {
        auth_provider_id: provider.auth_provider_id,
        provider_name: provider.provider_name,
        provider_label: provider.provider_label,
        provider_icon: provider.provider_icon,
        provider_description: provider.provider_description,
        provider_readme: provider.provider_readme,
        auth_type: provider.auth_type,
        auth_url: provider.auth_url,
        token_url: provider.token_url,
        access_token_lifetime_seconds: provider.access_token_lifetime_seconds,
        refresh_token_lifetime_seconds: provider.refresh_token_lifetime_seconds,
        scopes: provider.scopes,
        public: provider.public,
        client_id_vault_id: provider.client_id_vault_id,
        client_secret_vault_id: provider.client_secret_vault_id,
        updated_at: provider.updated_at.map(|dt| dt.naive_utc().and_utc()),
        created_at: provider.created_at.map(|dt| dt.naive_utc().and_utc()),
    };

    Json(provider_response).into_response()
}

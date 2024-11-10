use crate::auth;
use crate::auth::init::AccountAuthProviderAccount;
use crate::workflow_types::Task;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;

use crate::templater::Templater;
use uuid::Uuid;

// Fake account data for testing purposes
pub async fn get_fake_account_auth_provider_account(
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    let fake_account = AccountAuthProviderAccount {
        account_auth_provider_account_id: Uuid::new_v4(),
        account_id: Uuid::new_v4(),
        access_token_vault_id: "airtable_access_token".to_string(),
        refresh_token_vault_id: "airtable_refresh_token".to_string(),
        auth_provider: Some(serde_json::json!({
            "auth_provider_id": "airtable",
            "provider_name": "airtable",
            "provider_label": "airtable",
            "provider_icon": "<svg>...</svg>",
            "provider_description": "Connect with your airtable account",
            "provider_readme": "Internal notes for managing airtable connection",
            "auth_type": "oauth2",
            "auth_url": "https://accounts.airtable.com/o/oauth2/auth",
            "token_url": "https://oauth2.airtableapis.com/token",
            "provider_data": {},
            "access_token_lifetime_seconds": "3600",
            "refresh_token_lifetime_seconds": "2592000",
            "redirect_url": "https://example.com/auth/callback",
            "client_id": "your_client_id",
            "client_secret": "your_client_secret",
            "scopes": "email profile",
            "public": false
        })),
        auth_provider_id: "airtable".to_string(),
        account_auth_provider_account_label: "My airtable Account".to_string(),
        account_auth_provider_account_slug: "airtable".to_string(),
        account_data: Some(serde_json::json!({
            "email": "user@example.com",
            "name": "Test User"
        })),
        access_token: "fake_access_token".to_string(),
        access_token_expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        refresh_token: Some("fake_refresh_token".to_string()),
        refresh_token_expires_at: Some(chrono::Utc::now() + chrono::Duration::days(30)),
        updated_at: Some(chrono::Utc::now()),
        created_at: Some(chrono::Utc::now()),
        updated_by: Some(Uuid::new_v4()),
        created_by: Some(Uuid::new_v4()),
    };

    Ok(vec![fake_account])
}
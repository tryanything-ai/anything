use dotenv::dotenv;
// use postgrest::Postgrest; // Removed - using pgsodium_secrets instead
use std::{env, sync::Arc, time::Duration};

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::bundler::secrets::secrets_cache::SecretsCache;
use crate::AppState;

pub mod secrets_cache;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptedSecret {
    pub secret_id: Uuid,
    pub secret_name: String,
    pub secret_value: String,
    pub secret_description: Option<String>,
}

pub async fn get_decrypted_secrets(
    state: Arc<AppState>,
    account_id: &str,
) -> Result<Vec<DecryptedSecret>, Box<dyn std::error::Error + Send + Sync>> {
    // Try to get from cache first
    if let Some(cache_entry) = state.bundler_secrets_cache.get(account_id) {
        if let Some(cached_secrets) = cache_entry.get(account_id) {
            println!(
                "[BUNDLER] Using cached secrets for account_id: {}",
                account_id
            );
            return Ok(cached_secrets);
        }
    }

    println!(
        "[BUNDLER] Cache miss for secrets, fetching from DB for account_id: {}",
        account_id
    );

    // If not in cache, fetch from DB
    let secrets = fetch_secrets_from_vault(state.clone(), account_id).await?;

    // Update cache - get or create cache for this account
    let cache = state
        .bundler_secrets_cache
        .entry(account_id.to_string())
        .or_insert_with(|| SecretsCache::new(Duration::from_secs(86400)));

    cache.set(account_id, secrets.clone());
    println!(
        "[BUNDLER] Updated secrets cache for account_id: {}",
        account_id
    );

    Ok(secrets)
}

// Secrets for building context with API KEYS - now uses pgsodium_secrets
pub async fn fetch_secrets_from_vault(
    state: Arc<AppState>,
    account_id: &str,
) -> Result<Vec<DecryptedSecret>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[BUNDLER] Attempting to get decrypted secrets for account_id: {}",
        account_id
    );

    // TODO: Replace with proper pgsodium_secrets SeaORM query
    // For now, return empty vec as placeholder
    let items: Vec<DecryptedSecret> = Vec::new();

    println!(
        "[BUNDLER] Successfully retrieved {} decrypted secrets",
        items.len()
    );

    Ok(items)
}

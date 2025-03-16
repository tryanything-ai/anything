use dotenv::dotenv;
use postgrest::Postgrest;
use std::{env, sync::Arc};

use uuid::Uuid;

use serde::{Deserialize, Serialize};

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
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<DecryptedSecret>, Box<dyn std::error::Error + Send + Sync>> {
    // Try to get from cache first using a read lock
    {
        let cache = state.bundler_secrets_cache.read().await;
        if let Some(cached_secrets) = cache.get(account_id) {
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
    let secrets = fetch_secrets_from_vault(client, account_id).await?;

    // Update cache with a write lock
    {
        let mut cache = state.bundler_secrets_cache.write().await;
        cache.set(account_id, secrets.clone());
        println!(
            "[BUNDLER] Updated secrets cache for account_id: {}",
            account_id
        );
    }

    Ok(secrets)
}

// Secrets for building context with API KEYS
pub async fn fetch_secrets_from_vault(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<DecryptedSecret>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Attempting to get decrypted secrets for account_id: {}",
        account_id
    );

    let input = serde_json::json!({
        "team_account_id": account_id.to_string()
    })
    .to_string();

    let response = client
        .rpc("get_decrypted_secrets", &input)
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await?;

    println!(
        "[BUNDLER] Response for get_decryped_secrets: {:?}",
        response
    );

    let body = response.text().await?;
    let items: Vec<DecryptedSecret> = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("[BUNDLER] Error parsing decrypted secrets: {}", e);
            println!("[BUNDLER] Response body: {}", body);
            return Err(Box::new(e));
        }
    };

    println!(
        "[BUNDLER] Successfully retrieved {} decrypted secrets",
        items.len()
    );

    Ok(items)
}

use crate::auth::init::AccountAuthProviderAccount;
use crate::bundler::accounts::accounts_cache::AccountsCache;
use crate::AppState;
use chrono::Utc;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde_json::json;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

pub mod accounts_cache;

use crate::auth::refresh::refresh_accounts;

use std::error::Error;

pub async fn fetch_cached_auth_accounts(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
    refresh_auth: bool,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn Error + Send + Sync>> {
    println!("[FAST AUTH ACCOUNTS] Fetching cached auth accounts");

    //Check if accounts are in the the cache
    let mut accounts: Vec<AccountAuthProviderAccount> = {
        if let Some(cache_entry) = state.bundler_accounts_cache.get(account_id) {
            cache_entry.get(account_id).unwrap_or_default()
        } else {
            Vec::new()
        }
    };

    //If not, fetch them from the DB
    if accounts.is_empty() {
        println!("[FAST AUTH ACCOUNTS] No cached accounts found, fetching from DB");
        accounts = fetch_accounts_from_db(client, account_id).await?;
    }

    //If caller needs up to date info
    //Check if cached accounts need to have access_token refreshed
    if refresh_auth {
        let now = Utc::now();
        let expiry_threshold = now + chrono::Duration::minutes(5);
        let needs_refresh = accounts.iter().any(|account| {
            !account.failed
                && account
                    .access_token_expires_at
                    .map(|expires_at| expires_at <= expiry_threshold)
                    .unwrap_or(false)
        });

        if !needs_refresh {
            println!("[FAST AUTH ACCOUNTS] Cached accounts do not need refresh");
        } else {
            println!("[FAST AUTH ACCOUNTS] Cached accounts need to have access_token refreshed");
            accounts = refresh_accounts(client, accounts).await?;
        }
    }

    //Update the cache
    let cache = state
        .bundler_accounts_cache
        .entry(account_id.to_string())
        .or_insert_with(|| AccountsCache::new(Duration::from_secs(86400)));

    cache.set(account_id, accounts.clone());

    Ok(accounts)
}

async fn fetch_accounts_from_db(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Fetching auth accounts from DB for account_id: {}",
        account_id
    );

    let response = client
        .rpc(
            "get_decrypted_account_and_provider",
            json!({"p_account_id": account_id}).to_string(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await?;

    let body = response.text().await?;

    // First check if we got an error response from the database
    if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(&body) {
        if let Some(error_message) = error_response.get("message") {
            debug!("[BUNDLER] Database error: {}", error_message);
            return Err(format!("Database error: {}", error_message).into());
        }
    }

    // If no error, try to parse as accounts
    let accounts: Vec<AccountAuthProviderAccount> = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(e) => {
            debug!("[BUNDLER] Error parsing auth accounts: {}", e);
            debug!("[BUNDLER] Response body: {}", body);
            return Err(Box::new(e));
        }
    };

    println!(
        "[BUNDLER] Successfully retrieved {} auth accounts from DB",
        accounts.len()
    );

    Ok(accounts)
}

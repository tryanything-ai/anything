use crate::auth::init::AccountAuthProviderAccount;
use crate::AppState;
use chrono::Utc;
use postgrest::Postgrest;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use serde_json::json;
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
        let cache = state.bundler_accounts_cache.read().await;
        let accounts = cache.get(account_id).unwrap_or_default();
        accounts
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
    {
        let mut cache = state.bundler_accounts_cache.write().await;
        cache.set(account_id, accounts.clone());
    }

    Ok(accounts)
}


async fn fetch_accounts_from_db(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    debug!(
        "[BUNDLER] Fetching auth accounts from DB for account_id: {}",
        account_id
    );

    let response = client
        .rpc(
            "get_account_auth_provider_accounts",
            json!({"p_account_id": account_id}).to_string(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await?;

    let body = response.text().await?;
    let accounts: Vec<AccountAuthProviderAccount> = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(e) => {
            debug!("[BUNDLER] Error parsing auth accounts: {}", e);
            debug!("[BUNDLER] Response body: {}", body);
            return Err(Box::new(e));
        }
    };

    debug!(
        "[BUNDLER] Successfully retrieved {} auth accounts from DB",
        accounts.len()
    );

    Ok(accounts)
}


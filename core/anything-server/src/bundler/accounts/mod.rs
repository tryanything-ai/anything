use crate::auth::init::AccountAuthProviderAccount;
use crate::{auth, AppState};
use dotenv::dotenv;
use postgrest::Postgrest;
use serde_json::json;
use std::env;
use std::sync::Arc;
use tracing::debug;

pub mod accounts_cache;

pub async fn get_refreshed_auth_accounts(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    debug!(
        "[BUNDLER] Starting auth account refresh for account_id: {}",
        account_id
    );

    // Refresh accounts in DB - do this outside of any locks
    let refreshed_accounts = refresh_accounts_in_db(client, account_id).await?;

    // Update cache with a write lock
    {
        let mut cache = state.bundler_accounts_cache.write().await;
        cache.set(account_id, refreshed_accounts.clone());
        debug!(
            "[BUNDLER] Updated accounts cache after refresh for account_id: {}",
            account_id
        );
    }

    Ok(refreshed_accounts)
}

async fn refresh_accounts_in_db(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    debug!(
        "[BUNDLER] Refreshing auth accounts in DB for account_id: {}",
        account_id
    );

    let accounts = auth::refresh::refresh_accounts(client, account_id).await?;

    debug!(
        "[BUNDLER] Successfully refreshed {} auth accounts in DB",
        accounts.len()
    );

    // Log individual account details at debug level
    for account in &accounts {
        debug!(
            "[BUNDLER] Refreshed account: {} (provider: {})",
            account.account_auth_provider_account_slug, account.auth_provider_id
        );
    }

    Ok(accounts)
}

pub async fn get_auth_accounts(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    // Try to get from cache first using a read lock
    {
        let cache = state.bundler_accounts_cache.read().await;
        if let Some(cached_accounts) = cache.get(account_id) {
            debug!(
                "[BUNDLER] Using cached auth accounts for account_id: {}",
                account_id
            );
            return Ok(cached_accounts);
        }
    }

    debug!(
        "[BUNDLER] Cache miss for auth accounts, fetching from DB for account_id: {}",
        account_id
    );

    // If not in cache, fetch from DB
    let accounts = fetch_accounts_from_db(client, account_id).await?;

    // Update cache with a write lock
    {
        let mut cache = state.bundler_accounts_cache.write().await;
        cache.set(account_id, accounts.clone());
        debug!(
            "[BUNDLER] Updated auth accounts cache for account_id: {}",
            account_id
        );
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
// pub async fn get_refreshed_auth_accounts(
//     accounts_cache: &mut AccountsCache,
//     client: &Postgrest,
//     account_id: &str,
// ) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
//     println!(
//         "[BUNDLER] Refreshing auth accounts for account_id: {}",
//         account_id
//     );

//     let accounts = auth::refresh::refresh_accounts(client, account_id).await?;

//     println!(
//         "[BUNDLER] Successfully refreshed {} auth accounts",
//         accounts.len()
//     );

//     Ok(accounts)
// }

// pub async fn get_auth_accounts(
//     accounts_cache: &mut AccountsCache,
//     client: &Postgrest,
//     account_id: &str,
// ) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
//     dotenv().ok();
//     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

//     println!(
//         "[BUNDLER] Fetching auth accounts for account_id: {}",
//         account_id
//     );

//     let response = client
//         .rpc(
//             "get_account_auth_provider_accounts",
//             json!({"p_account_id": account_id}).to_string(),
//         )
//         .auth(supabase_service_role_api_key.clone())
//         .execute()
//         .await?;

//     let body = response.text().await?;
//     let accounts: Vec<AccountAuthProviderAccount> = serde_json::from_str(&body)?;

//     println!("[BUNDLER] Retrieved {} auth accounts", accounts.len());

//     Ok(accounts)
// }

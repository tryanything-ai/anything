use crate::auth;
use crate::auth::init::AccountAuthProviderAccount;
use dotenv::dotenv;
use postgrest::Postgrest;
use serde_json::json;
use std::env;

pub mod accounts_cache;

pub async fn get_refreshed_auth_accounts(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[BUNDLER] Refreshing auth accounts for account_id: {}",
        account_id
    );

    let accounts = auth::refresh::refresh_accounts(client, account_id).await?;

    println!(
        "[BUNDLER] Successfully refreshed {} auth accounts",
        accounts.len()
    );

    Ok(accounts)
}

pub async fn get_auth_accounts(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    println!(
        "[BUNDLER] Fetching auth accounts for account_id: {}",
        account_id
    );

    let response = client
        .rpc(
            "get_account_auth_provider_accounts",
            json!({"p_account_id": account_id}).to_string(),
        )
        .auth(supabase_service_role_api_key.clone())
        .execute()
        .await?;

    let body = response.text().await?;
    let accounts: Vec<AccountAuthProviderAccount> = serde_json::from_str(&body)?;

    println!("[BUNDLER] Retrieved {} auth accounts", accounts.len());

    Ok(accounts)
}

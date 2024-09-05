use axum::http::StatusCode;
use postgrest::Postgrest;
use serde_json::Value;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

use std::env;

use crate::auth::init::{AccountAuthProviderAccount, AuthProvider, ErrorResponse, OAuthToken};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountAuthProviderAccount {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
}

pub async fn refresh_accounts(
    client: &Postgrest,
    account_id: &str,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    println!(
        "[AUTH REFRESH] Starting refresh_accounts for account_id: {}",
        account_id
    );

    let response = client
        .from("account_auth_provider_accounts")
        .auth(supabase_service_role_api_key.clone())
        .select("*, auth_provider:auth_providers(*)")
        .eq("account_id", account_id)
        .execute()
        .await?;

    println!("[AUTH REFRESH] Received response from database");

    let body = response.text().await?;
    // println!("[AUTH REFRESH] Response body: {}", body);

    let accounts: Vec<AccountAuthProviderAccount> = serde_json::from_str(&body)?;
    // println!("[AUTH REFRESH] Parsed accounts: {:?}", accounts);

    for account in &accounts {
        println!("[AUTH REFRESH] Processing account: {:?}", account.auth_provider_id);

        let auth_provider: AuthProvider = match &account.auth_provider {
            Some(value) => serde_json::from_value(value.clone())?,
            None => {
                println!(
                    "[AUTH REFRESH] No auth_provider found for account: {:?}",
                    account.auth_provider_id
                );
                continue; // or handle the None case appropriately
            }
        };

        if let Some(expires_at) = account.access_token_expires_at {
            let now = Utc::now();
            let expiry_threshold = now + chrono::Duration::minutes(5);

            println!(
                "[AUTH REFRESH] Current time: {}, ACCESS_TOKEN expiry time: {}, Threshold: {}",
                now, expires_at, expiry_threshold
            );

            if expires_at < expiry_threshold {
                println!(
                    "[AUTH REFRESH] Token is about to expire or has expired for account: {:?}",
                    account.account_id
                );

                match refresh_access_token(
                    &auth_provider,
                    &account.refresh_token.clone().unwrap_or_default(),
                )
                .await
                {
                    Ok(new_token) => {
                        println!(
                            "[AUTH REFRESH] Successfully refreshed token: {:?}",
                            new_token
                        );
                        

                        let mut access_token_expires_at = None;
                        if let Some(access_token_lifespan) =
                            auth_provider.access_token_lifetime_seconds
                        {
                            let access_token_lifespan: i64 =
                                access_token_lifespan.parse().unwrap_or(0);
                            access_token_expires_at =
                                Some(Utc::now() + chrono::Duration::seconds(access_token_lifespan));
                            println!(
                                "[AUTH REFRESH] Updated access_token_expires_at: {:?}",
                                access_token_expires_at
                            );
                        }

                        let mut refresh_token_expires_at = None;
                        if let Some(refresh_token_lifespan) =
                            auth_provider.refresh_token_lifetime_seconds
                        {
                            let refresh_token_lifespan: i64 =
                                refresh_token_lifespan.parse().unwrap_or(0);
                            refresh_token_expires_at = Some(
                                Utc::now() + chrono::Duration::seconds(refresh_token_lifespan),
                            );
                            println!(
                                "[AUTH REFRESH] Updated refresh_token_expires_at: {:?}",
                                refresh_token_expires_at
                            );
                        }

                        let account_updates = UpdateAccountAuthProviderAccount {
                            access_token: new_token.access_token,
                            refresh_token: new_token.refresh_token,
                            access_token_expires_at,
                            refresh_token_expires_at,
                        };

                        println!(
                            "[AUTH REFRESH] Updated account with new token data: {:?}",
                            account_updates
                        );

                        // Optionally, update the account in the database
                        let update_response = client
                            .from("account_auth_provider_accounts")
                            .auth(supabase_service_role_api_key.clone())
                            .update(serde_json::to_string(&account_updates).unwrap())
                            .eq(
                                "account_auth_provider_account_id",
                                account.account_auth_provider_account_id.to_string(),
                            )
                            .execute()
                            .await;

                        if let Err(e) = update_response {
                            println!(
                                "[AUTH REFRESH] Failed to update account with new token: {:?}",
                                e
                            );
                        } else {
                            println!("[AUTH REFRESH] Successfully updated account with new token");
                        }
                    }
                    Err((status, msg)) => {
                        println!(
                            "[AUTH REFRESH] Failed to refresh access token: Status: {:?}, Message: {:?}",
                            status, msg
                        );
                    }
                }
            } else {
                println!(
                    "[AUTH REFRESH] Token is still valid for account: {:?}",
                    account.auth_provider_id
                );
            }
        } else {
            println!(
                "[AUTH REFRESH] No access_token_expires_at found for account: {:?}",
                account.auth_provider_id
            );
        }
    }
    //TODO: if any accounts refresh fails it will kill all the automations in the whole system
    //fetch all the newly refreshed accounts
    let new_response = client
        .from("account_auth_provider_accounts")
        .auth(supabase_service_role_api_key.clone())
        .select("*, auth_provider:auth_providers(*)")
        .eq("account_id", account_id)
        .execute()
        .await?;

    let new_body = new_response.text().await?;
    let new_accounts: Vec<AccountAuthProviderAccount> = serde_json::from_str(&new_body)?;

    Ok(new_accounts)
}

pub async fn refresh_access_token(
    auth_provider: &AuthProvider,
    refresh_token: &str,
) -> Result<OAuthToken, (StatusCode, String)> {
    let client = Client::new();

    let request = client
        .post(&auth_provider.token_url)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");

    // Add Authorization header if client_secret is present
    // if let Some(client_secret) = &auth_provider.client_secret {
    //     let credentials = format!("{}:{}", auth_provider.client_id, client_secret);
    //     let encoded_credentials =
    //         base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(credentials);
    //     request = request.header(
    //         header::AUTHORIZATION,
    //         format!("Basic {}", encoded_credentials),
    //     );
    // }

    let form_params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", &auth_provider.client_id),
    ];

    println!("[AUTH REFRESH] Refresh token exchange form_params: {:?}", form_params);

    let response = request
        .form(&form_params)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status = response.status();
    println!("[AUTH REFRESH] Refresh token response status: {:?}", status);

    let body = response.text().await.map_err(|e| {
        println!("[AUTH REFRESH] Error reading refresh token response body: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    println!("[AUTH REFRESH] Refresh token response body: {:?}", body);

    if status.is_success() {
        let token: Value = serde_json::from_str(&body).map_err(|e| {
            println!("[AUTH REFRESH] Failed to parse refresh token response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse refresh token response: {}", e),
            )
        })?;

        let access_token = token["access_token"].as_str().unwrap_or("").to_string();
        let refresh_token = token["refresh_token"].as_str().map(|s| s.to_string());
        let expires_in = token["expires_in"].as_i64().unwrap_or(3600);
        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in);

        Ok(OAuthToken {
            access_token,
            refresh_token,
            expires_at: Some(expires_at),
        })
    } else {
        let error: ErrorResponse = serde_json::from_str(&body).map_err(|e| {
            println!("[AUTH REFRESH] Failed to parse refresh token error response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse refresh token error response: {}", e),
            )
        })?;

        let status_code = if error.error == "invalid_client" {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::BAD_REQUEST
        };

        println!(
            "[AUTH REFRESH] Returning refresh token error with status code: {:?}, description: {:?}",
            status_code, error.error_description
        );
        Err((status_code, error.error_description))
    }
}

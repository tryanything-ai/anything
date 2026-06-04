use axum::http::StatusCode;
use serde_json::Value;

use chrono::{DateTime, Utc};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::auth::init_seaorm::{AccountAuthProviderAccount, AuthProvider, ErrorResponse, OAuthToken};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountAuthProviderAccount {
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
}

pub async fn refresh_accounts(
    state: Arc<AppState>,
    accounts: Vec<AccountAuthProviderAccount>,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    let mut accounts = accounts;

    println!("[AUTH REFRESH] Parsed accounts: {:?}", accounts);

    for account in accounts.iter_mut() {
        println!(
            "[AUTH REFRESH] Processing account: {:?}",
            account.auth_provider_id
        );

        // Skip if failure retries exceeds 3
        if account.failure_retries >= 3 {
            println!(
                "[AUTH REFRESH] Skipping account due to too many failures: {:?}",
                account.auth_provider_id
            );
            continue;
        }

        let auth_provider: AuthProvider = match &account.auth_provider {
            Some(value) => serde_json::from_value(value.clone())?,
            None => {
                println!(
                    "[AUTH REFRESH] No auth_provider found for account: {:?}",
                    account.auth_provider_id
                );

                // TODO: Update account as failed using SeaORM
                println!("[AUTH REFRESH] TODO: Mark account as failed in database");
                
                // Update the in-memory account with failure info
                account.failed = true;
                account.failed_at = if account.failure_retries == 0 {
                    Some(Utc::now())
                } else {
                    account.failed_at
                };
                account.failed_reason = Some("Service not supported".to_string());
                account.failure_retries += 1;
                account.failed_at = Some(Utc::now());

                continue;
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

                        // Calculate new expiry times if provided in auth provider config
                        let mut access_token_expires_at = None;
                        // TODO: Get access token lifetime from vault configuration
                        // For now, default to 1 hour
                        access_token_expires_at = Some(Utc::now() + chrono::Duration::seconds(3600));
                        println!(
                            "[AUTH REFRESH] Updated access_token_expires_at: {:?}",
                            access_token_expires_at
                        );

                        // TODO: Get refresh token lifetime from vault configuration  
                        // For now, default to 30 days
                        let mut refresh_token_expires_at = Some(Utc::now() + chrono::Duration::days(30));
                        println!(
                            "[AUTH REFRESH] Updated refresh_token_expires_at: {:?}",
                            refresh_token_expires_at
                        );

                        // TODO: Update tokens in pgsodium secrets using SeaORM
                        println!("[AUTH REFRESH] TODO: Update access token in pgsodium secrets");
                        
                        if let Some(_new_refresh_token) = &new_token.refresh_token {
                            println!("[AUTH REFRESH] TODO: Update refresh token in pgsodium secrets");
                            // Update refresh token expiry only if we got a new refresh token
                            refresh_token_expires_at = refresh_token_expires_at;
                        } else {
                            // Keep existing refresh token expiry if no new refresh token
                            refresh_token_expires_at = account.refresh_token_expires_at;
                        }

                        let account_updates = UpdateAccountAuthProviderAccount {
                            access_token_expires_at,
                            refresh_token_expires_at,
                        };

                        println!(
                            "[AUTH REFRESH] Updated account with new token data: {:?}",
                            account_updates
                        );

                        // TODO: Update the account in the database using SeaORM
                        println!("[AUTH REFRESH] TODO: Update account auth provider account in database");
                        
                        // Update the in-memory account with new values
                        account.access_token = new_token.access_token;
                        if let Some(new_refresh_token) = new_token.refresh_token {
                            account.refresh_token = Some(new_refresh_token);
                        }
                        account.access_token_expires_at = access_token_expires_at;
                        account.refresh_token_expires_at = refresh_token_expires_at;
                    }
                    Err((status, msg)) => {
                        // TODO: Update account as failed using SeaORM
                        println!("[AUTH REFRESH] TODO: Mark account as failed in database");
                        
                        // Update the in-memory account with failure info
                        account.failed = true;
                        account.failed_at = if account.failure_retries == 0 {
                            Some(Utc::now())
                        } else {
                            account.failed_at
                        };
                        account.failed_reason = Some(format!(
                            "Failed to refresh token: Status: {}, Message: {}",
                            status, msg
                        ));
                        account.failure_retries += 1;
                        account.failed_at = Some(Utc::now());
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

    Ok(accounts)
}

pub async fn refresh_access_token(
    auth_provider: &AuthProvider,
    refresh_token: &str,
) -> Result<OAuthToken, (StatusCode, String)> {
    let client = Client::new();

    let token_url = auth_provider.token_url.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Token URL not configured".to_string(),
    ))?;

    let request = client
        .post(token_url)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");

    // TODO: Get client_id and client_secret from vault using auth_provider.client_id_vault_id
    let client_id = "placeholder_client_id"; // Would need to decrypt from vault
    
    let mut form_params = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
    ];

    // TODO: Get client_secret from vault if configured
    // if let Some(client_secret) = &auth_provider.client_secret {
    //     form_params.push(("client_secret", client_secret));
    // }

    println!(
        "[AUTH REFRESH] Refresh token exchange form_params: {:?}",
        form_params
    );

    let response = request
        .form(&form_params)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status = response.status();
    println!("[AUTH REFRESH] Refresh token response status: {:?}", status);

    let body = response.text().await.map_err(|e| {
        println!(
            "[AUTH REFRESH] Error reading refresh token response body: {:?}",
            e
        );
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    println!("[AUTH REFRESH] Refresh token response body: {:?}", body);

    if status.is_success() {
        let token: Value = serde_json::from_str(&body).map_err(|e| {
            println!(
                "[AUTH REFRESH] Failed to parse refresh token response: {:?}",
                e
            );
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
            token_type: "Bearer".to_string(),
            expires_in: Some(3600), // Default to 1 hour
            refresh_token,
            scope: None,
        })
    } else {
        let error: ErrorResponse = serde_json::from_str(&body).map_err(|e| {
            println!(
                "[AUTH REFRESH] Failed to parse refresh token error response: {:?}",
                e
            );
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
        Err((status_code, error.error_description.unwrap_or_else(|| error.error.clone())))
    }
}

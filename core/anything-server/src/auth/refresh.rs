use axum::http::StatusCode;
use postgrest::Postgrest;
use serde_json::Value;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

use crate::auth::init::{AccountAuthProviderAccount, AuthProvider, ErrorResponse, OAuthToken};

use crate::vault::update_secret_in_vault;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountAuthProviderAccount {
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
}

pub async fn refresh_accounts(
    client: &Postgrest,
    accounts: Vec<AccountAuthProviderAccount>,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

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

                let failed_updates = json!({
                    "failed": true,
                    "failed_at": if account.failure_retries == 0 { Some(chrono::Utc::now()) } else { account.failed_at },
                    "failed_reason": "Service not supported",
                    "failure_retries": account.failure_retries + 1,
                    "last_failure_retry": chrono::Utc::now(),
                });

                let failed_response = client
                    .from("account_auth_provider_accounts")
                    .auth(supabase_service_role_api_key.clone())
                    .update(failed_updates.to_string())
                    .eq(
                        "account_auth_provider_account_id",
                        account.account_auth_provider_account_id.to_string(),
                    )
                    .execute()
                    .await;

                if let Err(e) = failed_response {
                    println!("[AUTH REFRESH] Failed to mark account as failed: {:?}", e);
                } else {
                    println!("[AUTH REFRESH] Successfully marked account as failed");
                    // Update the in-memory account with failure info
                    account.failed = true;
                    account.failed_at = if account.failure_retries == 0 {
                        Some(Utc::now())
                    } else {
                        account.failed_at
                    };
                    account.failed_reason = Some("Service no supported".to_string());
                    account.failure_retries += 1;
                    account.last_failure_retry = Some(Utc::now());
                }

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

                        let refresh_token_value = new_token
                            .refresh_token
                            .as_deref()
                            .unwrap_or_default()
                            .to_string();

                        //parallel update of both tokens in the vault
                        let (access_result, refresh_result) = tokio::join!(
                            update_secret_in_vault(
                                client,
                                &account.access_token_vault_id,
                                &new_token.access_token,
                            ),
                            update_secret_in_vault(
                                client,
                                &account.refresh_token_vault_id,
                                &refresh_token_value,
                            )
                        );

                        access_result?;
                        refresh_result?;

                        let account_updates = UpdateAccountAuthProviderAccount {
                            access_token_expires_at,
                            refresh_token_expires_at,
                        };

                        println!(
                            "[AUTH REFRESH] Updated account with new token data: {:?}",
                            account_updates
                        );

                        // Update the account in the database
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
                            // Update the in-memory account with new values
                            account.access_token = new_token.access_token;
                            account.refresh_token = new_token.refresh_token;
                            account.access_token_expires_at = access_token_expires_at;
                            account.refresh_token_expires_at = refresh_token_expires_at;
                        }
                    }
                    Err((status, msg)) => {
                        let failed_updates = json!({
                            "failed": true,
                            "failed_at": if account.failure_retries == 0 { Some(chrono::Utc::now()) } else { account.failed_at },
                            "failed_reason": format!("Failed to refresh token: Status: {}, Message: {}", status, msg),
                            "failure_retries": account.failure_retries + 1,
                            "last_failure_retry": chrono::Utc::now(),
                        });

                        let failed_response = client
                            .from("account_auth_provider_accounts")
                            .auth(supabase_service_role_api_key.clone())
                            .update(failed_updates.to_string())
                            .eq(
                                "account_auth_provider_account_id",
                                account.account_auth_provider_account_id.to_string(),
                            )
                            .execute()
                            .await;

                        if let Err(e) = failed_response {
                            println!("[AUTH REFRESH] Failed to mark account as failed: {:?}", e);
                        } else {
                            println!("[AUTH REFRESH] Successfully marked account as failed");
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
                            account.last_failure_retry = Some(Utc::now());
                        }
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
            refresh_token,
            expires_at: Some(expires_at),
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
        Err((status_code, error.error_description))
    }
}

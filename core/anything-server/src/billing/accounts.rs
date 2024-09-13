use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;
use std::sync::Arc;
use stripe::{CreateCustomer, CreateSubscription, CreateSubscriptionItems, Customer, Subscription};

use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserByIdParams {
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub instance_id: Option<uuid::Uuid>,
    pub id: uuid::Uuid,
    pub aud: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
    pub encrypted_password: Option<String>,
    pub email_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub invited_at: Option<chrono::DateTime<chrono::Utc>>,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub recovery_token: Option<String>,
    pub recovery_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub email_change_token_new: Option<String>,
    pub email_change: Option<String>,
    pub email_change_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_sign_in_at: Option<chrono::DateTime<chrono::Utc>>,
    pub raw_app_meta_data: Option<serde_json::Value>,
    pub raw_user_meta_data: Option<serde_json::Value>,
    pub is_super_admin: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub phone: Option<String>,
    pub phone_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub phone_change: Option<String>,
    pub phone_change_token: Option<String>,
    pub phone_change_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub email_change_token_current: Option<String>,
    pub email_change_confirm_status: Option<i16>,
    pub banned_until: Option<chrono::DateTime<chrono::Utc>>,
    pub reauthentication_token: Option<String>,
    pub reauthentication_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_sso_user: bool,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_anonymous: bool,
}

// Define the input struct for the SQL function
#[derive(Debug, Serialize)]
struct UpsertCustomerSubscriptionInput {
    account_id: uuid::Uuid,
    customer: Option<serde_json::Value>,
    subscription: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebhookPayload<T> {
    #[serde(rename = "INSERT")]
    Insert {
        table: String,
        schema: String,
        record: T,
        old_record: Option<()>,
    },
    #[serde(rename = "UPDATE")]
    Update {
        table: String,
        schema: String,
        record: T,
        old_record: T,
    },
    #[serde(rename = "DELETE")]
    Delete {
        table: String,
        schema: String,
        record: Option<()>,
        old_record: T,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct TableRecord {
    id: uuid::Uuid,
    primary_owner_user_id: uuid::Uuid,
    name: Option<String>,
    slug: Option<String>,
    personal_account: bool,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    created_by: Option<uuid::Uuid>,
    updated_by: Option<uuid::Uuid>,
    private_metadata: serde_json::Value,
    public_metadata: serde_json::Value,
}

pub type NewAccountWebhookPayload = WebhookPayload<TableRecord>;

pub async fn handle_new_account_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewAccountWebhookPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    match payload {
        WebhookPayload::Insert { record, .. } => {
            println!(
                "New account created making stripe account now: {:?}",
                record.clone()
            );

            // Check if it's not a personal account
            if !record.personal_account {
                // Fetch user data from Supabase
                let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
                    .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

                let input = GetUserByIdParams {
                    user_id: record.primary_owner_user_id,
                };

                let user_response = match state
                    .anything_client
                    .rpc("get_user_by_id", serde_json::to_string(&input).unwrap())
                    .auth(supabase_service_role_api_key.clone())
                    .execute()
                    .await
                {
                    Ok(response) => {
                        println!("Response status: {:?}", response.status());
                        println!("Response headers: {:?}", response.headers());

                        if response.status().is_success() {
                            response
                        } else {
                            let status = response.status();
                            let error_body = response
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unable to read error body".to_string());
                            eprintln!("Error response body: {}", error_body);
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to fetch user data. Status: {}", status),
                            ));
                        }
                    }
                    Err(err) => {
                        eprintln!("Error fetching user data: {:?}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to fetch user data".to_string(),
                        ));
                    }
                };

                let user: User = match user_response.json().await {
                    Ok(user) => user,
                    Err(err) => {
                        eprintln!("Error parsing user response: {:?}", err);
                        // let response_text = user_response.text().await
                        //     .unwrap_or_else(|_| "Unable to read response body".to_string());
                        // eprintln!("Response body: {}", response_text);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to parse user response".to_string(),
                        ));
                    }
                };

                println!("User data for non-personal account: {:?}", user);

                // Handle the new account creation
                let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Stripe secret key not found".to_string(),
                    )
                })?;

                let client = stripe::Client::new(stripe_secret_key);

                // Create a new Stripe customer
                let customer = Customer::create(
                    &client,
                    CreateCustomer {
                        // name: Some("Alexander Lyon"),
                        email: Some(user.email.as_deref().unwrap_or("")),
                        // description: Some(
                        //     "A customer created through the Anything platform.",
                        // ),
                        metadata: Some(std::collections::HashMap::from([
                            (
                                String::from("team_name"),
                                String::from(record.name.as_deref().unwrap_or("")),
                            ),
                            (String::from("team_id"), String::from(record.id.to_string())),
                        ])),

                        ..Default::default()
                    },
                )
                .await
                .unwrap();

                println!(
                    "created a customer at https://dashboard.stripe.com/test/customers/{}",
                    customer.id
                );

                // Update the accounts_billing table with Stripe customer data
                let update_account_billing = serde_json::json!({
                    "stripe_customer_id": customer.id,
                    "stripe_data": serde_json::to_value(&customer).unwrap(),
                });

                match state
                    .anything_client
                    .from("accounts_billing")
                    .auth(&supabase_service_role_api_key)
                    .eq("account_id", record.id.to_string())
                    .update(update_account_billing.to_string())
                    .execute()
                    .await
                {
                    Ok(response) => {
                        println!("Successfully updated accounts_billing: {:?}", response);
                    }
                    Err(err) => {
                        eprintln!("Failed to update accounts_billing: {:?}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to update billing information".to_string(),
                        ));
                    }
                }

                // Create the subscriptoin
                // let subscription = {
                //     let mut params = CreateSubscription::new(customer.id.clone());
                //     params.items = Some(vec![
                //         CreateSubscriptionItems {
                //             price: Some("price_1PwpfXFBAuZoeGEU0iJhmcxF".to_string()), //0.99 / 1k extra
                //             ..Default::default()
                //         },
                //         CreateSubscriptionItems {
                //             price: Some("price_1Pwpe2FBAuZoeGEUh9zS63rH".to_string()), //$10 / month for 10k
                //             ..Default::default()
                //         },
                //     ]);
                //     params.trial_period_days = Some(7);
                //     // params.default_payment_method = Some(&payment_method.id);
                //     params.expand = &["items", "items.data.price.product", "schedule"];

                //     Subscription::create(&client, params).await.unwrap()
                // };

                // Create the input for the SQL function
                // let stripe_input = UpsertCustomerSubscriptionInput {
                //     account_id: record.id,
                //     customer: Some(serde_json::to_value(customer).unwrap()),
                //     subscription: Some(serde_json::to_value(subscription).unwrap()), // We're not creating a subscription at this point
                // };

                // Update the account with the Stripe customer ID
                //     let update_account_billing_response = match state
                //         .public_client
                //         .rpc(
                //             "service_role_upsert_customer_subscription",
                //             serde_json::to_string(&stripe_input).unwrap(),
                //         )
                //         .auth(supabase_service_role_api_key.clone())
                //         .execute()
                //         .await
                //     {
                //         Ok(response) => {
                //             println!("Response status: {:?}", response.status());
                //             println!("Response headers: {:?}", response.headers());

                //             if response.status().is_success() {
                //                 response
                //             } else {
                //                 let status = response.status();
                //                 let error_body = response
                //                     .text()
                //                     .await
                //                     .unwrap_or_else(|_| "Unable to read error body".to_string());
                //                 eprintln!("Error response body: {}", error_body);
                //                 return Err((
                //                     StatusCode::INTERNAL_SERVER_ERROR,
                //                     format!("Failed to fetch user data. Status: {}", status),
                //                 ));
                //             }
                //         }
                //         Err(err) => {
                //             eprintln!("Error fetching user data: {:?}", err);
                //             return Err((
                //                 StatusCode::INTERNAL_SERVER_ERROR,
                //                 "Failed to fetch user data".to_string(),
                //             ));
                //         }
                //     };
            }

            Ok(StatusCode::CREATED)
        }
        _ => Ok(StatusCode::OK), // Ignore other types of webhook payloads
    }
}

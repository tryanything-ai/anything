use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;
use std::sync::Arc;


// #[derive(Debug, Deserialize)]
// #[serde(tag = "type")]
// pub enum WebhookPayload<T> {
//     #[serde(rename = "INSERT")]
//     Insert {
//         table: String,
//         schema: String,
//         record: T,
//         old_record: Option<()>,
//     },
//     #[serde(rename = "UPDATE")]
//     Update {
//         table: String,
//         schema: String,
//         record: T,
//         old_record: T,
//     },
//     #[serde(rename = "DELETE")]
//     Delete {
//         table: String,
//         schema: String,
//         record: Option<()>,
//         old_record: T,
//     },
// }

// #[derive(Debug, Deserialize)]
// pub struct TableRecord {
//     id: uuid::Uuid,
//     primary_owner_user_id: uuid::Uuid,
//     name: Option<String>,
//     slug: Option<String>,
//     personal_account: bool,
//     updated_at: Option<chrono::DateTime<chrono::Utc>>,
//     created_at: Option<chrono::DateTime<chrono::Utc>>,
//     created_by: Option<uuid::Uuid>,
//     updated_by: Option<uuid::Uuid>,
//     private_metadata: serde_json::Value,
//     public_metadata: serde_json::Value,
// }

// pub type NewAccountWebhookPayload = WebhookPayload<TableRecord>;

pub async fn handle_new_account_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    // match payload {
    //     WebhookPayload::Insert { record, .. } => {
    println!(
        "New account created making stripe account now: {:?}",
        payload
    );
    // Handle the new account creation
    // let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|_| {
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         "Stripe secret key not found".to_string(),
    //     )
    // })?;

    // let client = StripeClient::new(stripe_secret_key);

    // let mut create_account = CreateAccount::new();
    // // You might want to get the email from somewhere else, as it's not in the TableRecord
    // // create_account.email = Some(&record.email);
    // create_account.type_ = Some(stripe::AccountType::Standard);

    // let account = StripeAccount::create(&client, create_account)
    //     .await
    //     .map_err(|e| {
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             format!("Failed to create Stripe account: {}", e),
    //         )
    //     })?;

    // Here you might want to update the account in your database with the Stripe account ID
    // For example:
    // update_account_with_stripe_id(state, record.id, account.id).await?;

    //     Ok(StatusCode::CREATED)
    // }
    // _ => Ok(StatusCode::OK), // Ignore other types of webhook payloads
    // }
    Ok(StatusCode::OK)
}

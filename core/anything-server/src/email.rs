use crate::{
    billing::accounts::{User, WebhookPayload},
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct LoopsUser {
    #[serde(rename = "userId")]
    user_id: uuid::Uuid,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lastName")]
    last_name: Option<String>,
}

pub type NewUserWebhookPayload = WebhookPayload<User>;

pub async fn handle_new_account_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUserWebhookPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("[EXTERNAL EMAIL SYSTEM] Received new account webhook");
    let loops_api_key = std::env::var("LOOPS_API_KEY").expect("LOOPS_API_KEY must be set");
    let loops_api_url = "https://app.loops.so/api/v1/contacts/create";

    let client = Client::new();

    let user = match payload {
        WebhookPayload::Insert { record, .. } => {
            println!("[EXTERNAL EMAIL SYSTEM] Processing INSERT payload");
            record
        }
        _ => {
            println!("[EXTERNAL EMAIL SYSTEM] Received unexpected payload type");
            return Err((
                StatusCode::BAD_REQUEST,
                "Expected INSERT payload".to_string(),
            ));
        }
    };

    println!("[EXTERNAL EMAIL SYSTEM] Creating LoopsUser struct");
    let loops_user = LoopsUser {
        user_id: user.id,
        email: user.email.clone().unwrap_or_default(),
        first_name: None,
        last_name: None,
    };
    println!("[EXTERNAL EMAIL SYSTEM] LoopsUser: {:?}", loops_user);

    println!("[EXTERNAL EMAIL SYSTEM] Sending request to Loops API");
    let response = client
        .post(loops_api_url)
        .header("Authorization", format!("Bearer {}", loops_api_key))
        .json(&loops_user)
        .send()
        .await
        .map_err(|e| {
            println!(
                "[EXTERNAL EMAIL SYSTEM] Failed to send request to Loops: {}",
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to send request to Loops: {}", e),
            )
        })?;

    if response.status().is_success() {
        println!("[EXTERNAL EMAIL SYSTEM] Successfully created user in Loops");
        Ok(StatusCode::OK)
    } else {
        println!(
            "[EXTERNAL EMAIL SYSTEM] Failed to create user in Loops. Status: {}",
            response.status()
        );
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user in Loops: {}", response.status()),
        ))
    }
}

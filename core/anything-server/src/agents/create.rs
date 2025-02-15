use anyhow::Result;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;
use crate::agents::{twilio, vapi};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVoiceAgentInput {
    name: String,
    greeting: String,
    area_code: String,
}

#[derive(Debug, Serialize)]
pub struct VoiceAgentResponse {
    id: String,
    phone_number: String,
}

pub async fn create_agent(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateVoiceAgentInput>,
) -> impl IntoResponse {
    // Provision Twilio number
    let twilio_number = match twilio::provision_twilio_number(&payload.area_code).await {
        Ok(number) => number,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to provision Twilio number",
            )
                .into_response()
        }
    };

    // Create Vapi agent
    let agent_id = match vapi::create_vapi_agent(&payload.name, &payload.greeting).await {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create Vapi agent",
            )
                .into_response()
        }
    };

    // Connect the Twilio number to the Vapi agent
    let vapi_api_key = std::env::var("VAPI_API_KEY").expect("VAPI_API_KEY must be set");
    let client = reqwest::Client::new();

    match client
        .post("https://api.vapi.ai/phone-numbers")
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .json(&serde_json::json!({
            "phoneNumber": twilio_number.phone_number,
            "agentId": agent_id
        }))
        .send()
        .await
    {
        Ok(_) => (),
        Err(_) => {
            // Clean up on failure
            let _ = vapi::delete_vapi_agent(&agent_id).await;
            let _ = twilio::delete_twilio_number(&twilio_number.sid).await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to connect phone number",
            )
                .into_response()
        }
    };

    // Return success response
    Json(VoiceAgentResponse {
        id: agent_id,
        phone_number: twilio_number.phone_number,
    })
    .into_response()
}

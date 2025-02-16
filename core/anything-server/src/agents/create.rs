use anyhow::Result;

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::agents::vapi::create_vapi_agent;
use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAgentInput {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAgentResponse {
    agent_id: String,
}

pub async fn create_agent(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateAgentInput>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    // Create default config with greeting and system prompt
    let config = serde_json::json!({
        "greeting": "Hello, this is Mary from Mary's Dental. How can I assist you today?",
        "system_prompt":
        r#"You are a voice assistant for Mary's Dental, a dental office located at 123 North Face Place, Anaheim, California. The hours are 8 AM to 5PM daily, but they are closed on Sundays.

Mary's dental provides dental services to the local Anaheim community. The practicing dentist is Dr. Mary Smith.

You are tasked with answering questions about the business, and booking appointments. If they wish to book an appointment, your goal is to gather necessary information from callers in a friendly and efficient manner like follows:

1. Ask for their full name.
2. Ask for the purpose of their appointment.
3. Request their preferred date and time for the appointment.
4. Confirm all details with the caller, including the date and time of the appointment.

- Be sure to be kind of funny and witty!
- Keep all your responses short and simple. Use casual language, phrases like "Umm...", "Well...", and "I mean" are preferred.
- This is a voice conversation, so keep your responses short, like in a real conversation. Don't ramble for too long."#
    });


    // Create VAPI agent first
    let vapi_response = match create_vapi_agent(
        &account_id,
        &payload.name,
        config["greeting"].as_str().unwrap_or_default(),
        config["system_prompt"].as_str().unwrap_or_default(),
    )
    .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create VAPI agent",
            )
                .into_response()
        }
    };

    // Create agent record with VAPI details
    let agent_input = serde_json::json!({
        "agent_id": vapi_response["id"],
        "agent_name": payload.name,
        "account_id": account_id,
        "active": false,
        "archived": false,
        "config": config,
        "vapi_assistant_id": vapi_response["id"],
        "vapi_config": vapi_response
    });

    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .insert(agent_input.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create agent record",
            )
                .into_response()
        }
    };

    let agent = match response.json::<serde_json::Value>().await {
        Ok(agent) => agent,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent response",
            )
                .into_response()
        }
    };

    let agent_id = agent["agent_id"].as_str().unwrap_or("").to_string();

    // Return success response
    Json(CreateAgentResponse { agent_id }).into_response()
}

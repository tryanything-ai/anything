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

    // Create agent record
    let agent_input = serde_json::json!({
        "agent_name": payload.name,
        "account_id": account_id,
        "active": false,
        "archived": false,
        "config": config
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
                "Failed to create agent record"
            )
                .into_response()
        }
    };

    let agent = match response.json::<serde_json::Value>().await {
        Ok(agent) => agent,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Failed to parse agent response"
            )
                .into_response()
        }
    };

    let agent_id = agent["agent_id"]
        .as_str()
        .unwrap_or("")
        .to_string();

    // // Create Vapi agent
    // let agent_id = match vapi::create_vapi_agent(&payload.name).await {
    //     Ok(id) => id,
    //     Err(_) => {
    //         return (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Failed to create Vapi agent",
    //         )
    //             .into_response()
    //     }
    // };

    // // Connect the Twilio number to the Vapi agent
    // let vapi_api_key = std::env::var("VAPI_API_KEY").expect("VAPI_API_KEY must be set");
    // let client = reqwest::Client::new();

    // match client
    //     .post("https://api.vapi.ai/phone-numbers")
    //     .header("Authorization", format!("Bearer {}", vapi_api_key))
    //     .json(&serde_json::json!({
    //         "phoneNumber": twilio_number.phone_number,
    //         "agentId": agent_id
    //     }))
    //     .send()
    //     .await
    // {
    //     Ok(_) => (),
    //     Err(_) => {
    //         // Clean up on failure
    //         let _ = vapi::delete_vapi_agent(&agent_id).await;
    //         let _ = twilio::delete_twilio_number(&twilio_number.sid).await;
    //         return (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Failed to connect phone number",
    //         )
    //             .into_response();
    //     }
    // };

    // Return success response
    Json(CreateAgentResponse { agent_id }).into_response()
}

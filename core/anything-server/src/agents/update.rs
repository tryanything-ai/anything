use anyhow::Result;

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::agents::vapi::update_vapi_agent;
use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAgentInput {
    name: String,
    greeting: String,
    system_prompt: String,
}

pub async fn update_agent(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateAgentInput>,
) -> impl IntoResponse {
    let client = &state.anything_client;
    println!("Updating agent: {}", agent_id);
    // Update Vapi First
    let vapi_response = match update_vapi_agent(
        &agent_id, //We make the agent id and vapi agent ID the same on creation so this should work
        &payload.name,
        &payload.greeting,
        &payload.system_prompt,
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

    // Create config update with provided fields
    let config = serde_json::json!({
        "greeting": payload.greeting,
        "system_prompt": payload.system_prompt
    });

    let agent_update = serde_json::json!({
        "agent_name": payload.name,
        "config": config,
        "vapi_config": vapi_response
    });

    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .eq("agent_id", agent_id)
        .eq("account_id", account_id)
        .update(agent_update.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update agent record",
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

    Json(agent).into_response()
}

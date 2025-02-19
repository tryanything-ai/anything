use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

pub async fn delete_agent(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    //TODO: delete phone number from vapi if this was connected to a phone number
    //TODO: delete agent channel
    //TODO: delete agent from vapi

    let client = &state.anything_client;

    // First get the communication channels before deleting them
    let channels_response = match client
        .from("agent_communication_channels")
        .auth(user.jwt.clone())
        .eq("agent_id", &agent_id)
        .eq("account_id", &account_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get agent communication channels",
            )
                .into_response()
        }
    };

    let channels_body = match channels_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read channels response body",
            )
                .into_response()
        }
    };

    let channels: Vec<serde_json::Value> = match serde_json::from_str(&channels_body) {
        Ok(channels) => channels,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse channels JSON",
            )
                .into_response()
        }
    };

    // Delete any phone numbers from VAPI
    for channel in &channels {
        if let Some(channel_type) = channel.get("channel_type").and_then(|t| t.as_str()) {
            if channel_type == "phone" {
                if let Some(vapi_phone_number_id) = channel
                    .get("vapi_phone_number_id")
                    .and_then(|id| id.as_str())
                {
                    if let Err(_) =
                        crate::agents::vapi::delete_vapi_phone_number(vapi_phone_number_id).await
                    {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to delete VAPI phone number",
                        )
                            .into_response();
                    }
                }
            }
        }
    }

    // Now archive the communications channels
    let delete_communication_channel_response = match client
        .from("agent_communication_channels")
        .auth(user.jwt.clone())
        .eq("agent_id", &agent_id)
        .eq("account_id", &account_id)
        .update("{\"archived\": true, \"active\": false}")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete agent communication channel",
            )
                .into_response()
        }
    };

    let response = match client
        .from("agents")
        .auth(user.jwt.clone())
        .eq("agent_id", &agent_id)
        .eq("account_id", &account_id)
        .update("{\"archived\": true, \"active\": false}")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    Json(body).into_response()
}

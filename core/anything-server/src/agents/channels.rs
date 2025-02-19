use crate::supabase_jwt_middleware::User;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectPhoneToAgent {
    phone_number_id: String,
}
pub async fn connect_phone_number_to_agent(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<ConnectPhoneToAgent>,
) -> impl IntoResponse {
    println!("[CHANNELS] Handling update_agent_channel");
    println!("[CHANNELS] Account ID: {}", account_id);
    println!("[CHANNELS] Agent ID: {}", agent_id);
    println!("[CHANNELS] Phone Number ID: {}", payload.phone_number_id);

    let client = &state.anything_client;

    let update_json = serde_json::json!({
        "channel_type": "phone",
        "account_id": account_id,
        "agent_id": agent_id,
        "phone_number_id": payload.phone_number_id,
    });
    println!("[CHANNELS] Update JSON: {:?}", update_json);

    let response = match client
        .from("agent_communication_channels")
        .auth(user.jwt)
        .insert(update_json.to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[CHANNELS] Successfully updated agent channel");
            response
        }
        Err(err) => {
            eprintln!("[CHANNELS] Error updating agent channel: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[CHANNELS] Response body: {}", body);
            body
        }
        Err(err) => {
            eprintln!("[CHANNELS] Error reading response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    //TODO: add to VAPI

    println!("[CHANNELS] Successfully completed operation");
    Json(body).into_response()
}


pub async fn remove_phone_number_from_agent(
    Path((account_id, agent_id, phone_number_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[CHANNELS] Removing phone number {} from agent {}",
        phone_number_id, agent_id
    );

    let client = &state.anything_client;

    let response = match client
        .from("agent_communication_channels")
        .auth(user.jwt)
        .eq("channel_type", "phone")
        .eq("account_id", &account_id)
        .eq("agent_id", &agent_id)
        .eq("phone_number_id", &phone_number_id)
        .delete()
        .execute()
        .await
    {
        Ok(response) => {
            println!("[CHANNELS] Successfully removed agent channel");
            response
        }
        Err(err) => {
            eprintln!("[CHANNELS] Error removing agent channel: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[CHANNELS] Response body: {}", body);
            body
        }
        Err(err) => {
            eprintln!("[CHANNELS] Error reading response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    //TODO: remove from VAPI

    println!("[CHANNELS] Successfully completed operation");
    Json(body).into_response()
}

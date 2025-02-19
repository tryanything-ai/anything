use crate::agents::vapi::create_vapi_phone_number_from_twilio_number;
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
use serde_json::Value;
use crate::agents::vapi::delete_vapi_phone_number;

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

    //TODO: check if phone number is already connected to an agent. DOn't let us do this if that is the case.
    //We don't want lots of duplicates etc in here. that would be bad.

    let vapi_result = match create_vapi_phone_number_from_twilio_number(
        state.clone(),
        &payload.phone_number_id,
        &agent_id,
    )
    .await
    {
        Ok(json) => json,
        Err(e) => {
            eprintln!("[CHANNELS] Error creating VAPI phone number: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create VAPI phone number",
            )
                .into_response();
        }
    };

    let client = &state.anything_client;

    let insert_json = serde_json::json!({
        "channel_type": "phone",
        "account_id": account_id,
        "agent_id": agent_id,
        "phone_number_id": payload.phone_number_id,
        "vapi_phone_number_id": vapi_result["id"]
    });
    println!("[CHANNELS] Update JSON: {:?}", insert_json);

    let response = match client
        .from("agent_communication_channels")
        .auth(user.jwt)
        .insert(insert_json.to_string())
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

    let vapi_phone_number_id = match serde_json::from_str::<Value>(&body) {
        Ok(json) => json["vapi_phone_number_id"].to_string(),
        Err(e) => {
            eprintln!("[CHANNELS] Error parsing response body: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse response body").into_response();
        }
    };

    //TODO: remove from VAPI
    let vapi_result = match delete_vapi_phone_number(&vapi_phone_number_id).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("[CHANNELS] Error deleting VAPI phone number: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete VAPI phone number").into_response();
        }
    };

    println!("[CHANNELS] Successfully completed operation");
    Json(body).into_response()
}

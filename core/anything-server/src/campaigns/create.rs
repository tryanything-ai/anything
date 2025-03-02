use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCampaignInput {
    pub name: String,
    pub description: String,
    pub agent_id: String,
}

pub async fn create_campaign(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<CreateCampaignInput>,
) -> impl IntoResponse {
    println!("Handling create_campaign");

    if payload.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Campaign name is required").into_response();
    }

    if payload.agent_id.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Agent ID is required").into_response();
    }

    let client = &state.anything_client;

    let campaign_data = serde_json::json!({
        "account_id": account_id,
        "campaign_name": payload.name,
        "campaign_description": payload.description,
        "agent_id": payload.agent_id,
        "campaign_status": "inactive",
        "active": true,
        "archived": false
    });

    let response = match client
        .from("campaigns")
        .auth(&user.jwt)
        .insert(campaign_data.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create campaign",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let campaign: Value = match serde_json::from_str(&body) {
        Ok(campaign) => campaign,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    (StatusCode::CREATED, Json(campaign)).into_response()
}

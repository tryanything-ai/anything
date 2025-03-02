use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCampaignInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub agent_id: Option<String>,
    pub schedule_days_of_week: Option<Vec<String>>,
    pub schedule_start_time: Option<String>,
    pub schedule_end_time: Option<String>,
    pub timezone: Option<String>,
}

pub async fn update_campaign(
    Path((account_id, campaign_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateCampaignInput>,
) -> impl IntoResponse {
    println!(
        "[CAMPAIGN] Handling update_campaign for campaign {} in account {}",
        campaign_id, account_id
    );

    let client = &state.anything_client;

    // Build the update data dynamically based on what fields are provided
    let mut update_data = json!({});

    if let Some(name) = payload.name {
        update_data["campaign_name"] = json!(name);
    }

    if let Some(description) = payload.description {
        update_data["campaign_description"] = json!(description);
    }

    if let Some(agent_id) = payload.agent_id {
        update_data["agent_id"] = json!(agent_id);
    }

    if let Some(days) = payload.schedule_days_of_week {
        update_data["schedule_days_of_week"] = json!(days);
    }

    if let Some(start_time) = payload.schedule_start_time {
        update_data["schedule_start_time"] = json!(start_time);
    }

    if let Some(end_time) = payload.schedule_end_time {
        update_data["schedule_end_time"] = json!(end_time);
    }

    if let Some(timezone) = payload.timezone {
        update_data["timezone"] = json!(timezone);
    }

    // If no fields were provided, return an error
    if update_data.as_object().unwrap().is_empty() {
        println!("[CAMPAIGN] No fields provided for update");
        return (StatusCode::BAD_REQUEST, "No fields provided for update").into_response();
    }

    println!("[CAMPAIGN] Updating campaign with data: {:?}", update_data);

    // Update the campaign
    let response = match client
        .from("campaigns")
        .auth(&user.jwt)
        .eq("campaign_id", &campaign_id)
        .eq("account_id", &account_id)
        .eq("archived", "false")
        .update(update_data.to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!("[CAMPAIGN] Successfully updated campaign");
            response
        }
        Err(err) => {
            println!("[CAMPAIGN] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update campaign",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            println!("[CAMPAIGN] Successfully read response body");
            body
        }
        Err(err) => {
            println!("[CAMPAIGN] Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let result: Value = match serde_json::from_str(&body) {
        Ok(result) => {
            println!("[CAMPAIGN] Successfully parsed JSON response");
            result
        }
        Err(err) => {
            println!("[CAMPAIGN] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(result).into_response()
}

pub async fn update_campaign_status(
    Path((account_id, campaign_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    _headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    println!("Handling update_campaign_status");

    let status = match payload.get("status") {
        Some(status) => match status.as_str() {
            Some(s) => s,
            None => return (StatusCode::BAD_REQUEST, "Status must be a string").into_response(),
        },
        None => return (StatusCode::BAD_REQUEST, "Status is required").into_response(),
    };

    if status != "active" && status != "inactive" && status != "completed" {
        return (
            StatusCode::BAD_REQUEST,
            "Status must be 'active', 'inactive', or 'completed'",
        )
            .into_response();
    }

    let client = &state.anything_client;

    let update_data = serde_json::json!({
        "campaign_status": status
    });

    let response = match client
        .from("campaigns")
        .auth(&user.jwt)
        .eq("campaign_id", &campaign_id)
        .eq("account_id", &account_id)
        .update(update_data.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update campaign status",
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

    Json(campaign).into_response()
}

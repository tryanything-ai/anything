use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

pub async fn delete_campaign(
    Path((account_id, campaign_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling delete_campaign");

    let client = &state.anything_client;

    // We don't actually delete the campaign, just mark it as archived and inactive
    let update_data = serde_json::json!({
        "archived": true,
        "active": false
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
                "Failed to delete campaign",
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

    let result: Value = match serde_json::from_str(&body) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(result).into_response()
}

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::custom_auth::User;
use crate::AppState;
// TODO: Add marketplace entities when available
// use crate::entities::{marketplace_action_templates, marketplace_profiles};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishActionRequest {
    pub action_template_id: String,
    pub marketplace_profile_id: Option<String>,
}

// Get actions from marketplace using SeaORM
pub async fn get_actions_from_marketplace(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("Handling get_actions_from_marketplace with SeaORM");

    // TODO: Implement marketplace action templates query when entity is available
    let marketplace_actions = json!({
        "message": "get_actions_from_marketplace not fully implemented with SeaORM",
        "actions": [],
        "status": "placeholder"
    });

    Json(marketplace_actions).into_response()
}

// Publish action template to marketplace using SeaORM
pub async fn publish_action_template(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<PublishActionRequest>,
) -> impl IntoResponse {
    println!(
        "Handling publish_action_template for action: {}",
        payload.action_template_id
    );

    // TODO: Implement the full publish workflow:
    // 1. Validate the action template exists and user owns it
    // 2. Get or create marketplace profile
    // 3. Copy action to marketplace schema
    // 4. Set up proper permissions

    let response = json!({
        "message": "publish_action_template not fully implemented with SeaORM",
        "action_template_id": payload.action_template_id,
        "marketplace_profile_id": payload.marketplace_profile_id,
        "status": "placeholder"
    });

    Json(response).into_response()
}

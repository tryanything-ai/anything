use axum::{
    extract::{Extension, Path, State},
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
// use crate::entities::{marketplace_workflows, marketplace_profiles};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishWorkflowRequest {
    pub workflow_id: String,
    pub workflow_version_id: String,
    pub marketplace_profile_id: Option<String>,
}

// Get marketplace workflows using SeaORM
pub async fn get_marketplace_workflows(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("Handling get_marketplace_workflows with SeaORM");

    // TODO: Implement marketplace workflows query when entity is available
    let marketplace_workflows = json!({
        "message": "get_marketplace_workflows not fully implemented with SeaORM",
        "workflows": [],
        "status": "placeholder"
    });

    Json(marketplace_workflows).into_response()
}

// Get marketplace workflow by slug using SeaORM
pub async fn get_marketplace_workflow_by_slug(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("Handling get_marketplace_workflow_by_slug for slug: {}", slug);

    // TODO: Implement marketplace workflow query by slug when entity is available
    let workflow = json!({
        "message": "get_marketplace_workflow_by_slug not fully implemented with SeaORM",
        "slug": slug,
        "workflow": null,
        "status": "placeholder"
    });

    Json(workflow).into_response()
}

// Publish workflow to marketplace using SeaORM
pub async fn publish_workflow_to_marketplace(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<PublishWorkflowRequest>,
) -> impl IntoResponse {
    println!(
        "Handling publish_workflow_to_marketplace for workflow: {}, version: {}",
        workflow_id, workflow_version_id
    );

    // TODO: Implement the full publish workflow:
    // 1. Validate the workflow version exists and user owns it
    // 2. Get or create marketplace profile
    // 3. Copy workflow to marketplace schema
    // 4. Set up proper permissions
    // 5. Generate slug and metadata

    let response = json!({
        "message": "publish_workflow_to_marketplace not fully implemented with SeaORM",
        "account_id": account_id,
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "marketplace_profile_id": payload.marketplace_profile_id,
        "status": "placeholder"
    });

    Json(response).into_response()
}

// Clone marketplace workflow template using SeaORM
pub async fn clone_marketplace_workflow_template(
    Path((account_id, template_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "Handling clone_marketplace_workflow_template for template: {} to account: {}",
        template_id, account_id
    );

    // TODO: Implement the full clone workflow:
    // 1. Fetch the marketplace template
    // 2. Create new workflow in user's account
    // 3. Copy all workflow versions and metadata
    // 4. Update ownership and permissions

    let response = json!({
        "message": "clone_marketplace_workflow_template not fully implemented with SeaORM",
        "account_id": account_id,
        "template_id": template_id,
        "cloned_workflow_id": null,
        "status": "placeholder"
    });

    Json(response).into_response()
}

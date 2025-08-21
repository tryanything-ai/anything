use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::custom_auth::User;
use crate::entities::{flows, flow_versions};
use crate::types::workflow_types::WorkflowVersionDefinition;
use crate::AppState;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, JoinType, ActiveModelTrait, Set};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkflowRequest {
    pub flow_name: String,
    pub flow_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWorkflowRequest {
    pub flow_name: Option<String>,
    pub flow_description: Option<String>,
    pub active: Option<bool>,
}

// Get workflows using SeaORM
pub async fn get_workflows(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_workflows with SeaORM");

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Get flows with their versions using SeaORM
    let workflows = match flows::Entity::find()
        .filter(flows::Column::AccountId.eq(account_uuid))
        .filter(flows::Column::Archived.eq(false))
        .find_with_related(flow_versions::Entity)
        .order_by_desc(flows::Column::CreatedAt)
        .all(&*state.db)
        .await
    {
        Ok(workflows) => workflows,
        Err(err) => {
            println!("Failed to execute database query: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    if workflows.is_empty() {
        return Json(json!([])).into_response();
    }

    // Transform the data to match the expected format
    let result: Vec<Value> = workflows
        .into_iter()
        .map(|(workflow, versions)| {
            let draft_versions: Vec<&flow_versions::Model> = versions
                .iter()
                .filter(|v| !v.published)
                .collect();
            let published_versions: Vec<&flow_versions::Model> = versions
                .iter()
                .filter(|v| v.published)
                .collect();

            json!({
                "flow_id": workflow.flow_id,
                "account_id": workflow.account_id,
                "flow_name": workflow.flow_name,
                "flow_description": workflow.description,
                "active": workflow.active,
                "archived": workflow.archived,
                "created_at": workflow.created_at,
                "updated_at": workflow.updated_at,
                "created_by": workflow.created_by,
                "updated_by": workflow.updated_by,
                "draft_workflow_versions": draft_versions,
                "published_workflow_versions": published_versions
            })
        })
        .collect();

    Json(result).into_response()
}

// Get single workflow using SeaORM
pub async fn get_workflow(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_workflow with SeaORM for flow: {}", flow_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let flow_uuid = match Uuid::parse_str(&flow_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid flow ID").into_response(),
    };

    let workflow_with_versions = match flows::Entity::find()
        .filter(flows::Column::FlowId.eq(flow_uuid))
        .filter(flows::Column::AccountId.eq(account_uuid))
        .find_with_related(flow_versions::Entity)
        .all(&*state.db)
        .await
    {
        Ok(results) => {
            if let Some((workflow, versions)) = results.into_iter().next() {
                (workflow, versions)
            } else {
                return (StatusCode::NOT_FOUND, "Workflow not found").into_response();
            }
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let (workflow, versions) = workflow_with_versions;
    let response = json!({
        "flow_id": workflow.flow_id,
        "account_id": workflow.account_id,
        "flow_name": workflow.flow_name,
        "flow_description": workflow.description,
        "active": workflow.active,
        "archived": workflow.archived,
        "created_at": workflow.created_at,
        "updated_at": workflow.updated_at,
        "flow_versions": versions
    });

    Json(response).into_response()
}

// Create workflow using SeaORM
pub async fn create_workflow(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateWorkflowRequest>,
) -> impl IntoResponse {
    println!("Handling create_workflow with SeaORM");

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let flow_id = Uuid::new_v4();

    let new_workflow = flows::ActiveModel {
        flow_id: Set(flow_id),
        account_id: Set(account_uuid),
        flow_name: Set(payload.flow_name.clone()),
        description: Set(payload.flow_description.clone()),
        active: Set(true),
        archived: Set(false),
        created_by: Set(Some(user.id)),
        updated_by: Set(Some(user.id)),
        ..Default::default()
    };

    let created_workflow = match new_workflow.insert(&*state.db).await {
        Ok(workflow) => workflow,
        Err(err) => {
            println!("Failed to create workflow: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create workflow").into_response();
        }
    };

    let response = json!({
        "flow_id": created_workflow.flow_id,
        "account_id": created_workflow.account_id,
        "flow_name": created_workflow.flow_name,
        "flow_description": created_workflow.description,
        "active": created_workflow.active,
        "created_at": created_workflow.created_at,
        "created_by": created_workflow.created_by
    });

    Json(response).into_response()
}

// Placeholder implementations for other workflow functions
pub async fn get_flow_versions(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "message": "get_flow_versions not yet implemented with SeaORM",
        "flow_id": flow_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn get_flow_version(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "message": "get_flow_version not yet implemented with SeaORM",
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn create_workflow_from_json(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    Json(json!({
        "message": "create_workflow_from_json not yet implemented with SeaORM",
        "account_id": account_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn delete_workflow(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "message": "delete_workflow not yet implemented with SeaORM",
        "flow_id": flow_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn update_workflow(
    Path((account_id, flow_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateWorkflowRequest>,
) -> impl IntoResponse {
    Json(json!({
        "message": "update_workflow not yet implemented with SeaORM",
        "flow_id": flow_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn update_workflow_version(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    Json(json!({
        "message": "update_workflow_version not yet implemented with SeaORM",
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn publish_workflow_version(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "message": "publish_workflow_version not yet implemented with SeaORM",
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "status": "placeholder"
    })).into_response()
}

pub async fn get_agent_tool_workflows(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    Json(json!({
        "message": "get_agent_tool_workflows not yet implemented with SeaORM",
        "account_id": account_id,
        "workflows": [],
        "status": "placeholder"
    })).into_response()
}

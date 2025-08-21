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
use crate::entities::agents;
use crate::AppState;
use sea_orm::{EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter, Set};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAgentInput {
    pub agent_name: Option<String>,
    pub description: Option<String>,
    pub agent_type: Option<String>,
    pub configuration: Option<Value>,
    pub active: Option<bool>,
}

pub async fn update_agent(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateAgentInput>,
) -> impl IntoResponse {
    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let agent_uuid = match Uuid::parse_str(&agent_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid agent ID").into_response(),
    };

    // Find the existing agent
    let existing_agent = match agents::Entity::find()
        .filter(agents::Column::AgentId.eq(agent_uuid))
        .filter(agents::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(agent)) => agent,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Agent not found").into_response();
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Create active model for update
    let mut active_agent: agents::ActiveModel = existing_agent.into();

    // Apply updates
    if let Some(name) = payload.agent_name {
        active_agent.agent_name = Set(name);
    }
    if let Some(description) = payload.description {
        active_agent.description = Set(Some(description));
    }
    if let Some(agent_type) = payload.agent_type {
        active_agent.agent_type = Set(agent_type);
    }
    if let Some(configuration) = payload.configuration {
        active_agent.configuration = Set(configuration);
    }
    if let Some(active) = payload.active {
        active_agent.active = Set(active);
    }
    
    active_agent.updated_by = Set(Some(user.id));

    // Save the updated agent
    let updated_agent = match active_agent.update(&*state.db).await {
        Ok(agent) => agent,
        Err(err) => {
            println!("Failed to update agent: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update agent").into_response();
        }
    };

    let response = json!({
        "agent_id": updated_agent.agent_id,
        "account_id": updated_agent.account_id,
        "agent_name": updated_agent.agent_name,
        "description": updated_agent.description,
        "agent_type": updated_agent.agent_type,
        "configuration": updated_agent.configuration,
        "active": updated_agent.active,
        "updated_at": updated_agent.updated_at,
        "updated_by": updated_agent.updated_by
    });

    Json(response).into_response()
}

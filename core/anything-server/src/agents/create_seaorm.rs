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
use sea_orm::{EntityTrait, ActiveModelTrait, Set};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAgentInput {
    pub agent_name: String,
    pub description: Option<String>,
    pub agent_type: Option<String>,
    pub configuration: Option<Value>,
}

pub async fn create_agent(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateAgentInput>,
) -> impl IntoResponse {
    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let agent_id = Uuid::new_v4();

    let new_agent = agents::ActiveModel {
        agent_id: Set(agent_id),
        account_id: Set(account_uuid),
        agent_name: Set(payload.agent_name.clone()),
        description: Set(payload.description.clone()),
        agent_type: Set(payload.agent_type.unwrap_or_else(|| "default".to_string())),
        configuration: Set(payload.configuration.unwrap_or_else(|| json!({}))),
        active: Set(true),
        archived: Set(false),
        created_by: Set(Some(user.id)),
        updated_by: Set(Some(user.id)),
        ..Default::default()
    };

    let created_agent = match new_agent.insert(&*state.db).await {
        Ok(agent) => agent,
        Err(err) => {
            println!("Failed to create agent: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create agent").into_response();
        }
    };

    let response = json!({
        "agent_id": created_agent.agent_id,
        "account_id": created_agent.account_id,
        "agent_name": created_agent.agent_name,
        "description": created_agent.description,
        "agent_type": created_agent.agent_type,
        "configuration": created_agent.configuration,
        "active": created_agent.active,
        "created_at": created_agent.created_at,
        "created_by": created_agent.created_by
    });

    Json(response).into_response()
}

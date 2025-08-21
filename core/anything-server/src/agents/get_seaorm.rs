use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::custom_auth::User;
use crate::entities::agents;
use crate::AppState;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

pub async fn get_agent(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let agent_uuid = match Uuid::parse_str(&agent_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid agent ID").into_response(),
    };

    let agent = match agents::Entity::find()
        .filter(agents::Column::AgentId.eq(agent_uuid))
        .filter(agents::Column::AccountId.eq(account_uuid))
        .filter(agents::Column::Archived.eq(false))
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

    let response = json!({
        "agent_id": agent.agent_id,
        "account_id": agent.account_id,
        "agent_name": agent.agent_name,
        "description": agent.description,
        "agent_type": agent.agent_type,
        "configuration": agent.configuration,
        "active": agent.active,
        "archived": agent.archived,
        "created_at": agent.created_at,
        "updated_at": agent.updated_at,
        "created_by": agent.created_by,
        "updated_by": agent.updated_by
    });

    Json(response).into_response()
}

pub async fn get_agents(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let agents_list = match agents::Entity::find()
        .filter(agents::Column::AccountId.eq(account_uuid))
        .filter(agents::Column::Archived.eq(false))
        .all(&*state.db)
        .await
    {
        Ok(agents) => agents,
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let response: Vec<Value> = agents_list
        .into_iter()
        .map(|agent| json!({
            "agent_id": agent.agent_id,
            "account_id": agent.account_id,
            "agent_name": agent.agent_name,
            "description": agent.description,
            "agent_type": agent.agent_type,
            "configuration": agent.configuration,
            "active": agent.active,
            "archived": agent.archived,
            "created_at": agent.created_at,
            "updated_at": agent.updated_at,
            "created_by": agent.created_by,
            "updated_by": agent.updated_by
        }))
        .collect();

    Json(response).into_response()
}

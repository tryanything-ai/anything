use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::custom_auth::User;
use crate::entities::agents;
use crate::AppState;
use sea_orm::{EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter, Set};

pub async fn delete_agent(
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

    // Mark as archived instead of deleting
    let mut active_agent: agents::ActiveModel = existing_agent.into();
    active_agent.archived = Set(true);
    active_agent.active = Set(false);
    active_agent.updated_by = Set(Some(user.id));

    match active_agent.update(&*state.db).await {
        Ok(_) => {
            Json(json!({
                "message": "Agent archived successfully",
                "agent_id": agent_uuid
            })).into_response()
        }
        Err(err) => {
            println!("Failed to archive agent: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to archive agent").into_response()
        }
    }
}

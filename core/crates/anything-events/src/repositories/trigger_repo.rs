use chrono::Utc;
use sqlx::SqlitePool;

use crate::{
    errors::{EventsError, EventsResult},
    models::trigger::{CreateTrigger, Trigger, TriggerId},
};

#[derive(Debug, Clone)]
pub struct TriggerRepoImpl {
    #[cfg(debug_assertions)]
    pub pool: SqlitePool,
}

impl TriggerRepoImpl {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
pub trait FlowRepo {
    async fn create_trigger(&self, create_trigger: CreateTrigger) -> EventsResult<TriggerId>;
    // async fn get_flows(&self) -> EventsResult<Flow>;
    // async fn get_flow_by_id(&self, event_id: EventId) -> EventsResult<Event>;
}

#[async_trait::async_trait]
impl FlowRepo for TriggerRepoImpl {
    async fn create_trigger(&self, create_trigger: CreateTrigger) -> EventsResult<TriggerId> {
        let row = sqlx::query(
            r#"
        INSERT INTO triggers (trigger_id, event_name, payload, metadata, timestamp)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING trigger_id
        "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(create_trigger.event_name)
        .bind(create_trigger.payload)
        .bind(create_trigger.metadata)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

        let id = row.get("trigger_id");
        Ok(id)
    }
}

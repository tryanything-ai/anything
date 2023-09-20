use chrono::Utc;
use sqlx::Row;
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

#[cfg(test)]
mod tests {
    use crate::internal::test_helper::TestTriggerRepo;
    use sqlx::Row;

    use super::*;

    #[tokio::test]
    async fn test_create_a_trigger_stores() -> anyhow::Result<()> {
        let test_repo = TestTriggerRepo::new().await;
        let dummy_create = test_repo.dummy_create_trigger();

        let res = test_repo
            .trigger_repo
            .create_trigger(dummy_create.clone())
            .await;
        assert!(res.is_ok());

        let res = sqlx::query("SELECT * FROM triggers where trigger_id = ?1")
            .bind(res.unwrap())
            .fetch_one(&test_repo.pool)
            .await;

        assert!(res.is_ok());
        let row = res.unwrap();
        assert_eq!(row.get::<String, _>("event_name"), dummy_create.event_name);

        Ok(())
    }
}

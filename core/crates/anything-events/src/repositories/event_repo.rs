use sqlx::Row;
use sqlx::SqlitePool;

use crate::{
    errors::{EventsError, EventsResult},
    models::event::{CreateEvent, Event, EventId},
};

#[derive(Debug, Clone)]
pub struct EventRepoImpl {
    #[cfg(debug_assertions)]
    pub pool: SqlitePool,
}

impl EventRepoImpl {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
pub trait EventRepo {
    async fn save_event(&self, event: CreateEvent) -> EventsResult<EventId>;
    async fn find_by_id(&self, event_id: EventId) -> EventsResult<Event>;
}

#[async_trait::async_trait]
impl EventRepo for EventRepoImpl {
    /// Save a new event based on CreateEvent struct
    async fn save_event(&self, event: CreateEvent) -> EventsResult<EventId> {
        let row = sqlx::query(
            r#"
            INSERT INTO events (id, flow_id, trigger_id, name, context, started_at, ended_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING id
            "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(event.flow_id)
        .bind(event.trigger_id)
        .bind(event.name)
        .bind(event.context)
        .bind(event.started_at)
        .bind(event.ended_at)
        // .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

        let id = row.get("id");

        Ok(id)
    }

    async fn find_by_id(&self, event_id: EventId) -> EventsResult<Event> {
        let row = sqlx::query_as::<_, Event>("SELECT * from events WHERE id = ?1")
            .bind(event_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Row;

    use crate::internal::test_helper::TestEventRepo;

    use super::*;

    #[tokio::test]
    async fn test_save_event() -> anyhow::Result<()> {
        let test_repo = TestEventRepo::new().await;
        let fake_event = test_repo.dummy_create_event();
        let cloned_fake_event = fake_event.clone();

        let res = test_repo.event_repo.save_event(fake_event).await;
        assert!(res.is_ok());

        let res = sqlx::query("SELECT * FROM events LIMIT 1")
            .fetch_all(&test_repo.pool)
            .await?;

        assert_eq!(res.len(), 1);
        let row = res.first();
        assert!(row.is_some());
        let row = row.unwrap();
        let name: String = row.get("name");
        assert_eq!(&cloned_fake_event.name, &name);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_event_by_id() -> anyhow::Result<()> {
        let test_repo = TestEventRepo::new().await;
        let dummy_event = test_repo.dummy_create_event();
        let _r = test_repo
            .insert_dummy_event(test_repo.dummy_create_event())
            .await;
        let fake_event = test_repo.insert_dummy_event(dummy_event).await?;

        let found = test_repo.event_repo.find_by_id(fake_event.id).await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert_eq!(found.name, fake_event.name);

        Ok(())
    }
}

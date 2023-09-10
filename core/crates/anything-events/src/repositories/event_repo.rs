use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::{
    errors::EventsResult,
    models::event::{CreateEvent, Event, EventId},
};

#[derive(Debug, Clone)]
pub struct EventRepoImpl {
    pool: SqlitePool,
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
            INSERT INTO events (source_id, event_name, payload, metadata)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(event.source_id)
        .bind(event.event_name)
        .bind(event.payload)
        .bind(event.metadata)
        // .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        Ok(row.last_insert_rowid())
    }

    async fn find_by_id(&self, event_id: EventId) -> EventsResult<Event> {
        let row = sqlx::query_as::<_, Event>("SELECT * from events WHERE id = ?1")
            .bind(event_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }
}

#[cfg(test)]
mod tests {
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
        let name: String = row.get("event_name");
        assert_eq!(&cloned_fake_event.event_name, &name);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_event_by_id() -> anyhow::Result<()> {
        let test_repo = TestEventRepo::new().await;
        let fake_event = test_repo.insert_dummy_data().await?;

        let found = test_repo.event_repo.find_by_id(fake_event.id).await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert_eq!(found.event_name, fake_event.event_name);

        Ok(())
    }
}

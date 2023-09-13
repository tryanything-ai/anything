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
            INSERT INTO events (source_id, event_name, payload, metadata, event_id)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING event_id
            "#,
        )
        .bind(event.source_id)
        .bind(event.event_name)
        .bind(event.payload)
        .bind(event.metadata)
        .bind(event.event_id)
        // .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

        let id = row.get("event_id");

        Ok(id)
    }

    async fn find_by_id(&self, event_id: EventId) -> EventsResult<Event> {
        let row = sqlx::query_as::<_, Event>("SELECT * from events WHERE event_id = ?1")
            .bind(event_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
            })?;

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
        let row = row.unwrap().clone();
        let name: String = row.get("event_name");
        assert_eq!(&cloned_fake_event.event_name, &name);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_event_by_id() -> anyhow::Result<()> {
        let test_repo = TestEventRepo::new().await;
        let _r = test_repo.insert_dummy_event().await;
        let fake_event = test_repo.insert_dummy_event().await?;

        let found = test_repo.event_repo.find_by_id(fake_event.event_id).await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert_eq!(found.event_name, fake_event.event_name);

        Ok(())
    }
}

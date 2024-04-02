use sqlx::Row;
use std::fmt::Debug;

use crate::{
    datastore::{Datastore, DatastoreTrait, RepoImpl},
    error::{PersistenceError, PersistenceResult},
    models::event::{CreateEvent, EventId, EventList, StoreEvent},
};

/// Guarenteed Methods available in the Trigger repo
#[async_trait::async_trait]
pub trait EventRepo {
    async fn save_event(&self, event: CreateEvent) -> PersistenceResult<EventId>;
    async fn find_by_id(&self, event_id: EventId) -> PersistenceResult<StoreEvent>;
    async fn find_events_since(
        &self,
        since_date: chrono::DateTime<chrono::Utc>,
    ) -> PersistenceResult<EventList>;
    async fn find_flow_events(&self, flow_id: String) -> PersistenceResult<EventList>;
    async fn reset(&self) -> PersistenceResult<()>;
    async fn get_oldest_waiting_event(&self) -> PersistenceResult<Option<StoreEvent>>;
    async fn mark_event_as_processing(&self, event_id: String) -> PersistenceResult<()>;
    async fn mark_event_as_complete(&self, event_id: String) -> PersistenceResult<()>;
    async fn get_completed_events_for_session(&self, session_id: String) -> PersistenceResult<EventList>;
}

#[derive(Clone)]
pub struct EventRepoImpl {
    pub datastore: Datastore,
}

impl Debug for EventRepoImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventRepoImpl  {{ /* Format your fields here */ }}")
    }
}

#[cfg(feature = "sqlite")]
#[async_trait::async_trait]
impl RepoImpl<sqlx::Sqlite> for EventRepoImpl {
    fn new_with_datastore(datastore: Datastore) -> PersistenceResult<Self> {
        Ok(EventRepoImpl { datastore })
    }

    async fn get_transaction<'a>(&self) -> PersistenceResult<sqlx::Transaction<'a, sqlx::Sqlite>> {
        let pool = self.datastore.get_pool();
        let tx = pool
            .begin()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(tx)
    }
}

#[async_trait::async_trait]
impl EventRepo for EventRepoImpl {
    async fn save_event(&self, event: CreateEvent) -> PersistenceResult<EventId> {
        let pool = self.datastore.get_pool();
        let row = sqlx::query(
            r#"
            INSERT INTO events (event_id, event_status, flow_id, flow_version_id, flow_version_name, trigger_id, trigger_session_id, trigger_session_status, flow_session_id, flow_session_status, node_id, is_trigger, engine_id, stage, config, context, created_at, started_at, ended_at, debug_result, result)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)
            RETURNING event_id
            "#,
        )
        .bind(event.event_id)
        .bind(event.event_status)
        .bind(event.flow_id)
        .bind(event.flow_version_id)
        .bind(event.flow_version_name)
        .bind(event.trigger_id)
        .bind(event.trigger_session_id)
        .bind(event.trigger_session_status)
        .bind(event.flow_session_id)
        .bind(event.flow_session_status)
        .bind(event.node_id)
        .bind(event.is_trigger)
        .bind(event.engine_id)
        .bind(event.stage)
        .bind(event.config)
        .bind(event.context)
        .bind(event.created_at)
        .bind(event.started_at)
        .bind(event.ended_at)
        .bind(event.debug_result)
        .bind(event.result)
        .fetch_one(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let id = row.get("event_id");

        Ok(id)
    }

    async fn find_by_id(&self, event_id: EventId) -> PersistenceResult<StoreEvent> {
        let pool = self.datastore.get_pool();
        let row = sqlx::query_as::<_, StoreEvent>("SELECT * from events WHERE id = ?1")
            .bind(event_id)
            .fetch_one(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row)
    }

    async fn find_events_since(
        &self,
        since_date: chrono::DateTime<chrono::Utc>,
    ) -> PersistenceResult<EventList> {
        let pool = self.datastore.get_pool();
        let rows = sqlx::query_as::<_, StoreEvent>(
            "SELECT * from events WHERE started_at > ?1 ORDER BY started_at ASC",
        )
        .bind(since_date)
        .fetch_all(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(rows)
    }

    async fn get_oldest_waiting_event(&self) -> PersistenceResult<Option<StoreEvent>> {
        println!("get_oldest_waiting_event");
        let pool = self.datastore.get_pool();
        let event = sqlx::query_as::<_, StoreEvent>(
            "SELECT * from events WHERE event_status = 'WAITING' ORDER BY created_at ASC LIMIT 1",
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(event)
    }

    async fn mark_event_as_processing(&self, event_id: String) -> PersistenceResult<()> {
        println!("mark_event_as_processing");
        let pool = self.datastore.get_pool();
        let result = sqlx::query(
            "UPDATE events SET event_status = 'PROCESSING', flow_session_status = 'PROCESSING', trigger_session_status = 'PROCESSING' WHERE event_id = ?1",
        )
        .bind(event_id)
        .execute(pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error occurred in mark_event_as_processing: {:?}", e);
                Err(PersistenceError::DatabaseError(e))
            }
        }
    }

    async fn mark_event_as_complete(&self, event_id: String) -> PersistenceResult<()> {
        let pool = self.datastore.get_pool();
        sqlx::query("UPDATE events SET event_status = 'COMPLETE' WHERE event_id = ?1")
            .bind(event_id)
            .execute(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;
        Ok(())
    }

    async fn get_completed_events_for_session(&self, session_id: String) -> PersistenceResult<EventList> {
        let pool = self.datastore.get_pool();
        let rows = sqlx::query_as::<_, StoreEvent>(
            "SELECT * from events WHERE session_id = ?1 AND event_status = 'COMPLETE' ORDER BY created_at ASC",
        )
        .bind(session_id)
        .fetch_all(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(rows)
    }

    async fn find_flow_events(&self, flow_id: String) -> PersistenceResult<EventList> {
        let pool = self.datastore.get_pool();
        let rows = sqlx::query_as::<_, StoreEvent>(
            "SELECT * from events WHERE flow_id = ?1 ORDER BY started_at ASC",
        )
        .bind(flow_id)
        .fetch_all(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(rows)
    }

    async fn reset(&self) -> PersistenceResult<()> {
        let pool = self.datastore.get_pool();
        sqlx::query("DELETE FROM events")
            .execute(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;
    use crate::test_helper::{get_test_datastore, TestEventHelper, TestFlowHelper};

    // #[tokio::test]
    // async fn test_save_event() {
    //     let datastore = get_test_datastore().await.unwrap();
    //     let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
    //     let test_helper = TestEventHelper::new(datastore.clone());

    //     let create_event = CreateEvent {
    //         node_id: "test".to_string(),
    //         flow_id: None,
    //         trigger_id: None,
    //         context: serde_json::json!({}),
    //         started_at: None,
    //         ended_at: None,
    //     };

    //     let res = event_repo.save_event(create_event.clone()).await;
    //     assert!(res.is_ok());
    //     let event_id = res.unwrap();

    //     let stored_event = test_helper.get_event_by_id(event_id).await;

    //     assert_eq!(stored_event.name, "test".to_string());
    //     assert_eq!(stored_event.flow_id, None);
    // }

    #[tokio::test]
    async fn test_get_event_by_id() {
        let datastore = get_test_datastore().await.unwrap();
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        // let test_helper = TestEventHelper::new(datastore.clone());

        let event_id = create_event(
            event_repo.clone(),
            "test".to_string(),
            "test".to_string(),
            None,
            None,
        )
        .await;

        let stored_event = event_repo.find_by_id(event_id).await.unwrap();
        assert_eq!(stored_event.name, "test".to_string());
        assert_eq!(stored_event.flow_id, Some("test".to_string()));
    }

    #[tokio::test]
    async fn test_get_events_since() {
        let datastore = get_test_datastore().await.unwrap();
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        // let test_helper = TestEventHelper::new(datastore.clone());

        let event_id = create_event(
            event_repo.clone(),
            "test".to_string(),
            "test".to_string(),
            None,
            None,
        )
        .await;
        let _event_id2 = create_event(
            event_repo.clone(),
            "earlier".to_string(),
            "test".to_string(),
            None,
            Some(Utc::now() - Duration::days(31)),
        )
        .await;
        let event_id3 = create_event(
            event_repo.clone(),
            "test".to_string(),
            "test".to_string(),
            None,
            Some(Utc::now() - Duration::days(20)),
        )
        .await;

        let stored_event = event_repo
            .find_events_since(Utc::now() - Duration::days(21))
            .await
            .unwrap();
        assert!(stored_event.len() == 2);
        assert!(stored_event.iter().any(|e| e.id == event_id));
        assert!(stored_event.iter().any(|e| e.id == event_id3));
    }

    #[tokio::test]
    async fn test_get_flow_events() {
        let datastore = get_test_datastore().await.unwrap();
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        // Create a dummy flow
        let flow_id = String::from("test");
        let test_flow_helper = TestFlowHelper::new(datastore.clone());
        let create_flow = test_flow_helper.make_create_flow(flow_id.clone()).await;
        test_flow_helper.create_flow(create_flow).await;

        let event_id = create_event(
            event_repo.clone(),
            flow_id.clone(),
            "test".to_string(),
            None,
            None,
        )
        .await;
        let event_id2 = create_event(
            event_repo.clone(),
            flow_id.clone(),
            "test".to_string(),
            None,
            Some(Utc::now() - Duration::days(31)),
        )
        .await;
        let _event_id3 = create_event(
            event_repo.clone(),
            "not-the-same-event".to_string(),
            "not-the-same-flow".to_string(),
            None,
            Some(Utc::now() - Duration::days(20)),
        )
        .await;

        let stored_events = event_repo.find_flow_events(flow_id.clone()).await.unwrap();
        assert!(stored_events.len() == 2);
        // sorted backwards by ASC
        assert!(stored_events[1].id == event_id);
        assert!(stored_events[0].id == event_id2);
    }

    // async fn create_event(
    //     event_repo: EventRepoImpl,
    //     event_name: String,
    //     flow_id: String,
    //     context: Option<serde_json::Value>,
    //     started_at: Option<chrono::DateTime<chrono::Utc>>,
    // ) -> EventId {
    //     let create_event = CreateEvent {
    //         name: event_name,
    //         flow_id: Some(flow_id),
    //         trigger_id: None,
    //         context: context.unwrap_or(serde_json::json!({"test": "test"})),
    //         started_at: Some(started_at.unwrap_or(chrono::offset::Utc::now())),
    //         ended_at: None,
    //     };

    //     let res = event_repo.save_event(create_event.clone()).await;
    //     assert!(res.is_ok());
    //     let event_id = res.unwrap();
    //     event_id
    // }
}

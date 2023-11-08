use chrono::Utc;
use sqlx::Row;
use std::fmt::Debug;

use crate::{
    datastore::{Datastore, DatastoreTrait, RepoImpl},
    error::{PersistenceError, PersistenceResult},
    models::trigger::{CreateTrigger, StoredTrigger, TriggerId},
};

/// Guarenteed Methods available in the Trigger repo
#[async_trait::async_trait]
pub trait TriggerRepo {
    // TODO: Add types
    async fn create_trigger(&self, create_trigger: CreateTrigger) -> PersistenceResult<TriggerId>;
    async fn get_trigger_by_id(&self, trigger_id: TriggerId) -> PersistenceResult<StoredTrigger>;
    async fn reset(&self) -> PersistenceResult<()>;
}

#[derive(Clone)]
pub struct TriggerRepoImpl {
    pub datastore: Datastore,
}

impl Debug for TriggerRepoImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TriggerRepoImpl {{ /* Format your fields here */ }}")
    }
}

#[cfg(feature = "sqlite")]
#[async_trait::async_trait]
impl RepoImpl<sqlx::Sqlite> for TriggerRepoImpl {
    fn new_with_datastore(datastore: Datastore) -> PersistenceResult<Self> {
        Ok(TriggerRepoImpl { datastore })
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
impl TriggerRepo for TriggerRepoImpl {
    async fn create_trigger(&self, create_trigger: CreateTrigger) -> PersistenceResult<TriggerId> {
        let pool = self.datastore.get_pool();

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
        .fetch_one(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let id = row.get("trigger_id");
        Ok(id)
    }

    async fn get_trigger_by_id(&self, trigger_id: TriggerId) -> PersistenceResult<StoredTrigger> {
        let pool = self.datastore.get_pool();

        let trigger =
            sqlx::query_as::<_, StoredTrigger>(r#"SELECT * from triggers WHERE trigger_id = ?1"#)
                .bind(&trigger_id)
                .fetch_one(pool)
                .await
                .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(trigger)
    }

    async fn reset(&self) -> PersistenceResult<()> {
        let pool = self.datastore.get_pool();

        let _ = sqlx::query("DELETE FROM triggers")
            .execute(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{get_test_datastore, TestTriggerHelper};

    #[tokio::test]
    async fn test_create_a_trigger_stores() {
        let datastore = get_test_datastore().await.unwrap();
        let trigger_repo = TriggerRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        let test_helper = TestTriggerHelper::new(datastore.clone());

        let create_trigger = CreateTrigger {
            trigger_id: uuid::Uuid::new_v4().to_string(),
            event_name: "test".to_string(),
            payload: serde_json::json!({"test": "test"}),
            metadata: None,
        };

        let res = trigger_repo.create_trigger(create_trigger.clone()).await;
        assert!(res.is_ok());
        let trigger_id = res.unwrap();

        let t = test_helper.get_trigger_by_id(trigger_id).await;
        assert_eq!(t.event_name, "test".to_string());
    }

    #[tokio::test]
    async fn test_get_trigger_by_trigger_id() {
        let datastore = get_test_datastore().await.unwrap();
        let trigger_repo = TriggerRepoImpl::new_with_datastore(datastore.clone()).unwrap();

        let trigger_id = create_trigger(trigger_repo.clone(), "test-event".to_string(), None).await;

        let res = trigger_repo.get_trigger_by_id(trigger_id).await;
        assert!(res.is_ok());
        let trigger = res.unwrap();
        assert_eq!(trigger.payload, serde_json::json!({"test": "test"}));
        assert_eq!(trigger.event_name, "test-event".to_string());
    }

    async fn create_trigger(
        trigger_repo: TriggerRepoImpl,
        event_name: String,
        payload: Option<serde_json::Value>,
    ) -> TriggerId {
        let create_trigger = CreateTrigger {
            trigger_id: uuid::Uuid::new_v4().to_string(),
            event_name,
            payload: payload.unwrap_or(serde_json::json!({"test": "test"})),
            metadata: None,
        };

        let res = trigger_repo.create_trigger(create_trigger.clone()).await;
        assert!(res.is_ok());
        let trigger_id = res.unwrap();
        trigger_id
    }
}

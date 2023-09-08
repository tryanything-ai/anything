use async_trait::async_trait;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::{config::Config, models::Event, EvtResult};

use super::store::StoreAdapter;

#[derive(Debug, Clone)]
pub struct SqliteStoreAdapter {
    pub pool: SqlitePool,
}

impl SqliteStoreAdapter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn default() -> Self {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        Self { pool }
    }
}

#[async_trait]
impl StoreAdapter for SqliteStoreAdapter {
    async fn init<'a>(&'a self, _config: &Config) -> EvtResult<bool> {
        sqlx::query(include_str!("../../sql/schema.sql"))
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    async fn save(&self, event: Event) -> EvtResult<bool> {
        let _evt = sqlx::query(
            r#"
            INSERT INTO events (event_name,payload,metadata,tags) VALUES (?,?,?,?)
        "#,
        )
        .bind(event.event_name)
        .bind(serde_json::json!(event.payload))
        .bind(serde_json::json!(event.metadata))
        .bind(serde_json::json!(event.tags))
        .execute(&self.pool)
        .await?;

        Ok(true)
    }
    async fn all(&self) -> EvtResult<Vec<Event>> {
        let res = sqlx::query_as::<_, Event>(
            r#"SELECT id, event_name, payload, metadata, tags, timestamp FROM events"#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(res)
    }

    async fn get(&self, id: i64) -> EvtResult<Event> {
        let res = sqlx::query_as::<_, Event>(
            r#"SELECT id, event_name, payload, metadata, tags, timestamp FROM events WHERE id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(res)
    }

    fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::internal::test_helper::{
        get_pool, get_test_config, insert_n_dummy_data, insert_new_event, select_all_events,
    };

    use super::*;

    #[tokio::test]
    async fn test_initializes_properly() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;
        let config = get_test_config();

        let store = SqliteStoreAdapter::new(pool);

        let res = store.init(&config).await?;
        assert!(res);

        Ok(())
    }

    #[tokio::test]
    async fn test_can_insert_valid_event() -> Result<(), Box<dyn std::error::Error>> {
        let pool = get_pool().await?;
        let pool_clone = pool.clone();

        let store = SqliteStoreAdapter::new(pool);
        let _res = store.init(&get_test_config()).await?;

        let fake_event = Event::new(
            "wee".to_string(),
            serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
            vec![],
        );
        let fake_event = fake_event
            .with_metadata(HashMap::from([("time".to_string(), "now".to_string())]))
            .with_tags(vec!["bob".to_string()]);

        let res = store.save(fake_event).await;
        assert!(res.is_ok());

        let cols = select_all_events(&pool_clone).await?;
        assert_eq!(cols.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn it_can_read_all_events() -> EvtResult<()> {
        let pool = get_pool().await?;
        let pool_clone = pool.clone();

        let store = SqliteStoreAdapter::new(pool);
        let _res = store.init(&get_test_config()).await?;

        insert_n_dummy_data(&pool_clone, 4).await?;

        let all = store.all().await;
        assert!(all.is_ok());
        let all = all.unwrap();
        assert_eq!(all.len(), 4);
        Ok(())
    }

    #[tokio::test]
    async fn it_can_read_a_single_event() -> EvtResult<()> {
        let pool = get_pool().await?;
        let pool_clone = pool.clone();

        let store = SqliteStoreAdapter::new(pool);
        let _res = store.init(&get_test_config()).await?;

        let fake_event = Event::new(
            "wee".to_string(),
            serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
            vec![],
        );
        let fake_event = fake_event
            .with_metadata(HashMap::from([("time".to_string(), "now".to_string())]))
            .with_tags(vec!["bob".to_string()]);
        insert_n_dummy_data(&pool_clone, 4).await?;
        let last_id = insert_new_event(&pool_clone, fake_event.clone()).await?;

        let evt = store.get(last_id).await?;
        assert_eq!(evt.event_name, "wee".to_string());
        assert_eq!(evt.tags.0.len(), 1);

        let effective_metadata = serde_json::from_str::<Value>("{\"time\": \"now\"}").unwrap();
        assert_eq!(evt.metadata.as_object(), effective_metadata.as_object());

        Ok(())
    }
}

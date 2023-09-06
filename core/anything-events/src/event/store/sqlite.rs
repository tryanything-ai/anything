use super::{FetchResult, SaveResult, StoreAdapter};
use crate::{types::AnythingResult, Event};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteQueryResult, SqliteRow},
    Database, Pool, Row, SqlitePool,
};

#[derive(Debug, Clone)]
pub struct SqliteStoreAdapter {
    pub pool: SqlitePool,
}

impl SqliteStoreAdapter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StoreAdapter for SqliteStoreAdapter {
    async fn init<'a>(&'a self) -> SaveResult {
        sqlx::query(include_str!("sql/sqlite/01-events_table.sql"))
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    async fn save(&self, event: Event) -> SaveResult {
        // id BIGINT,
        // name TEXT NOT NULL,
        // payload json NOT NULL,
        // metadata json NOT NULL,
        // tags json NOT NULL,
        // timestamp timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
        let evt = sqlx::query(
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

        println!("result: {:?}", evt);
        Ok(true)
    }
    async fn read(&self, since: Option<DateTime<Utc>>) -> FetchResult {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    #[tokio::test]
    async fn test_initializes_properly() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;

        let store = SqliteStoreAdapter::new(pool);

        let res = store.init().await?;
        assert!(res);

        Ok(())
    }

    #[tokio::test]
    async fn test_can_insert_valid_event() -> Result<(), Box<dyn std::error::Error>> {
        let pool = get_pool().await?;
        let pool_clone = pool.clone();

        let store = SqliteStoreAdapter::new(pool);
        let _res = store.init().await?;

        let fake_event = Event::new(
            "wee".to_string(),
            serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
            vec![],
        );
        let fake_event = fake_event
            .with_metadata(HashMap::from([("time".to_string(), "now".to_string())]))
            .with_tags(vec!["bob".to_string()]);

        let res = store.save(fake_event).await;
        println!("res: {:?}", res);
        assert!(res.is_ok());

        let cols = select_all_events(&pool_clone).await?;

        Ok(())
    }
}

pub async fn get_pool() -> AnythingResult<Pool<sqlx::Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    Ok(pool)
}

pub async fn select_all_events(pool: &SqlitePool) -> AnythingResult<Vec<Event>> {
    let res = sqlx::query_as::<_, Event>(
        r#"SELECT id, event_name, payload, metadata, tags, timestamp FROM events"#,
    )
    .fetch_all(pool)
    .await?;

    // let events = res.into_iter().collect::<Vec<Event>>();
    println!("res: {:#?}", res);

    Ok(res)
}

pub async fn insert_new_event(pool: SqlitePool, query: &str) -> AnythingResult<()> {
    Ok(())
}

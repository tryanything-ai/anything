use std::collections::HashMap;

use super::{FetchResult, SaveResult, StoreAdapter};
use crate::{types::AnythingResult, Event};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use fake::{Fake, Faker};
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
    async fn all(&self) -> AnythingResult<Vec<Event>> {
        let res = sqlx::query_as::<_, Event>(
            r#"SELECT id, event_name, payload, metadata, tags, timestamp FROM events"#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(res)
    }

    async fn get(&self, id: i64) -> AnythingResult<Event> {
        let res = sqlx::query_as::<_, Event>(
            r#"SELECT id, event_name, payload, metadata, tags, timestamp FROM events WHERE id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(res)
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
        assert!(res.is_ok());

        let cols = select_all_events(&pool_clone).await?;
        assert_eq!(cols.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn it_can_read_all_events() -> AnythingResult<()> {
        let pool = get_pool().await?;
        let pool_clone = pool.clone();

        let store = SqliteStoreAdapter::new(pool);
        let _res = store.init().await?;

        insert_n_dummy_data(&pool_clone, 4).await?;

        let all = store.all().await;
        assert!(all.is_ok());
        let all = all.unwrap();
        assert_eq!(all.len(), 4);
        Ok(())
    }

    #[tokio::test]
    async fn it_can_read_a_single_event() -> AnythingResult<()> {
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
        insert_n_dummy_data(&pool_clone, 4).await?;
        let last_id = insert_new_event(&pool_clone, fake_event.clone()).await?;

        let evt = store.get(last_id).await?;
        assert_eq!(evt.event_name, "wee".to_string());
        assert_eq!(evt.tags.0.len(), 1);

        let effective_metadata = serde_json::from_str::<JsonValue>("{\"time\": \"now\"}").unwrap();
        assert_eq!(evt.metadata.as_object(), effective_metadata.as_object());

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

    Ok(res)
}

pub async fn insert_new_event(pool: &SqlitePool, event: Event) -> AnythingResult<i64> {
    let res = sqlx::query(
        r#"INSERT INTO events (event_name, payload, metadata, tags) VALUES (?, ?, ?, ?)"#,
    )
    .bind(event.event_name)
    .bind(event.payload)
    .bind(event.metadata)
    .bind(event.tags)
    .execute(pool)
    .await?;

    Ok(res.last_insert_rowid())
}

pub async fn insert_n_dummy_data(pool: &SqlitePool, count: i8) -> AnythingResult<()> {
    for _i in 0..count {
        insert_dummy_data(pool).await?;
    }
    Ok(())
}

pub async fn insert_dummy_data(pool: &SqlitePool) -> AnythingResult<()> {
    let payload = generate_dummy_hashmap();
    let metadata = generate_dummy_hashmap();
    let tags = fake::faker::lorem::en::Words(3..5).fake();

    let fake_event = Event::new(
        fake::faker::name::en::Name().fake(),
        serde_json::json!(payload),
        tags,
    );
    let fake_event = fake_event.with_metadata(metadata);

    let _res = insert_new_event(pool, fake_event).await?;

    Ok(())
}

fn generate_dummy_hashmap() -> HashMap<String, String> {
    let mut payload: HashMap<String, String> = HashMap::default();

    for _ in 1..4 {
        let key = fake::faker::name::en::Name().fake();
        let value = fake::faker::name::en::Name().fake();
        payload.insert(key, value);
    }
    payload
}

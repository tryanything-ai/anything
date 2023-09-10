use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{AnyPool, Row, SqlitePool};

use crate::config::AnythingEventsConfig;
use crate::errors::EventsResult;
use crate::{Event, Tag, Tags};

#[derive(Debug, Clone)]
pub enum StoreAdapterType {
    SQLITE,
}

#[async_trait::async_trait]
pub trait StoreAdapter {}

#[derive()]
pub struct Store {
    pub store: Arc<SqliteStoreAdapter>,
}

impl Debug for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            // .field("config", &self.store)
            .finish()
    }
}

pub async fn bootstrap_store(config: &AnythingEventsConfig) -> EventsResult<Store> {
    let store = match determine_store_backend(config.database.uri.clone()) {
        StoreAdapterType::SQLITE => SqliteStoreAdapter::init_from_config(config),
    }
    .await?;

    Ok(Store {
        store: Arc::new(store),
    })
}

fn determine_store_backend(database_uri: String) -> StoreAdapterType {
    match database_uri {
        database_uri if database_uri.starts_with("sqlite") => StoreAdapterType::SQLITE,
        _ => panic!("Unsupported database type"),
    }
}

#[derive(Debug, Clone)]
pub struct SqliteStoreAdapter {
    pub pool: SqlitePool,
}

impl SqliteStoreAdapter {
    async fn init_from_config(config: &AnythingEventsConfig) -> EventsResult<Self> {
        // For sqlite databases, we use the root directory
        let root_dir = config.root_dir.clone();
        let db_dir = root_dir.join("database");

        let database_file = db_dir.join("eventurous.db");
        // let database_uri = format!("sqlite://{}", database_file.to_str().unwrap());

        let options = SqliteConnectOptions::new()
            .filename(database_file)
            .create_if_missing(true);

        let mut pool = SqlitePoolOptions::new();
        if let Some(max_connections) = config.database.max_connections {
            pool = pool.max_connections(max_connections as u32);
        }

        let pool = pool.connect_with(options).await?;

        sqlx::query(include_str!("../../sql/schema.sql"))
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }
}

async fn save_event(pool: &SqlitePool, event: Event) -> EventsResult<i64> {
    // Find or create new tags
    let _evt = sqlx::query(
            r#"
            INSERT INTO events (source_id, event_name, payload, metadata, tags, timestamp) VALUES (?,?,?,?,?,?)
        "#,
        )
        .bind(event.source_id)
        .bind(event.event_name)
        .bind(serde_json::json!(event.payload))
        .bind(serde_json::json!(event.metadata))
        .bind(serde_json::json!(event.tags))
        .execute(pool)
        .await?;

    Ok(1)
}

// async fn find_or_create_tags(pool: &SqlitePool, tag_name: String) -> EventsResult<i64> {
//     let tag_query
// }

async fn find_tag(pool: &SqlitePool, tag_name: String) -> EventsResult<Tag> {
    let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ?")
        .bind(tag_name)
        .fetch_one(pool)
        .await?;

    Ok(tag)
}

#[cfg(test)]
mod tests {
    use crate::internal::test_helper::{get_pool, insert_new_tag};

    use super::*;

    #[tokio::test]
    async fn test_find_tags() -> anyhow::Result<()> {
        let pool = get_pool().await?;
        let tag_name = "bob".to_string();
        let tag = find_tag(&pool, tag_name.clone()).await;
        assert!(tag.is_err());

        let tag: Tag = "bob".to_string().into();
        insert_new_tag(&pool, tag).await?;

        let tag = find_tag(&pool, tag_name.clone()).await;

        println!("tag: {:?}", tag);
        assert!(tag.is_ok());
        Ok(())
    }
}

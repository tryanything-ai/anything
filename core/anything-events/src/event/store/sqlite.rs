use super::{SaveResult, SaveStatus, StoreAdapter};
use crate::{event::store::store_query::StoreQuery, types::AnythingResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, SqlitePool};

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
    async fn init<'a>(&'a self) -> AnythingResult<SaveStatus> {
        sqlx::query(include_str!("sql/sqlite/01-events_table.sql"))
            .execute(&self.pool)
            .await?;

        Ok(SaveStatus::Ok)
    }

    async fn save<'a, E>(&'a self, event: &'a E) -> AnythingResult<SaveStatus> {
        Ok(SaveStatus::Ok)
    }

    async fn read<'a, D: Database + Send + Sync, E: Send + Sync + Clone>(
        &'a self,
        query: &'a StoreQuery<'a, D, E>,
        since: Option<DateTime<Utc>>,
    ) -> AnythingResult<Vec<E>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(res, SaveStatus::Ok);

        Ok(())
    }
}

pub async fn insert_new_event(pool: SqlitePool, query: &str) -> AnythingResult<()> {
    Ok(())
}

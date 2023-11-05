use anything_common::tracing;

use crate::error::{PersistenceError, PersistenceResult};

use super::{DatabaseTransaction, Datastore};

pub struct SqliteDatastore {
    pool: sqlx::sqlite::SqlitePool,
}

#[async_trait::async_trait]
impl Datastore<sqlx::Sqlite> for SqliteDatastore {
    async fn new_with_pool(pool: sqlx::sqlite::SqlitePool) -> PersistenceResult<Self>
    where
        Self: Sized + Send + Sync,
    {
        let sqlite_datastore = SqliteDatastore { pool };
        sqlite_datastore
            .after_create()
            .await
            .expect("unable to run migrations"); // Assuming this method exists
        Ok(sqlite_datastore)
    }

    async fn after_create(&self) -> PersistenceResult<()> {
        let pool = self.pool.clone();

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Error running migrations: {}", e);
                PersistenceError::MigrationError(e)
            })?;
        Ok(())
    }

    async fn begin_transaction(&self) -> PersistenceResult<DatabaseTransaction<'_>> {
        let tx = self.pool.begin().await?;
        Ok(DatabaseTransaction::Sqlite(tx))
    }

    fn get_pool(&self) -> &sqlx::Pool<sqlx::Sqlite> {
        &self.pool
    }
}

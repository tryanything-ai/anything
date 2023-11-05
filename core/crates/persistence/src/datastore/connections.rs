use anything_common::tracing;
use anything_coordinator::AnythingConfig;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

use crate::error::PersistenceResult;

pub async fn create_sqlite_pool_from_config(
    anything_config: AnythingConfig,
) -> PersistenceResult<Pool<Sqlite>> {
    let uri = anything_config.database_config().uri.clone();
    let options = match uri {
        None => {
            let database_dir = &anything_config.runtime_config().database_dir();
            let db_file = database_dir.join("anything.db");
            tracing::debug!("Using database file: {:?}", db_file);
            SqliteConnectOptions::new()
                .filename(&db_file)
                .create_if_missing(true)
        }
        Some(uri) => {
            let uri = uri.clone();
            SqliteConnectOptions::new()
                .filename(&uri)
                .create_if_missing(true)
        }
    };

    let mut pool = SqlitePoolOptions::new();

    if let Some(max_connections) = anything_config.database_config().max_connections {
        pool = pool.max_connections(max_connections);
    }

    let pool = pool.connect_with(options).await?;

    Ok(pool)
}

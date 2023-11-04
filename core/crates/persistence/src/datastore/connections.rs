use crate::datastore::types::DatastoreTrait;
use crate::error::PersistenceResult;
use anything_common::{tracing, AnythingConfig};
use anything_store::FileStore;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

use crate::datastore::sqlite::SqliteDatastore;

pub async fn create_sqlite_pool_from_config_and_file_store(
    anything_config: AnythingConfig,
    file_store: FileStore,
) -> PersistenceResult<Pool<Sqlite>> {
    let uri = anything_config.database_config().uri.clone();
    let options = match uri {
        None => {
            let database_dir = file_store.create_directory(&["database"]).unwrap();
            // let database_dir = &anything_config.runtime_config().database_dir();
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

pub async fn create_sqlite_datastore_from_config_and_file_store(
    anything_config: AnythingConfig,
    file_store: FileStore,
) -> PersistenceResult<SqliteDatastore> {
    let pool = create_sqlite_pool_from_config_and_file_store(anything_config, file_store).await?;
    let ds = SqliteDatastore::new_with_pool(pool).await.unwrap();
    Ok(ds)
}

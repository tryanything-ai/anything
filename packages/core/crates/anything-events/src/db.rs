use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

use crate::{
    config::AnythingEventsConfig,
    errors::{EventsError, EventsResult},
};

pub async fn create_sqlite_pool(config: &AnythingEventsConfig) -> EventsResult<SqlitePool> {
    let options = match config.database.uri {
        None => {
            let root_dir = config.root_dir.clone();
            let db_dir = root_dir.join("database");

            let database_file = db_dir.join("anything.db");
            tracing::debug!("Using database file: {:?}", database_file);
            SqliteConnectOptions::new()
                .filename(database_file)
                .create_if_missing(true)
        }
        Some(ref uri) => {
            let uri = uri.clone();
            SqliteConnectOptions::new()
                .filename(uri)
                .create_if_missing(true)
        }
    };
    // let database_uri = format!("sqlite://{}", database_file.to_str().unwrap());

    let mut pool = SqlitePoolOptions::new();
    if let Some(max_connections) = config.database.max_connections {
        pool = pool.max_connections(max_connections as u32);
    }

    let pool = pool
        .connect_with(options)
        .await
        .expect("failed to connect to sqlite db");

    // Migrate
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Migration error: {:?}", e);
            EventsError::MigrationError(e)
        })?;

    // DB.set(pool).expect("unable to set pool");
    Ok(pool)
}

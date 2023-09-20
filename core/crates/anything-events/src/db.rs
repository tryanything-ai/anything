use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

use crate::{
    config::AnythingEventsConfig,
    errors::{DatabaseError, EventsError, EventsResult},
};

pub async fn create_sqlite_pool(config: &AnythingEventsConfig) -> EventsResult<SqlitePool> {
    let root_dir = config.root_dir.clone();
    let db_dir = root_dir.join("database");

    let database_file = db_dir.join("anything.db");
    tracing::debug!("Using database file: {:?}", database_file);
    // let database_uri = format!("sqlite://{}", database_file.to_str().unwrap());

    let options = SqliteConnectOptions::new()
        .filename(database_file)
        .create_if_missing(true);

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
        .map_err(|e| EventsError::DatabaseError(DatabaseError::DBError(Box::new(e))))?;

    // DB.set(pool).expect("unable to set pool");
    Ok(pool)
}

use crate::{
    datastore::{sqlite::SqliteDatastore, Datastore},
    error::{PersistenceError, PersistenceResult},
};
use anything_common::tracing;
use sqlx::sqlite::SqlitePoolOptions;

#[allow(unused)]
pub async fn get_test_datastore() -> PersistenceResult<SqliteDatastore> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

    let res = sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error running migrations: {}", e);
            PersistenceError::MigrationError(e)
        });

    assert!(res.is_ok());

    let ds = SqliteDatastore::new_with_pool(pool).await.unwrap();

    Ok(ds)
}

// pub struct MockDatastore<T> {
//     pub pool: Arc<Pool<sqlx::Sqlite>>,
//     _phantom: PhantomData<T>,
// }

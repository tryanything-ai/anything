use crate::datastore::types::DatastoreTrait;
use crate::models::flow::StoredFlow;
use crate::{
    datastore::sqlite::SqliteDatastore,
    error::{PersistenceError, PersistenceResult},
};
use anything_common::tracing;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[cfg(feature = "sqlite")]
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

pub async fn select_all_flows(pool: &SqlitePool) -> PersistenceResult<Vec<StoredFlow>> {
    let query = sqlx::query_as::<_, StoredFlow>(r#"SELECT * FROM flows"#);

    let result = query.fetch_all(pool).await.map_err(|e| {
        tracing::error!("Error fetching flows: {}", e);
        PersistenceError::DatabaseError(e)
    })?;

    Ok(result)
}

// pub struct MockDatastore<T> {
//     pub pool: Arc<Pool<sqlx::Sqlite>>,
//     _phantom: PhantomData<T>,
// }

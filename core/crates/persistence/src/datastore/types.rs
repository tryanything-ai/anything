use crate::datastore::Datastore;
use crate::error::PersistenceResult;

#[async_trait::async_trait]
pub trait DatastoreTrait<DB>: Send + Sync
where
    DB: sqlx::Database + Sized + Send + Sync,
{
    fn get_pool(&self) -> &sqlx::Pool<DB>;

    async fn new_with_pool(pool: sqlx::Pool<DB>) -> PersistenceResult<Self>
    where
        Self: Sized + Sync + Send;

    async fn after_create(&self) -> PersistenceResult<()>;

    // async fn begin_transaction<'a>(&self) -> PersistenceResult<Box<dyn DatabaseTransaction<'a>>>;
}

#[async_trait::async_trait]
pub trait RepoImpl<DB>
where
    DB: sqlx::Database,
{
    fn new_with_datastore(datastore: Datastore) -> PersistenceResult<Self>
    where
        Self: Sized + Sync + Send + 'static;

    async fn get_transaction<'a>(&self) -> PersistenceResult<sqlx::Transaction<'a, DB>>;
}

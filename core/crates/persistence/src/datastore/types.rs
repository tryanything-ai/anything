use crate::error::PersistenceResult;

pub enum DatastorePool {
    Postgres(sqlx::Pool<sqlx::Postgres>),
    Sqlite(sqlx::Pool<sqlx::Sqlite>),
}

impl DatastorePool {
    pub fn postgres(&self) -> PersistenceResult<&sqlx::Pool<sqlx::Postgres>> {
        match self {
            DatastorePool::Postgres(tx) => Ok(tx),
            _ => Err(crate::error::PersistenceError::InvalidDatabaseType),
        }
    }

    pub fn sqlite(&self) -> PersistenceResult<&sqlx::Pool<sqlx::Sqlite>> {
        match self {
            DatastorePool::Sqlite(tx) => Ok(tx),
            _ => Err(crate::error::PersistenceError::InvalidDatabaseType),
        }
    }
}

pub enum DatabaseTransaction<'a> {
    Postgres(sqlx::Transaction<'a, sqlx::Postgres>),
    Sqlite(sqlx::Transaction<'a, sqlx::Sqlite>),
}

impl<'a> DatabaseTransaction<'a> {
    pub fn postgres(&self) -> Option<&sqlx::Transaction<'a, sqlx::Postgres>> {
        match self {
            DatabaseTransaction::Postgres(tx) => Some(tx),
            _ => None,
        }
    }

    pub fn sqlite(&self) -> Option<&sqlx::Transaction<'a, sqlx::Sqlite>> {
        match self {
            DatabaseTransaction::Sqlite(tx) => Some(tx),
            _ => None,
        }
    }
}

impl DatastorePool {
    pub async fn begin_transaction<'a, DB>(&self) -> PersistenceResult<DatabaseTransaction<'a>>
    where
        DB: sqlx::Database,
    {
        match self {
            DatastorePool::Postgres(pool) => {
                let tx = pool.begin().await?;
                Ok(DatabaseTransaction::Postgres(tx))
            }
            DatastorePool::Sqlite(pool) => {
                let tx = pool.begin().await?;
                Ok(DatabaseTransaction::Sqlite(tx))
            }
        }
    }
}

#[async_trait::async_trait]
pub trait Datastore<DB>: Send + Sync
where
    DB: sqlx::Database + Sized + Send + Sync,
{
    fn get_pool(&self) -> &sqlx::Pool<DB>;

    async fn new_with_pool(pool: sqlx::Pool<DB>) -> PersistenceResult<Self>
    where
        Self: Sized + Sync + Send;

    async fn after_create(&self) -> PersistenceResult<()>;

    async fn begin_transaction(&self) -> PersistenceResult<DatabaseTransaction<'_>>;
}

#[async_trait::async_trait]
impl Datastore<sqlx::Postgres> for DatastorePool {
    fn get_pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        match self {
            DatastorePool::Postgres(pool) => pool,
            _ => unreachable!("Invalid database type"),
        }
    }

    async fn new_with_pool(pool: sqlx::Pool<sqlx::Postgres>) -> PersistenceResult<Self> {
        Ok(DatastorePool::Postgres(pool))
    }

    async fn after_create(&self) -> PersistenceResult<()> {
        // Implement after_create for PostgreSQL
        Ok(())
    }

    async fn begin_transaction(&self) -> PersistenceResult<DatabaseTransaction<'_>> {
        self.begin_transaction::<sqlx::Postgres>().await
    }
}

#[async_trait::async_trait]
impl Datastore<sqlx::Sqlite> for DatastorePool {
    fn get_pool(&self) -> &sqlx::Pool<sqlx::Sqlite> {
        match self {
            DatastorePool::Sqlite(pool) => pool,
            _ => unreachable!("Invalid database type"),
        }
    }

    async fn new_with_pool(pool: sqlx::Pool<sqlx::Sqlite>) -> PersistenceResult<Self> {
        Ok(DatastorePool::Sqlite(pool))
    }

    async fn after_create(&self) -> PersistenceResult<()> {
        // Implement after_create for SQLite
        Ok(())
    }

    async fn begin_transaction(&self) -> PersistenceResult<DatabaseTransaction<'_>> {
        self.begin_transaction::<sqlx::Sqlite>().await
    }
}

#[async_trait::async_trait]
pub trait RepoImpl<DB>
where
    DB: sqlx::Database,
{
    fn new_with_datastore(datastore: Box<dyn Datastore<DB>>) -> PersistenceResult<Self>
    where
        Self: Sized + Sync + Send + 'static;
}

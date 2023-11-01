use crate::error::PersistenceResult;

mod sqlite;

#[async_trait::async_trait]
pub trait Datastore<T> {
    fn create(&self, item: T) -> PersistenceResult<()>;
    fn read(&self, id: u32) -> PersistenceResult<()>;
    fn update(&self, item: T) -> PersistenceResult<()>;
    fn delete(&self, id: u32) -> PersistenceResult<()>;
}

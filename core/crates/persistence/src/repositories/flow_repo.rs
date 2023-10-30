use anything_graph::Flow;
use sqlx::SqliteConnection;

use crate::{datastore::Datastore, error::PersistenceResult};

#[async_trait::async_trait]
pub trait FlowRepo {
    async fn create_flow(&self, flow: Flow) -> PersistenceResult<()>;
    async fn get_flows(&self) -> PersistenceResult<Vec<Flow>>;
}

#[derive(Debug)]
pub struct FlowRepoImpl {
    #[cfg(debug_assertions)]
    pub pool: SqliteConnection,
}

impl<Flow> Datastore<Flow> for FlowRepoImpl {
    fn create(&self, item: Flow) -> PersistenceResult<()> {
        todo!()
    }

    fn read(&self, id: u32) -> PersistenceResult<()> {
        todo!()
    }

    fn update(&self, item: Flow) -> PersistenceResult<()> {
        todo!()
    }

    fn delete(&self, id: u32) -> PersistenceResult<()> {
        todo!()
    }
}

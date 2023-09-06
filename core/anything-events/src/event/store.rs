use chrono::{DateTime, Utc};
use sqlx::Database;

use crate::event::store::store_query::StoreQuery;
use crate::types::AnythingResult;

pub mod ident;
pub mod sqlite;
pub mod store_query;
pub mod stream_query;

#[derive(Debug, PartialEq, Eq)]
pub enum SaveStatus {
    Ok,
    Duplicate,
}

pub type SaveResult = AnythingResult<SaveStatus>;

#[async_trait::async_trait]
pub trait StoreAdapter {
    async fn init<'a>(&'a self) -> AnythingResult<SaveStatus>;
    async fn save<'a, E: Send + Sync>(&'a self, event: &'a E) -> AnythingResult<SaveStatus>;
    async fn read<'a, D: Database + Send + Sync, E: Send + Sync + Clone>(
        &'a self,
        query: &'a StoreQuery<'a, D, E>,
        since: Option<DateTime<Utc>>,
    ) -> AnythingResult<Vec<E>>;
}

#[derive(Debug)]
pub struct Store<SA: StoreAdapter> {
    pub store: SA,
}

impl<SA> Store<SA>
where
    SA: StoreAdapter,
{
    pub fn new(store: SA) -> Self {
        Self { store }
    }

    pub fn initialize(&self) {}
}

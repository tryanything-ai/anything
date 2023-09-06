use crate::types::AnythingResult;
use crate::Event;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

pub mod ident;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub mod store_query;

pub type SaveResult = AnythingResult<bool>;
pub type FetchResult = AnythingResult<Vec<JsonValue>>;

#[async_trait::async_trait]
pub trait StoreAdapter {
    async fn init(&self) -> SaveResult;
    async fn save(&self, event: Event) -> SaveResult;
    async fn read(&self, since: Option<DateTime<Utc>>) -> FetchResult;
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

use std::sync::Arc;

use crate::{config::AnythingEventsConfig, errors::EventsResult, store::store::Store};

#[derive(Clone, Debug)]
pub struct Context {
    pub config: AnythingEventsConfig,
    pub store: Arc<Box<Store>>,
}

impl Context {
    pub async fn new(config: AnythingEventsConfig) -> EventsResult<Self> {
        let store = Store::from_config(&config).await?;

        Ok(Self {
            config,
            store: Arc::new(Box::new(store)),
        })
    }

    pub fn config(&self) -> &AnythingEventsConfig {
        &self.config
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[cfg(test)]
mod tests {}

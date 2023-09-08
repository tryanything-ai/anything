use std::sync::Arc;

use anything_core::AnythingConfig;

use crate::{errors::EventsResult, store::store::Store};

#[derive(Clone, Debug)]
pub struct Context {
    pub config: AnythingConfig,
    pub store: Arc<Box<Store>>,
}

impl Context {
    pub async fn new(config: AnythingConfig) -> EventsResult<Self> {
        let store = Store::from_config(&config).await?;

        Ok(Self {
            config,
            store: Arc::new(Box::new(store)),
        })
    }

    pub fn config(&self) -> &AnythingConfig {
        &self.config
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[cfg(test)]
mod tests {}

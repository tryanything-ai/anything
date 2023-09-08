use std::sync::Arc;

use crate::{config::Config, store::store::Store, EvtResult};

#[derive(Clone, Debug)]
pub struct Context {
    pub config: Config,
    pub store: Arc<Box<Store>>,
}

impl Context {
    pub async fn new(config: Config) -> EvtResult<Self> {
        let store = Store::from_config(&config).await?;

        Ok(Self {
            config,
            store: Arc::new(Box::new(store)),
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[cfg(test)]
mod tests {}

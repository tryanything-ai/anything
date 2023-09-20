// use serde::{Deserialize, Serialize};
use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    config::AnythingEventsConfig,
    db::create_sqlite_pool,
    errors::EventsResult,
    models::system_handler::SystemHandler,
    repositories::{event_repo::EventRepoImpl, flow_repo::FlowRepoImpl, Repositories},
};

// TODO: make #[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Context {
    pub pool: Arc<SqlitePool>,
    pub config: AnythingEventsConfig,
    pub repositories: Arc<Repositories>,
    pub system_handler: Arc<SystemHandler>,
}

impl Context {
    pub async fn new(config: AnythingEventsConfig) -> EventsResult<Self> {
        // let store = bootstrap_store(&config).await?;
        let pool = create_sqlite_pool(&config).await?;
        let repositories = Repositories {
            event_repo: EventRepoImpl::new(&pool),
            flow_repo: FlowRepoImpl::new(&pool),
        };
        let system_handler = Arc::new(SystemHandler::new(config.clone()));
        Ok(Self {
            config,
            pool: Arc::new(pool),
            repositories: Arc::new(repositories),
            system_handler,
        })
    }

    pub fn config(&self) -> &AnythingEventsConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {}

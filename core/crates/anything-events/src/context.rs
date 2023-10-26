// use serde::{Deserialize, Serialize};
use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    config::AnythingEventsConfig,
    db::create_sqlite_pool,
    errors::EventsResult,
    post_office::PostOffice,
    repositories::{
        event_repo::EventRepoImpl, flow_repo::FlowRepoImpl, trigger_repo::TriggerRepoImpl,
        Repositories,
    },
};

// TODO: make #[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Context {
    pub pool: Arc<SqlitePool>,
    pub config: AnythingEventsConfig,
    pub repositories: Arc<Repositories>,
    // pub system_handler: Arc<&'static Mutex<SystemHandler>>,
    pub post_office: Arc<PostOffice>,
}

impl Context {
    pub async fn new(config: AnythingEventsConfig) -> EventsResult<Self> {
        // let store = bootstrap_store(&config).await?;
        let pool = create_sqlite_pool(&config).await?;
        let repositories = Repositories {
            event_repo: EventRepoImpl::new(&pool),
            flow_repo: FlowRepoImpl::new(&pool),
            trigger_repo: TriggerRepoImpl::new(&pool),
        };
        // let system_handler = Arc::new(SystemHandler::global().clone());
        // let system_handler = Arc::new(SystemHandler::new(config.clone()));
        Ok(Self {
            config,
            pool: Arc::new(pool),
            repositories: Arc::new(repositories),
            post_office: Arc::new(PostOffice::open()),
            // system_handler,
        })
    }

    pub fn config(&self) -> &AnythingEventsConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {}

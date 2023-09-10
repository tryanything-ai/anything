use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    config::AnythingEventsConfig,
    db::create_sqlite_pool,
    errors::EventsResult,
    repositories::{event_repo::EventRepoImpl, Repositories},
};

#[derive(Clone, Debug)]
pub struct Context {
    pub pool: Arc<SqlitePool>,
    pub config: AnythingEventsConfig,
    pub repositories: Arc<Repositories>,
}

impl Context {
    pub async fn new(config: AnythingEventsConfig) -> EventsResult<Self> {
        // let store = bootstrap_store(&config).await?;
        let pool = create_sqlite_pool(&config).await?;
        let repositories = Repositories {
            event_repo: EventRepoImpl::new(&pool),
        };
        Ok(Self {
            config,
            pool: Arc::new(pool),
            repositories: Arc::new(repositories),
        })
    }

    pub fn config(&self) -> &AnythingEventsConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {}

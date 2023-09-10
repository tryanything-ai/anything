use sqlx::SqlitePool;

use crate::{
    errors::EventsResult,
    models::tag::{Tag, TagId},
};

#[derive(Debug, Clone)]
pub struct TagRepoImpl {
    pool: SqlitePool,
}

impl TagRepoImpl {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
pub trait TagRepo {}

#[async_trait::async_trait]
impl TagRepo for TagRepoImpl {}

use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct TriggerRepoImpl {
    #[cfg(debug_assertions)]
    pub pool: SqlitePool,
}

impl TriggerRepoImpl {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
pub trait FlowRepo {
    // async fn get_flows(&self) -> EventsResult<Flow>;
    // async fn get_flow_by_id(&self, event_id: EventId) -> EventsResult<Event>;
}

#[async_trait::async_trait]
impl FlowRepo for TriggerRepoImpl {}

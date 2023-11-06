use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use super::flow::FlowId;

pub type EventId = String;
pub type EventList = Vec<StoreEvent>;

/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct StoreEvent {
    pub id: String,
    pub flow_id: Option<FlowId>,
    pub trigger_id: Option<String>,
    pub name: String,
    pub context: Value,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateEvent {
    pub name: String,
    pub flow_id: Option<String>,
    pub trigger_id: Option<String>,
    pub context: Value,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

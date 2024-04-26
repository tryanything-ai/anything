use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use super::flow::FlowId;

pub type EventId = String;
pub type EventList = Vec<StoreEvent>;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct StoreEvent {
    pub event_id: String,
    pub event_status: String,
    pub flow_id: Option<FlowId>,
    pub flow_version_id: Option<String>,
    pub flow_version_name: Option<String>,
    pub trigger_id: Option<String>,
    pub trigger_session_id: Option<String>,
    pub trigger_session_status: String,
    pub flow_session_id: Option<String>,
    pub flow_session_status: String,
    pub node_id: String,
    pub is_trigger: bool,
    pub extension_id: String,
    pub stage: String,
    pub config: Option<Value>,
    pub context: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateEvent {
    pub event_id: String,
    pub event_status: String,
    pub flow_id: Option<FlowId>,
    pub flow_version_id: Option<String>,
    pub flow_version_name: Option<String>,
    pub trigger_id: Option<String>,
    pub trigger_session_id: Option<String>,
    pub trigger_session_status: String,
    pub flow_session_id: Option<String>,
    pub flow_session_status: String,
    pub node_id: String,
    pub is_trigger: bool,
    pub extension_id: String,
    pub stage: String,
    pub config: Option<Value>,
    pub context: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
}

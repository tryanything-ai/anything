use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

pub type TriggerId = String;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct StoredTrigger {
    pub trigger_id: TriggerId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Option<Value>,
    pub timestamp: Option<DateTime<Utc>>,
    // pub tags: Vec<String>,
}

impl StoredTrigger {
    pub fn new(event_name: String, payload: Value, metadata: Option<Value>) -> Self {
        Self {
            trigger_id: uuid::Uuid::new_v4().to_string(),
            event_name,
            payload,
            metadata,
            timestamp: Some(Utc::now()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTrigger {
    pub trigger_id: TriggerId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Option<Value>,
}

impl CreateTrigger {
    pub fn new(event_name: String, payload: Value, metadata: Option<Value>) -> Self {
        Self {
            trigger_id: uuid::Uuid::new_v4().to_string(),
            event_name,
            payload,
            metadata,
        }
    }
}

#![allow(unused)]
use std::collections::{HashMap, HashSet};

use anything_graph::flow::node::NodeState;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::SqlitePool;

use crate::generated::CreateTriggerRequest;

pub type TriggerId = String;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Trigger {
    pub trigger_id: TriggerId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Option<Value>,
    pub timestamp: Option<DateTime<Utc>>,
    // pub tags: Vec<String>,
}

impl Trigger {
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

impl Into<CreateTriggerRequest> for CreateTrigger {
    fn into(self) -> CreateTriggerRequest {
        CreateTriggerRequest {
            event_name: self.event_name,
            payload: self.payload.to_string(),
            metadata: match self.metadata {
                Some(m) => Some(m.to_string()),
                None => None,
            },
        }
    }
}

// impl Into<ProtoTrigger> for CreateTrigger {
//     fn into(self) -> ProtoTrigger {
//         let metadata = match self.metadata {
//             Some(m) => m,
//             None => Value::Null,
//         }
//         .to_string();
//         ProtoTrigger {
//             event_name: self.event_name,
//             payload: self.payload.to_string(),
//             metadata,
//             trigger_id: self.trigger_id,
//         }
//     }
// }

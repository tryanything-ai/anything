#![allow(unused)]
use std::collections::{HashMap, HashSet};

use anything_graph::flow::node::NodeState;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::SqlitePool;

pub type TriggerId = String;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Trigger {
    pub id: i64,
    pub trigger_id: TriggerId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Option<Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    // pub tags: Vec<String>,
}

// impl Into<ProtoTrigger> for Trigger {
//     fn into(self) -> ProtoTrigger {
//         ProtoTrigger {
//             event_name: self.event_name,
//             payload: self.payload.to_string(),
//             trigger_id: self.trigger_id,
//             metadata: match self.metadata {
//                 Some(m) => m.to_string(),
//                 None => "".to_string(),
//             },
//         }
//     }
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTrigger {
    pub trigger_id: TriggerId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Option<Value>,
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

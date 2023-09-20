#![allow(unused)]
use std::collections::{HashMap, HashSet};

use anything_graph::flow::node::NodeState;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::SqlitePool;

use crate::generated::Trigger as ProtoTrigger;

pub type TriggerId = String;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Trigger {
    pub id: i64,
    pub event_id: TriggerId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Value,
    pub timestamp: DateTime<Utc>,
    // pub tags: Vec<String>,
}

impl Into<ProtoTrigger> for Trigger {
    fn into(self) -> ProtoTrigger {
        ProtoTrigger {
            event_name: self.event_name,
            payload: self.payload.to_string(),
            metadata: self.metadata.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTrigger {
    pub event_name: String,
    pub payload: Value,
    pub metadata: Option<Value>,
}

impl Into<ProtoTrigger> for CreateTrigger {
    fn into(self) -> ProtoTrigger {
        let metadata = match self.metadata {
            Some(m) => m,
            None => Value::Null,
        }
        .to_string();
        ProtoTrigger {
            event_name: self.event_name,
            payload: self.payload.to_string(),
            metadata,
        }
    }
}

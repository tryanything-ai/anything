#![allow(unused)]
use std::collections::{HashMap, HashSet};

use anything_graph::flow::node::NodeState;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::SqlitePool;

use crate::errors::EventsResult;
use crate::generated::events::{CreateEvent as ProtoCreateEvent, Event as ProtoEvent};

use super::flow::FlowId;

pub type EventId = String;
pub type SourceId = String;
/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Event {
    pub id: String,
    pub flow_id: Option<FlowId>,
    pub trigger_id: Option<String>,
    pub name: String,
    pub context: Value,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    // pub tags: Vec<String>,
}
/*
id TEXT NOT NULL PRIMARY KEY,
    -- Not going to have both
    flow_id TEXT,
    trigger_id TEXT,
    name TEXT NOT NULL,
    context json NOT NULL,
    started_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    ended_at timestamp with time zone DEFAULT (CURRENT_TIMESTAMP)
 */

impl Into<ProtoEvent> for Event {
    fn into(self) -> ProtoEvent {
        use crate::generated::events::event::Source::{FlowId, TriggerId};
        let started_at = Some(match self.started_at {
            Some(t) => t.timestamp(),
            None => Utc::now().timestamp(),
        });
        let ended_at = Some(match self.ended_at {
            Some(t) => t.timestamp(),
            None => Utc::now().timestamp(),
        });
        ProtoEvent {
            id: self.id,
            context: self.context.to_string(),
            name: self.name,
            source: Some(match self.flow_id {
                Some(flow_id) => FlowId(flow_id),
                None => TriggerId(self.trigger_id.unwrap()),
            }),
            started_at,
            ended_at,
        }
    }
}

pub type EventList = Vec<Event>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateEvent {
    pub name: String,
    pub flow_id: Option<String>,
    pub trigger_id: Option<String>,
    pub context: Value,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Into<ProtoCreateEvent> for CreateEvent {
    fn into(self) -> ProtoCreateEvent {
        use crate::generated::events::event::Source::{FlowId, TriggerId};

        let started_at = Some(match self.started_at {
            Some(t) => t.timestamp(),
            None => Utc::now().timestamp(),
        });
        let ended_at = Some(match self.ended_at {
            Some(t) => t.timestamp(),
            None => Utc::now().timestamp(),
        });
        ProtoCreateEvent {
            event_name: self.name,
            payload: self.context.to_string(),
            metadata: None,
            trigger_id: self.trigger_id.unwrap(),
        }
    }
}

impl Event {}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NodeExecutionUpdate {
//     pub status: NodeState,
//     pub flow_uuid: String,
// }

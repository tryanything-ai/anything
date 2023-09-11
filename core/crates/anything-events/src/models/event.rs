#![allow(unused)]
use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::SqlitePool;

use crate::errors::EventsResult;
use crate::events::Event as ProtoEvent;
use crate::events::EventDetails;
use crate::events::EventIdentifier;
use crate::events::SourceIdentifier;

pub type EventId = i64;
/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Event {
    pub id: EventId,
    pub source_id: i64,
    pub event_name: String,
    // pub tags: TagList,
    pub payload: Value,
    pub metadata: Value,
    pub timestamp: DateTime<Utc>,
}

impl Into<ProtoEvent> for Event {
    fn into(self) -> ProtoEvent {
        ProtoEvent {
            identifier: Some(SourceIdentifier { source_id: self.id }),
            details: Some(EventDetails {
                name: self.event_name,
                tags: Vec::default(),
                payload: self.payload.to_string(),
                metadata: Some(self.metadata.to_string()),
            }),
        }
    }
}

pub type EventList = Vec<Event>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateEvent {
    pub source_id: EventId,
    pub event_name: String,
    // pub tags: TagList,
    pub payload: Value,
    pub metadata: Value,
}

impl Into<ProtoEvent> for CreateEvent {
    fn into(self) -> ProtoEvent {
        ProtoEvent {
            identifier: Some(SourceIdentifier {
                source_id: self.source_id,
            }),
            details: Some(EventDetails {
                name: self.event_name,
                payload: self.payload.to_string(),
                metadata: Some(self.metadata.to_string()),
                tags: Vec::default(),
            }),
        }
    }
}

impl Event {}

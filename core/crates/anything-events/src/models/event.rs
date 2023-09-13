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

pub type EventId = String;
pub type SourceId = String;
/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Event {
    pub id: i64,
    pub event_id: EventId,
    pub source_id: SourceId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Value,
    pub timestamp: DateTime<Utc>,
    // pub tags: Vec<String>,
}

impl Into<ProtoEvent> for Event {
    fn into(self) -> ProtoEvent {
        ProtoEvent {
            source_id: self.source_id,
            event_id: self.event_id,
            name: self.event_name,
            payload: self.payload.to_string(),
            metadata: Some(self.metadata.to_string()),
            // tags: Vec::default(),
        }
    }
}

pub type EventList = Vec<Event>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateEvent {
    pub event_id: String,
    pub source_id: EventId,
    pub event_name: String,
    pub payload: Value,
    pub metadata: Value,
    // pub tags: Vec<String>,
}

impl Into<ProtoEvent> for CreateEvent {
    fn into(self) -> ProtoEvent {
        ProtoEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            source_id: self.source_id,
            name: self.event_name,
            payload: self.payload.to_string(),
            metadata: Some(self.metadata.to_string()),
            // tags: self.tags,
        }
    }
}

impl Event {}

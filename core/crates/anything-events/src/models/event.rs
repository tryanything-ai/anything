#![allow(unused)]
use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::SqlitePool;

use crate::errors::EventsResult;

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

pub type EventList = Vec<Event>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateEvent {
    pub source_id: EventId,
    pub event_name: String,
    // pub tags: TagList,
    pub payload: Value,
    pub metadata: Value,
}

impl Event {}

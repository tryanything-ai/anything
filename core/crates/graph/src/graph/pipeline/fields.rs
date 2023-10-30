use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Field {
    Uint(u64),
    Boolean(bool),
    String(String),
    Text(String),
    Binary(#[serde(with = "serde_bytes")] Vec<u8>),
    Timestamp(DateTime<FixedOffset>),
    Date(NaiveDate),
    Json(serde_json::Value),
    Duration(String),
    Null,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldType {
    UInt,
    Boolean,
    String,
    Text,
    Binary,
    Timestamp,
    Date,
    Json,
    Duration,
}

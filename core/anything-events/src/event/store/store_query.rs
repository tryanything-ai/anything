#![allow(unused)]
use std::collections::HashMap;

use serde_json::Value as JsonValue;

#[derive(Clone)]
pub enum StoreQueryTypes<'a> {
    Query(QueryOptions<'a>),
    Insert(InsertOptions<'a>),
}

#[derive(Clone)]
pub struct QueryOptions<'a> {
    origin: Option<i64>,
    last_event_id: Option<i64>,
    end: Option<&'a str>,
}

#[derive(Clone)]
pub struct InsertOptions<'a> {
    id: Option<u64>,
    name: Option<String>,
    payload: Option<JsonValue>,
    metadata: Option<HashMap<String, String>>,
    returning: Option<&'a str>,
    tags: Option<Vec<String>>,
}

#[async_trait::async_trait]
pub trait StoreQuery {
    fn build(&mut self) -> &str;
}

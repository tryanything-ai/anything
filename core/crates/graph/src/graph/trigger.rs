use anything_runtime::{DeserializeValue, ValueKind};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::error::GraphResult;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Trigger {
    Empty(EmptyTrigger),
}

impl Default for Trigger {
    fn default() -> Self {
        Trigger::Empty(EmptyTrigger::default())
    }
}

pub trait TriggerType {
    fn kind(&self) -> &'static str;
    fn is_match(&self, root_url: &str, payload: &DeserializeValue) -> GraphResult<bool>;
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmptyTrigger {
    #[serde(default)]
    pub settings: Option<indexmap::IndexMap<String, ValueKind>>,
}

impl Default for EmptyTrigger {
    fn default() -> Self {
        Self { settings: None }
    }
}

impl TriggerType for EmptyTrigger {
    fn kind(&self) -> &'static str {
        "empty"
    }
    fn is_match(&self, event_path: &str, _payload: &DeserializeValue) -> GraphResult<bool> {
        Ok(event_path.starts_with("empty"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TriggerFilterPredicate {
    Empty,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerFilter(pub Vec<TriggerFilterPredicate>);

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerParams(pub IndexMap<String, String>);

impl TriggerFilter {
    // pub fn matches(&self, )
}

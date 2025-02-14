use std::collections::HashMap;

use super::react_flow_types::{HandleProps, NodePresentation};
use node_semver::{Range, Version};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::json_schema::JsonSchema;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginName(String);

impl PluginName {
    pub fn new(name: String) -> Result<Self, &'static str> {
        if name.starts_with('@') && name.contains('/') {
            Ok(PluginName(name))
        } else {
            Err("Plugin name must be in format @namespace/item")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for PluginName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PluginName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        PluginName::new(name).map_err(de::Error::custom)
    }
}

impl std::fmt::Display for PluginName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub anything_action_version: Version,
    pub r#type: ActionType,
    pub plugin_name: PluginName, //We will use this to fetch current plugin schema etc from database vs in the workflow
    pub plugin_version: Version,
    pub action_id: String, // This is a local action_id for in the workflow
    pub label: String,
    pub description: Option<String>,
    pub icon: String,
    pub inputs: Option<Value>,
    pub inputs_locked: Option<bool>,
    pub inputs_schema: Option<JsonSchema>,
    pub inputs_schema_locked: Option<bool>,
    pub plugin_config: Value,
    pub plugin_config_locked: Option<bool>,
    pub plugin_config_schema: JsonSchema,
    pub plugin_config_schema_locked: Option<bool>,
    pub presentation: Option<NodePresentation>,
    pub handles: Option<Vec<HandleProps>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Trigger,  // Trigger action
    Action,   // General action
    Loop,     // Loop action
    Decision, // Decision action
    Filter,   // Filter action
    Response, // Response action for making api endpoints
    Input,    // Input action for subflows
    Output,   // Output action for subflows
}

impl ActionType {
    pub fn as_str(&self) -> &str {
        match self {
            ActionType::Input => "input",
            ActionType::Trigger => "trigger",
            ActionType::Response => "response",
            ActionType::Action => "action",
            ActionType::Loop => "loop",
            ActionType::Decision => "decision",
            ActionType::Filter => "filter",
            ActionType::Output => "output",
        }
    }
}

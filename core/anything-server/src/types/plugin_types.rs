use super::react_flow_types::{HandleProps, NodePresentation};
use node_semver::Version;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::action_types::ActionType;
use crate::types::json_schema::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plugin {
    pub r#type: ActionType,
    pub featured: bool,
    pub action_template_definition: ActionTemplateDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionTemplateDefinition {
    pub anything_action_version: Version,
    pub r#type: ActionType,
    pub plugin_name: String,
    pub plugin_version: Version,
    pub action_id: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: String,
    pub inputs: Option<Value>,
    pub inputs_locked: bool,
    pub inputs_schema: Option<JsonSchema>,
    pub inputs_schema_locked: bool,
    pub plugin_config: Value,
    pub plugin_config_locked: bool,
    // pub plugin_config_schema: JsonSchema,
    pub plugin_config_schema_locked: bool,
    pub presentation: Option<NodePresentation>,
    pub handles: Option<Vec<HandleProps>>,
}

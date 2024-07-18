use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{DateTime, Timelike, Utc};
use serde_json::Value;
use uuid::Uuid;

use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub actions: Vec<Action>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub enum PluginType {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "trigger")]
    Trigger,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "decision")]
    Decision,
    #[serde(rename = "filter")]
    Filter,
    #[serde(rename = "output")]
    Output,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Action {
    pub anything_action_version: String,
    pub r#type: PluginType,
    pub plugin_id: String,
    pub node_id: String,
    pub plugin_version: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: String,
    pub variables: Option<Variable>,
    pub variables_schema: Option<Variable>,
    pub input: Variable,
    pub input_schema: Variable,
    pub presentation: Option<NodePresentation>,
    pub handles: Option<Vec<HandleProps>>,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct NodePresentation {
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Variable {
    #[serde(flatten)]
    pub inner: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HandleProps {
    pub id: String,
    pub r#type: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub source_handle: Option<String>,
    pub target: String,
    pub target_handle: Option<String>,
    pub r#type: String,
}

impl Workflow {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTaskInput {
    pub account_id: String,
    pub task_status: String,
    pub flow_id: String,
    pub flow_version_id: String,
    pub flow_version_name: String,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub node_id: String,
    pub is_trigger: bool,
    pub plugin_id: String,
    pub stage: String,
    pub config: Value,
    pub test_config: Option<Value>, // context: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskConfig {
    pub variables: Value,
    pub inputs: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    pub action_id: Option<String>, //if action_id is present, then we are testing just an action
    pub variables: Value,
    pub inputs: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task_id: Uuid,
    pub account_id: Uuid,
    pub task_status: String,
    pub flow_id: Uuid,
    pub flow_version_id: Uuid,
    pub flow_version_name: Option<String>,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub node_id: String,
    pub is_trigger: bool,
    pub plugin_id: Option<String>,
    pub stage: String,
    pub test_config: Option<Value>,
    pub config: Value,
    pub context: Option<Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
    pub archived: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Trigger {
    pub id: i32,
    pub cron_expression: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowVersion {
    pub flow_version_id: Uuid,
    pub flow_id: Uuid,
    pub flow_version: String,
    pub flow_definition: Value,
    // other fields as necessary
}

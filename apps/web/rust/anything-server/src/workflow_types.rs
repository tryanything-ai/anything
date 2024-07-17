use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
// #[serde(rename_all = "camelCase")]
pub struct HandleProps {
    // Define the fields of HandleProps based on the reactflow library
    // Add fields as per your requirement
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Edge {
    // Define the fields of Edge based on the reactflow library
    // Add fields as per your requirement
}

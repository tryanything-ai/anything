use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::task_types::ActionType;

use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub actions: Vec<Action>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Action {
    pub anything_action_version: String,
    pub action_type: ActionType,
    pub plugin_id: String,
    pub node_id: String,
    pub plugin_version: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: String,
    pub variables: Variable,
    pub variables_schema: Variable,
    pub input: Variable,
    pub input_schema: Variable,
    pub presentation: Option<NodePresentation>,
    pub handles: Option<Vec<HandleProps>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePresentation {
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub action_label: String,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub node_id: String,
    pub action_type: ActionType,
    pub plugin_id: String,
    pub stage: String,
    pub config: Value,
    pub test_config: Option<Value>, // context: Value,
    pub processing_order: i32,
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
    pub action_label: String,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub node_id: String,
    pub action_type: String, //Needed for UI to know what type of thing to show. ( loops vs forks vs triggers vs actions etc )
    pub plugin_id: Option<String>, //Needed for plugin engine to process it with a plugin.
    pub stage: String,
    pub test_config: Option<Value>,
    pub config: Value,
    pub context: Option<Value>, //TODO: probably delete so we don't leak secrets
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
    pub archived: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub processing_order: i32,
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
    pub flow_definition: Value,
}

impl Default for Workflow {
    fn default() -> Self {
        let action1 = Action {
            anything_action_version: "0.1.0".to_string(),
            action_type: ActionType::Trigger,
            plugin_id: "cron".to_string(),
            node_id: "cron_trigger".to_string(),
            plugin_version: "0.1.0".to_string(),
            label: "Every Hour".to_string(),
            description: Some("Cron Trigger to run workflow every hour".to_string()),
            icon: "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"lucide lucide-clock\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><polyline points=\"12 6 12 12 16 14\"/></svg>".to_string(),
            variables: Variable {
                inner: HashMap::new(),
            },
            variables_schema: Variable {
                inner: HashMap::new(),
            },
            input: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("cron_expression".to_string(), serde_json::json!("0 */2 * * * *"));
                    map
                },
            },
            input_schema: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("type".to_string(), serde_json::json!("object"));
                    map.insert("properties".to_string(), serde_json::json!({
                        "cron_expression": {
                            "title": "Cron Expression",
                            "description": "When to run the trigger",
                            "type": "string"
                        }
                    }));
                    map.insert("x-jsf-order".to_string(), serde_json::json!(["cron_expression"]));
                    map.insert("required".to_string(), serde_json::json!(["cron_expression"]));
                    map.insert("additionalProperties".to_string(), serde_json::json!(false));
                    map
                },
            },
            presentation: Some(NodePresentation {
                position: Position { x: 300.0, y: 100.0 },
            }),
            handles: Some(vec![HandleProps {
                id: "b".to_string(),
                r#type: "source".to_string(),
                position: "bottom".to_string(),
            }]),
        };

        let action2 = Action {
            anything_action_version: "0.1.0".to_string(),
            action_type: ActionType::Action,
            plugin_id: "http".to_string(),
            node_id: "http_action".to_string(),
            plugin_version: "0.1.0".to_string(),
            label: "Call External System".to_string(),
            description: Some("Use HTTP to call another system".to_string()),
            icon: "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 320\"><path d=\"m297.06 130.97c7.26-21.79 4.76-45.66-6.85-65.48-17.46-30.4-52.56-46.04-86.84-38.68-15.25-17.18-37.16-26.95-60.13-26.81-35.04-.08-66.13 22.48-76.91 55.82-22.51 4.61-41.94 18.7-53.31 38.67-17.59 30.32-13.58 68.54 9.92 94.54-7.26 21.79-4.76 45.66 6.85 65.48 17.46 30.4 52.56 46.04 86.84 38.68 15.24 17.18 37.16 26.95 60.13 26.8 35.06.09 66.16-22.49 76.94-55.86 22.51-4.61 41.94-18.7 53.31-38.67 17.57-30.32 13.55-68.51-9.94-94.51zm-120.28 168.11c-14.03.02-27.62-4.89-38.39-13.88.49-.26 1.34-.73 1.89-1.07l63.72-36.8c3.26-1.85 5.26-5.32 5.24-9.07v-89.83l26.93 15.55c.29.14.48.42.52.74v74.39c-.04 33.08-26.83 59.9-59.91 59.97zm-128.84-55.03c-7.03-12.14-9.56-26.37-7.15-40.18.47.28 1.3.79 1.89 1.13l63.72 36.8c3.23 1.89 7.23 1.89 10.47 0l77.79-44.92v31.1c.02.32-.13.63-.38.83l-64.41 37.19c-28.69 16.52-65.33 6.7-81.92-21.95zm-16.77-139.09c7-12.16 18.05-21.46 31.21-26.29 0 .55-.03 1.52-.03 2.2v73.61c-.02 3.74 1.98 7.21 5.23 9.06l77.79 44.91-26.93 15.55c-.27.18-.61.21-.91.08l-64.42-37.22c-28.63-16.58-38.45-53.21-21.95-81.89zm221.26 51.49-77.79-44.92 26.93-15.54c.27-.18.61-.21.91-.08l64.42 37.19c28.68 16.57 38.51 53.26 21.94 81.94-7.01 12.14-18.05 21.44-31.2 26.28v-75.81c.03-3.74-1.96-7.2-5.2-9.06zm26.8-40.34c-.47-.29-1.3-.79-1.89-1.13l-63.72-36.8c-3.23-1.89-7.23-1.89-10.47 0l-77.79 44.92v-31.1c-.02-.32.13-.63.38-.83l64.41-37.16c28.69-16.55 65.37-6.7 81.91 22 6.99 12.12 9.52 26.31 7.15 40.1zm-168.51 55.43-26.94-15.55c-.29-.14-.48-.42-.52-.74v-74.39c.02-33.12 26.89-59.96 60.01-59.94 14.01 0 27.57 4.92 38.34 13.88-.49.26-1.33.73-1.89 1.07l-63.72 36.8c-3.26 1.85-5.26 5.31-5.24 9.06l-.04 89.79zm14.63-31.54 34.65-20.01 34.65 20v40.01l-34.65 20-34.65-20z\"/></svg>".to_string(),
            variables: Variable {
                inner: HashMap::new(),
            },
            variables_schema: Variable {
                inner: HashMap::new(),
            },
            input: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("method".to_string(), serde_json::json!("GET"));
                    map.insert("url".to_string(), serde_json::json!("https://hp-api.onrender.com/api/character/9e3f7ce4-b9a7-4244-b709-dae5c1f1d4a8"));
                    map.insert("headers".to_string(), serde_json::json!(""));
                    map.insert("body".to_string(), serde_json::json!(""));
                    map
                },
            },
            input_schema: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("type".to_string(), serde_json::json!("object"));
                    map.insert("properties".to_string(), serde_json::json!({
                        "method": {
                            "title": "Method",
                            "description": "HTTP Method for request",
                            "type": "string",
                            "oneOf": [
                                { "value": "GET", "title": "GET" },
                                { "value": "POST", "title": "POST" },
                                { "value": "PUT", "title": "PUT" },
                                { "value": "DELETE", "title": "DELETE" }
                            ],
                            "x-jsf-presentation": {
                                "inputType": "select"
                            }
                        },
                        "url": {
                            "title": "URL",
                            "description": "URL for request",
                            "type": "string"
                        },
                        "headers": {
                            "title": "Headers",
                            "description": "Headers for request",
                            "type": "string"
                        },
                        "body": {
                            "title": "Body",
                            "description": "Body for request",
                            "type": "string"
                        }
                    }));
                    map.insert("x-jsf-order".to_string(), serde_json::json!(["url", "method", "headers", "body"]));
                    map.insert("required".to_string(), serde_json::json!(["method", "url"]));
                    map.insert("additionalProperties".to_string(), serde_json::json!(false));
                    map
                },
            },
            presentation: Some(NodePresentation {
                position: Position { x: 300.0, y: 300.0 },
            }),
            handles: Some(vec![
                HandleProps {
                    id: "a".to_string(),
                    r#type: "target".to_string(),
                    position: "top".to_string(),
                },
                HandleProps {
                    id: "b".to_string(),
                    r#type: "source".to_string(),
                    position: "bottom".to_string(),
                }
            ]),
        };

        let edge = Edge {
            id: "cron_trigger->http_action".to_string(),
            r#type: "anything".to_string(),
            source: "cron_trigger".to_string(),
            target: "http_action".to_string(),
            source_handle: Some("b".to_string()),
            target_handle: Some("a".to_string()),
        };

        Workflow {
            actions: vec![action1, action2],
            edges: vec![edge],
        }
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::task_types::ActionType;

use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workflow {
    pub actions: Vec<Action>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub anything_action_version: String,
    pub r#type: ActionType,
    pub plugin_id: String,
    pub action_id: String,
    pub plugin_version: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: String,
    pub variables: Variable,
    pub variables_locked: Option<bool>,
    pub variables_schema: Variable,
    pub variables_schema_locked: Option<bool>,
    pub input: Variable,
    pub input_locked: Option<bool>,
    pub input_schema: Variable,
    pub input_schema_locked: Option<bool>,
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
    pub action_id: String,
    pub r#type: ActionType,
    pub plugin_id: String,
    pub stage: String,
    pub config: Value,
    pub result: Option<Value>,
    pub test_config: Option<Value>, // context: Value,
    pub processing_order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskConfig {
    pub variables: Value,
    pub input: Value,
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
    pub action_id: String,
    pub r#type: String, //Needed for UI to know what type of thing to show. ( loops vs forks vs triggers vs actions etc )
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
    pub published: bool,
    pub account_id: Uuid,
    pub flow_definition: Value,
}

impl Default for Workflow {
    fn default() -> Self {
        let action1 = Action {
            anything_action_version: "0.1.0".to_string(),
            r#type: ActionType::Trigger,
            plugin_id: "cron".to_string(),
            action_id: "cron_trigger".to_string(),
            plugin_version: "0.1.0".to_string(),
            label: "Every Hour".to_string(),
            description: Some("Cron Trigger to run workflow every hour".to_string()),
            icon: "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"lucide lucide-clock\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><polyline points=\"12 6 12 12 16 14\"/></svg>".to_string(),
            variables: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("cron_expression".to_string(), serde_json::json!("0 0 * * * *"));
                    map
                },
            },
            variables_locked: Some(false),
            variables_schema: Variable {
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
            variables_schema_locked: Some(true),
            input: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("cron_expression".to_string(), serde_json::json!("{{variables.cron_expression}}"));
                    map
                },
            },
            input_locked: Some(true),
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
            input_schema_locked: Some(true),
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
            r#type: ActionType::Action,
            plugin_id: "http".to_string(),
            action_id: "http".to_string(),
            plugin_version: "0.1.0".to_string(),
            label: "Call External System".to_string(),
            description: Some("Use HTTP to call another system".to_string()),
            icon: "<svg fill=\"#000000\" width=\"800px\" height=\"800px\" viewBox=\"0 0 32 32\" id=\"icon\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style>.cls-1{fill:none;}</style></defs><title>HTTP</title><path d=\"M30,11H25V21h2V18h3a2.0027,2.0027,0,0,0,2-2V13A2.0023,2.0023,0,0,0,30,11Zm-3,5V13h3l.001,3Z\" transform=\"translate(0 0)\"/><polygon points=\"10 13 12 13 12 21 14 21 14 13 16 13 16 11 10 11 10 13\"/><polygon points=\"23 11 17 11 17 13 19 13 19 21 21 21 21 13 23 13 23 11\"/><polygon points=\"6 11 6 15 3 15 3 11 1 11 1 21 3 21 3 17 6 17 6 21 8 21 8 11 6 11\"/><rect id=\"_Transparent_Rectangle_\" data-name=\"&lt;Transparent Rectangle&gt;\" class=\"cls-1\" width=\"32\" height=\"32\"/></svg>".to_string(),
            variables: Variable {
                inner: HashMap::new(),
            },
            variables_locked: Some(false),
            variables_schema: Variable {
                inner: HashMap::new(),
            },
            variables_schema_locked: Some(false),
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
            input_locked: Some(false),
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
                                        {
                                            "value": "GET",
                                            "title": "GET"
                                        },
                                        {
                                            "value": "POST",
                                            "title": "POST"
                                        },
                                        {
                                            "value": "PUT",
                                            "title": "PUT"
                                        },
                                        {
                                            "value": "DELETE",
                                            "title": "DELETE"
                                        },
                                        {
                                            "value": "HEAD",
                                            "title": "HEAD"
                                        },
                                        { 
                                            "value": "OPTIONS",
                                            "title": "OPTIONS"
                                        },
                                        {
                                            "value": "PATCH", 
                                            "title": "PATCH"
                                        }
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
            input_schema_locked: Some(true),
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
            id: "cron_trigger->http".to_string(),
            r#type: "anything".to_string(),
            source: "cron_trigger".to_string(),
            target: "http".to_string(),
            source_handle: Some("b".to_string()),
            target_handle: Some("a".to_string()),
        };

        Workflow {
            actions: vec![action1, action2],
            edges: vec![edge],
        }
    }
}

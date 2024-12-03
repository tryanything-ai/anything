use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::types::action_types::ActionType;
use crate::types::react_flow_types::{Edge, HandleProps, NodePresentation, Position};
use crate::types::action_types::Action;
use crate::types::general::Variable;

use super::action_types::{InputFieldType, JsonSchema, JsonSchemaProperty, PresentationField, ValidationField, ValidationFieldType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowVersionDefinition {
    pub actions: Vec<Action>,
    pub edges: Vec<Edge>,
}

impl WorkflowVersionDefinition {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowVersion {
    pub flow_version_id: Uuid,
    pub flow_id: Uuid,
    pub published: bool,
    pub account_id: Uuid,
    pub flow_definition: Value,
}

//DUPLICATING INTO NEW NAME FOR NEW PROCESSOR
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseFlowVersion {
    pub flow_version_id: Uuid,
    pub flow_id: Uuid,
    pub published: bool,
    pub account_id: Uuid,
    pub flow_definition: WorkflowVersionDefinition,
}

//TODO: Upgrade defaults to new action types
impl Default for WorkflowVersionDefinition {
    fn default() -> Self {
        let action1 = Action {
            anything_action_version: "0.1.0".to_string(),
            r#type: ActionType::Trigger,
            plugin_id: "cron".to_string(),
            action_id: "cron".to_string(),
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
            variables_schema: JsonSchema {
                r#type: "object".to_string(),
                properties: {
                    let mut map = HashMap::new();
                    map.insert("cron_expression".to_string(), JsonSchemaProperty {
                        title: Some("Cron Expression".to_string()),
                        description: Some("When to run the trigger".to_string()),
                        r#type: Some("string".to_string()),
                        one_of: None,
                        all_of: None,
                        x_any_validation: Some(ValidationField {
                            r#type: ValidationFieldType::String,
                        }),
                        x_jsf_presentation: None,
                    });
                    map
                },
                required: Some(vec!["cron_expression".to_string()]),
                x_jsf_order: Some(vec!["cron_expression".to_string()]),
                additional_properties: Some(false),
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
            input_schema: JsonSchema {
                r#type: "object".to_string(),
                properties: {
                    let mut map = HashMap::new();
                    map.insert("cron_expression".to_string(), JsonSchemaProperty {
                        title: Some("Cron Expression".to_string()),
                        description: Some("When to run the trigger".to_string()),
                        r#type: Some("string".to_string()),
                        one_of: None,
                        all_of: None,
                        x_any_validation: None,
                        x_jsf_presentation: None,
                    });
                    map
                },
                required: Some(vec!["cron_expression".to_string()]),
                x_jsf_order: Some(vec!["cron_expression".to_string()]),
                additional_properties: Some(false),
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
            variables_schema: JsonSchema {
                r#type: "object".to_string(),
                properties: HashMap::new(),
                required: None,
                x_jsf_order: None,
                additional_properties: None,
            },
            variables_schema_locked: Some(false),
            input: Variable {
                inner: {
                    let mut map = HashMap::new();
                    map.insert("method".to_string(), serde_json::json!("GET"));
                    map.insert("url".to_string(), serde_json::json!("https://hp-api.onrender.com/api/character/9e3f7ce4-b9a7-4244-b709-dae5c1f1d4a8"));
                    map.insert("headers".to_string(), serde_json::json!("{}"));
                    map.insert("body".to_string(), serde_json::json!("{}"));
                    map
                },
            },
            input_locked: Some(false),
            input_schema: JsonSchema {
                r#type: "object".to_string(),
                required: Some(vec!["method".to_string(), "url".to_string()]),
                x_jsf_order: Some(vec!["method".to_string(), "url".to_string(), "headers".to_string(), "body".to_string()]),
                additional_properties: Some(false), 
                properties: {
                    let mut map = HashMap::new();
                    map.insert("method".to_string(), JsonSchemaProperty {
                        title: Some("Method".to_string()),
                        description: Some("HTTP Method for request".to_string()),
                        r#type: Some("string".to_string()),
                        one_of: Some(vec![
                            serde_json::json!({
                                "value": "GET",
                                "title": "GET"
                            }),
                            serde_json::json!({
                                "value": "POST",
                                "title": "POST"
                            }),
                            serde_json::json!({
                                "value": "PUT",
                                "title": "PUT"
                            }),
                            serde_json::json!({
                                "value": "DELETE",
                                "title": "DELETE"
                            }),
                            serde_json::json!({
                                "value": "HEAD",
                                "title": "HEAD"
                            }),
                            serde_json::json!({
                                "value": "OPTIONS",
                                "title": "OPTIONS"
                            }),
                            serde_json::json!({
                                "value": "PATCH",
                                "title": "PATCH"
                            }),
                        ]),
                        all_of: None,
                        x_any_validation: None,
                        x_jsf_presentation: Some(PresentationField {
                            input_type: InputFieldType::SelectOrVariable,
                        }),
                    });
                    map.insert("url".to_string(), JsonSchemaProperty {
                        title: Some("URL".to_string()),
                        description: Some("URL for request".to_string()),
                        r#type: Some("string".to_string()),
                        one_of: None,
                        all_of: None,
                        x_any_validation: None,
                        x_jsf_presentation: Some(PresentationField {
                            input_type: InputFieldType::Text,
                        }),
                    });
                    map.insert("headers".to_string(), JsonSchemaProperty {
                        title: Some("Headers".to_string()),
                        description: Some("Headers for request".to_string()),
                        r#type: Some("object".to_string()),
                        one_of: None,
                        all_of: None,
                        x_any_validation: Some(ValidationField {
                            r#type: ValidationFieldType::Object,
                        }),
                        x_jsf_presentation: Some(PresentationField {
                            input_type: InputFieldType::ObjectOrVariable,
                        }),
                    });
                    map.insert("body".to_string(), JsonSchemaProperty {
                        title: Some("Body".to_string()),
                        description: Some("Body for request".to_string()),
                        r#type: Some("object".to_string()),
                        one_of: None,
                        all_of: None,
                        x_any_validation: Some(ValidationField {
                            r#type: ValidationFieldType::Object,
                        }),
                        x_jsf_presentation: Some(PresentationField {
                            input_type: InputFieldType::ObjectOrVariable,
                        }),
                    });                 
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
            id: "cron->http".to_string(),
            r#type: "anything".to_string(),
            source: "cron".to_string(),
            target: "http".to_string(),
            source_handle: Some("b".to_string()),
            target_handle: Some("a".to_string()),
        };

        WorkflowVersionDefinition {
            actions: vec![action1, action2],
            edges: vec![edge],
        }
    }
}



use extism_pdk::*;
use jsonschema::{is_valid, JSONSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, ToBytes, Debug, PartialEq, Clone)]
#[encoding(Json)]
pub struct Handle {
    pub id: String,
    pub position: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, ToBytes, Debug, PartialEq)]
#[encoding(Json)]
pub struct Action {
    pub trigger: bool,
    pub action_name: String,
    pub action_label: String,
    pub plugin_id: String,
    pub icon: String,
    pub description: String,
    pub handles: Vec<Handle>,
    pub variables: Vec<Value>,
    pub config: Value,        //a base config that actually works.
    pub config_schema: Value, // a json schema for the config.
}

impl Action {
    // This method will help users discover the builder
    pub fn builder() -> ActionBuilder {
        ActionBuilder::default()
    }
}

#[derive(Default)]
pub struct ActionBuilder {
    trigger: Option<bool>,
    action_name: Option<String>,
    action_label: Option<String>,
    icon: Option<String>,
    description: Option<String>,
    handles: Option<Vec<Handle>>,
    variables: Vec<Value>,
    config: Option<Value>,
    config_schema: Option<Value>,
    plugin_id: Option<String>,
}

impl ActionBuilder {
    pub fn new() -> ActionBuilder {
        // Set the minimally required fields of Action.
        ActionBuilder {
            trigger: Some(false),
            //TODO: maybe add a "flow_manager" type of some sort to differentate ones taht use Flow Changing API? ( loops, decisions etc )
            action_name: Some("default_action_name".to_string()),
            action_label: Some("default_action_label".to_string()),
            icon: Some("default_icon".to_string()),
            description: Some("default_description".to_string()),
            handles: Some(vec![
                Handle {
                    id: "a".to_string(),
                    position: "top".to_string(),
                    r#type: "target".to_string(),
                },
                Handle {
                    id: "b".to_string(),
                    position: "bottom".to_string(),
                    r#type: "source".to_string(),
                },
            ]),
            variables: Vec::new(),
            config: Some(serde_json::json!({})),
            config_schema: Some(serde_json::json!({})),
            plugin_id: Some("default_plugin_id".to_string()),
        }
    }

    pub fn trigger(mut self, trigger: bool) -> Self {
        self.trigger = Some(trigger);
        self
    }

    pub fn action_name(mut self, action_name: String) -> Self {
        self.action_name = Some(action_name);
        self
    }

    pub fn action_label(mut self, action_label: String) -> Self {
        self.action_label = Some(action_label);
        self
    }

    pub fn icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn handles(mut self, handles: Vec<Handle>) -> Self {
        self.handles = Some(handles);
        self
    }

    pub fn variables(mut self, variables: Vec<Value>) -> Self {
        self.variables = variables;
        self
    }

    pub fn config(mut self, config: Value) -> Self {
        self.config = Some(config);
        self
    }

    pub fn config_schema(mut self, config_schema: Value) -> Self {
        self.config_schema = Some(config_schema);
        self
    }

    pub fn plugin_id(mut self, plugin_id: String) -> Self {
        self.plugin_id = Some(plugin_id);
        self
    }

    pub fn build(self) -> Action {
        Action {
            trigger: self.trigger.unwrap_or(false),
            action_name: self
                .action_name
                .unwrap_or_else(|| "default_action_name".to_string()),
            action_label: self
                .action_label
                .unwrap_or_else(|| "default_action_label".to_string()),
            icon: self.icon.unwrap_or_else(|| "default_icon".to_string()),
            description: self
                .description
                .unwrap_or_else(|| "default_description".to_string()),
            handles: self.handles.unwrap_or_else(|| {
                vec![
                    Handle {
                        id: "a".to_string(),
                        position: "top".to_string(),
                        r#type: "target".to_string(),
                    },
                    Handle {
                        id: "b".to_string(),
                        position: "bottom".to_string(),
                        r#type: "source".to_string(),
                    },
                ]
            }),
            variables: self.variables,
            config: self.config.unwrap_or_else(|| serde_json::json!({})),
            config_schema: self.config_schema.unwrap_or_else(|| serde_json::json!({})),
            plugin_id: self
                .plugin_id
                .unwrap_or_else(|| "default_plugin_id".to_string()),
        }
    }
}

#[test]
fn builder_test() {
    let action = Action {
        trigger: false,
        action_name: "example_node".to_string(),
        action_label: "Example Node".to_string(),
        icon: "<svg></svg>".to_string(),
        description: "This is an example action".to_string(),
        handles: vec![
            Handle {
                id: "a".to_string(),
                position: "top".to_string(),
                r#type: "target".to_string(),
            },
            Handle {
                id: "b".to_string(),
                position: "bottom".to_string(),
                r#type: "source".to_string(),
            },
        ],
        variables: vec![],
        config: serde_json::json!({
            "method": "GET",
            "url": "http://example.com",
            "headers": {},
            "body": ""
        }),
        config_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"]
                },
                "url": {
                    "type": "string"
                },
                "headers": {
                    "type": "object"
                },
                "body": {
                    "type": "string"
                }
            },
            "required": ["method", "url"],
            "additionalProperties": false
        }),
        plugin_id: "example_extension".to_string(),
    };

    let action_from_builder: Action = Action::builder()
        .trigger(false)
        .action_name("example_node".to_string())
        .action_label("Example Node".to_string())
        .icon("<svg></svg>".to_string())
        .description("This is an example action".to_string())
        .variables(vec![])
        .config(serde_json::json!({
            "method": "GET",
            "url": "http://example.com",
            "headers": {},
            "body": ""
        }))
        .config_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"]
                },
                "url": {
                    "type": "string"
                },
                "headers": {
                    "type": "object"
                },
                "body": {
                    "type": "string"
                }
            },
            "required": ["method", "url"],
            "additionalProperties": false
        }))
        .plugin_id("example_extension".to_string())
        .build();

    assert_eq!(action, action_from_builder);

    let compiled = JSONSchema::compile(&action_from_builder.config_schema).expect("A valid schema");

    assert!(compiled.is_valid(&action_from_builder.config));
}

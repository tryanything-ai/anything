use extism_pdk::*;
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
    pub input: Value,        //a base input that actually works.
    pub input_schema: Value, // a json schema for the input.
    pub output_schema: Value, // A JSON schema for the response.
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
    input: Option<Value>,
    input_schema: Option<Value>,
    output_schema: Option<Value>,
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
            input: Some(serde_json::json!({})),
            input_schema: Some(serde_json::json!({})),
            output_schema: Some(serde_json::json!({})),
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

    pub fn input(mut self, input: Value) -> Self {
        self.input = Some(input);
        self
    }

    pub fn input_schema(mut self, input_schema: Value) -> Self {
        self.input_schema = Some(input_schema);
        self
    }

    pub fn output_schema(mut self, output_schema: Value) -> Self {
        self.output_schema = Some(output_schema);
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
            input: self.input.unwrap_or_else(|| serde_json::json!({})),
            input_schema: self.input_schema.unwrap_or_else(|| serde_json::json!({})),
            output_schema: self.output_schema.unwrap_or_else(|| serde_json::json!({})),
            plugin_id: self
                .plugin_id
                .unwrap_or_else(|| "default_plugin_id".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonschema::{Draft, JSONSchema};

    #[test]
    fn builder_test() {
        use jsonschema::{is_valid, JSONSchema};
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
            input: serde_json::json!({
                "method": "GET",
                "url": "http://example.com",
                "headers": {},
                "body": ""
            }),
            input_schema: serde_json::json!({
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
            output_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "string",
                        "enum": ["success", "error"]
                    },
                    "response": {
                        "type": "object"
                    },
                    "error": {
                        "type": "object"
                    }
                },
                "required": ["status"]
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
            .input(serde_json::json!({
                "method": "GET",
                "url": "http://example.com",
                "headers": {},
                "body": ""
            }))
            .input_schema(serde_json::json!({
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
            .output_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "string",
                        "enum": ["success", "error"]
                    },
                    "response": {
                        "type": "object"
                    },
                    "error": {
                        "type": "object"
                    }
                },
                "required": ["status"]
            }))
            .plugin_id("example_extension".to_string())
            .build();

        assert_eq!(action, action_from_builder);

        let compiled =
            JSONSchema::compile(&action_from_builder.input_schema).expect("A valid schema");

        assert!(compiled.is_valid(&action_from_builder.input));

        let compiled_output =
            JSONSchema::compile(&action_from_builder.output_schema).expect("A valid schema");

        assert!(compiled_output.is_valid(&serde_json::json!({
            "status": "success"
        })));
    }
}

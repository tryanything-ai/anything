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
    pub node_name: String,
    pub node_label: String,
    pub icon: String,
    pub description: String,
    pub handles: Vec<Handle>,
    pub variables: Vec<Value>,
    pub config: Value,
    pub extension_id: String,
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
    node_name: Option<String>,
    node_label: Option<String>,
    icon: Option<String>,
    description: Option<String>,
    handles: Option<Vec<Handle>>,
    variables: Vec<Value>,
    config: Option<Value>,
    extension_id: Option<String>,
}

impl ActionBuilder {
    pub fn new() -> ActionBuilder {
        // Set the minimally required fields of Action.
        ActionBuilder {
            trigger: Some(false),
            node_name: Some("default_node_name".to_string()),
            node_label: Some("default_node_label".to_string()),
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
            extension_id: Some("default_extension_id".to_string()),
        }
    }

    pub fn trigger(mut self, trigger: bool) -> Self {
        self.trigger = Some(trigger);
        self
    }

    pub fn node_name(mut self, node_name: String) -> Self {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_label(mut self, node_label: String) -> Self {
        self.node_label = Some(node_label);
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

    pub fn extension_id(mut self, extension_id: String) -> Self {
        self.extension_id = Some(extension_id);
        self
    }

    pub fn build(self) -> Action {
        Action {
            trigger: self.trigger.unwrap_or(false),
            node_name: self.node_name.unwrap_or_else(|| "default_node_name".to_string()),
            node_label: self.node_label.unwrap_or_else(|| "default_node_label".to_string()),
            icon: self.icon.unwrap_or_else(|| "default_icon".to_string()),
            description: self.description.unwrap_or_else(|| "default_description".to_string()),
            handles: self.handles.unwrap_or_else(|| vec![
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
            variables: self.variables,
            config: self.config.unwrap_or_else(|| serde_json::json!({})),
            extension_id: self.extension_id.unwrap_or_else(|| "default_extension_id".to_string()),
        }
    }
}

#[test]
fn builder_test() {
    let action = Action {
        trigger: false,
        node_name: "example_node".to_string(),
        node_label: "Example Node".to_string(),
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
        extension_id: "example_extension".to_string(),
    };

    let action_from_builder: Action = Action::builder()
        .trigger(false)
        .node_name("example_node".to_string())
        .node_label("Example Node".to_string())
        .icon("<svg></svg>".to_string())
        .description("This is an example action".to_string())
        .variables(vec![])
        .config(serde_json::json!({
            "method": "GET",
            "url": "http://example.com",
            "headers": {},
            "body": ""
        }))
        .extension_id("example_extension".to_string())
        .build();

    assert_eq!(action, action_from_builder);
}

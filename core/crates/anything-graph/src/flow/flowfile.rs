//! A flowfile description build into a flow
//!
//!

use serde::Deserialize;
use serde_json::Value;
use std::path::PathBuf;
use toml::Table;

use super::{
    action::{Action, ActionBuilder, ActionType, ShellAction},
    flow::Flow,
    node::Node,
    trigger::Trigger,
};
use crate::error::{AppError, AppResult};

#[derive(Clone, Debug, Default)]
pub struct Flowfile {
    pub flow: Flow,
}

impl Flowfile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(file: PathBuf) -> AppResult<Self> {
        if !file.exists() {
            return Err(AppError::FileNotFound(file.to_string_lossy().to_string()));
        }
        let toml_contents = std::fs::read_to_string(file)?;
        let parsed_flow = toml::from_str::<ParseFlowfile>(&toml_contents)?;
        let flow: Flow = parsed_flow.into();
        let flowfile = Flowfile { flow };
        // let flowfile = Flowfile::default();
        Ok(flowfile)
    }
}

// ----------------------------------------------------
// Parsing structs
// ----------------------------------------------------

#[derive(Deserialize, Debug, Clone)]
struct ParseFlowfile {
    name: String,
    version: Option<String>,
    trigger: ParseTrigger,
    nodes: Vec<ParseNode>,
}

impl Into<Flow> for ParseFlowfile {
    fn into(self) -> Flow {
        let mut flow = Flow::new();
        flow.name = self.name;
        flow.version = self.version;
        flow.trigger = self.trigger.into();
        for node in self.nodes {
            let node: Node = node.into();
            flow.add_node_obj(&node).expect("unable to add node");
        }
        flow
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ParseTrigger {
    name: String,
    settings: Value,
}

impl Into<Trigger> for ParseTrigger {
    fn into(self) -> Trigger {
        match self.name.as_str() {
            "manual" => Trigger::Manual(super::trigger::ManualTrigger {
                name: self.name,
                settings: self.settings,
            }),
            "file_change" => Trigger::FileChange(super::trigger::FileChangeTrigger {
                settings: self.settings,
            }),
            "webhook" => Trigger::Webhook(super::trigger::WebhookTrigger {
                settings: self.settings,
            }),
            "schedule" => Trigger::Schedule(super::trigger::ScheduleTrigger {
                settings: self.settings,
            }),
            _ => Trigger::Empty(super::trigger::EmptyTrigger {
                settings: self.settings,
            }),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ParseNode {
    name: String,
    label: String,
    depends_on: Option<Vec<String>>,
    action: ParseAction,
}

impl Into<Node> for ParseNode {
    fn into(self) -> Node {
        let mut node = Node::new();
        node.name = self.name;
        node.label = self.label;
        node.depends_on = self.depends_on.unwrap_or_default();
        node.node_action = self.action.into();
        node
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ParseAction {
    pub action_type: String,
    config: Value,
}

impl Into<Action> for ParseAction {
    fn into(self) -> Action {
        let action_type = match self.action_type.as_str() {
            "shell" => {
                let config: ShellAction =
                    serde_json::from_str(self.config.to_string().as_str()).unwrap();
                ActionType::Shell(config)
            }
            _ => ActionType::Empty,
        };
        ActionBuilder::default()
            .action_type(action_type)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_parse_flowfile_into_flow() -> Result<()> {
        let simple_file = PathBuf::from("tests/resources/simple.toml");

        let _flow_file = Flowfile::from_file(simple_file)?;
        // println!("flow file: {:#?}", flow_file);
        Ok(())
    }
}

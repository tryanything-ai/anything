//! A flowfile description build into a flow
//!
//!
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};

use super::{
    action::{Action, ActionBuilder, ActionType, EmptyAction, RestAction, ShellAction},
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
        Self::from_string(toml_contents)
    }

    pub fn from_string(toml_contents: String) -> AppResult<Self> {
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
    id: Option<String>,
    name: String,
    version: Option<String>,
    trigger: ParseTrigger,
    nodes: Vec<ParseNode>,
}

impl Into<Flow> for ParseFlowfile {
    fn into(self) -> Flow {
        let mut flow = Flow::new();
        flow.name = self.name;
        flow.id = self.id.unwrap_or(uuid::Uuid::new_v4().to_string());
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
    variables: Option<Vec<HashMap<String, String>>>,
}

impl Into<Node> for ParseNode {
    fn into(self) -> Node {
        let mut node = Node::default();
        node.name = self.name;
        node.label = self.label.clone();
        node.depends_on = self.depends_on.unwrap_or_default();
        node.node_action = self.action.into();
        node.variables = optional_vec_map_into_hashmap(self.variables).unwrap_or_default();
        node
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ParseAction {
    pub action_type: String,
    config: Value,
}

#[derive(Deserialize, Debug, Clone)]
struct ParseShellConfig {
    pub command: String,
    pub executor: Option<String>,
    pub args: Option<Vec<HashMap<String, String>>>,
    pub cwd: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct ParseRestConfig {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<Vec<HashMap<String, String>>>,
    pub body: Option<String>,
    pub response_type: Option<String>,
    pub query_params: Option<Vec<HashMap<String, String>>>,
}

impl Into<Action> for ParseAction {
    fn into(self) -> Action {
        let action_type = match self.action_type.to_lowercase().as_str() {
            "shell" => {
                let config: ParseShellConfig =
                    serde_json::from_str(self.config.to_string().as_str()).unwrap();

                let args_map = optional_vec_map_into_hashmap(config.args);

                ActionType::Shell(ShellAction {
                    command: config.command,
                    executor: config.executor,
                    args: args_map,
                    cwd: config.cwd,
                })
            }
            "rest" => {
                let config: ParseRestConfig =
                    serde_json::from_str(self.config.to_string().as_str()).unwrap();
                let headers = optional_vec_map_into_hashmap(config.headers);
                let query_params = optional_vec_map_into_hashmap(config.query_params);
                ActionType::Rest(RestAction {
                    url: config.url,
                    method: config.method,
                    headers: headers,
                    body: config.body,
                    response_type: config.response_type,
                    query_params: query_params,
                })
            }
            _ => ActionType::Empty(EmptyAction {}),
        };
        ActionBuilder::default()
            .action_type(action_type)
            .build()
            .unwrap()
    }
}

fn optional_vec_map_into_hashmap(
    arg: Option<Vec<HashMap<String, String>>>,
) -> Option<HashMap<String, String>> {
    match arg {
        Some(args) => {
            let mut args_map = HashMap::new();
            for arg in args {
                for (k, v) in arg {
                    args_map.insert(k, v);
                }
            }
            Some(args_map)
        }
        None => None,
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

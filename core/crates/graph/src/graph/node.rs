use std::fmt::Display;
use std::hash::Hash;

use anything_common::tracing;
use anything_runtime::prelude::*;
use anything_runtime::string_or_struct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::core::id_alloc::alloc_id;

pub type NodeId = usize;
pub type GroupId = usize;

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub struct NodeGroup {
    #[serde(default = "alloc_id")]
    pub id: GroupId,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub node: Task,

    #[serde(default)]
    pub depends: Vec<Task>,
}

impl Eq for NodeGroup {}

impl Hash for NodeGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let group_node_ids: Vec<NodeId> = self.depends.iter().map(|node| node.id).collect();
        group_node_ids.hash(state);
    }
}

impl NodeGroup {}

fn default_version() -> String {
    "0.0.1".to_string()
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
pub struct RunOptions {
    #[serde(
        default,
        flatten,
        alias = "runtime",
        deserialize_with = "string_or_struct"
    )]
    engine: RawNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NodeType {
    GroupStart(NodeGroup),
    Task(Task),
    GroupEnd(NodeGroup),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::GroupStart(start) => write!(f, "group start {}", start.id),
            NodeType::Task(task) => write!(f, "node: {}", task.id),
            NodeType::GroupEnd(end) => write!(f, "group end {}", end.id),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder, Hash)]
#[builder(setter(into, strip_option), default)]
pub struct Task {
    #[serde(skip, default = "alloc_id")]
    /// The unique identifier of the node
    pub id: NodeId,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub group: Option<String>,

    #[serde(default = "default_version")]
    pub version: String,

    pub description: Option<String>,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default, alias = "dependencies")]
    pub depends_on: Vec<String>,

    #[serde(default, flatten, deserialize_with = "string_or_struct")]
    pub run_options: RawNode,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: alloc_id(),
            group: None,
            label: Some("default".to_string()),
            version: default_version(),
            name: "bob-david".to_string(),
            description: None,
            depends_on: vec![],
            run_options: RawNode::default(),
        }
    }
}

impl Into<ExecuteConfig> for Task {
    fn into(self) -> ExecuteConfig {
        let execute_config: ExecuteConfig = match self
            .run_options
            .engine
            .unwrap_or(EngineKind::default())
            .try_into()
        {
            Ok(config) => config,
            Err(e) => {
                tracing::error!("error converting node to execute config: {}", e);
                ExecuteConfig::default()
            }
        };

        execute_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_node_from_raw_str() {
        let raw_toml = r#"
        id = 1
        name = "echo-cheer"
        label = "echo-cheer"
        description = "Send holiday cheer"
        depends_on = ["echo-hello"]

        [runtime]
        interpreter = "deno"
        args = ["bob"]
        "#;
        let node: Task = toml::from_str(raw_toml).unwrap();
        assert_eq!(
            node.run_options.engine.unwrap(),
            EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: Some(vec!["bob".to_string()]),
                options: indexmap::IndexMap::new(),
            })
        );
    }

    #[test]
    fn test_load_node_with_options() {
        let raw_toml = r#"
        id = 1
        name = "echo-cheer"
        label = "echo-cheer"
        description = "Send holiday cheer"
        depends_on = ["echo-hello"]

        [engine]
        engine = "deno"
        options = { bob = "barky" }
        "#;
        let node: Task = toml::from_str(raw_toml).unwrap();
        assert_eq!(
            node.run_options.engine.unwrap(),
            EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: None,
                options: indexmap::indexmap! {
                    "bob".to_string() => EngineOption::from("barky".to_string())
                },
            })
        );
    }

    #[test]
    fn test_load_node_with_args_and_options() {
        let raw_toml = r#"
        id = 1
        name = "echo-cheer"
        label = "echo-cheer"
        description = "Send holiday cheer"
        depends_on = ["echo-hello"]

        [engine]
        engine = "deno"
        args = ["index.js"]
        options = { bob = "barky" }
        "#;
        let node: Task = toml::from_str(raw_toml).unwrap();
        assert_eq!(
            node.run_options.engine.unwrap(),
            EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: Some(vec!["index.js".to_string()]),
                options: indexmap::indexmap! {
                    "bob".to_string() => EngineOption::from("barky".to_string())
                },
            })
        );
    }

    #[test]
    fn test_loads_correctly_for_bash_task() {
        let raw_toml = format!(
            r#"
        name = "echo"
    
        [engine]
        engine = "system-shell"
        args = ["echo", "hello world"]
    
        "#,
        );
        let node: Task = toml::from_str(&raw_toml).unwrap();
        assert_eq!(
            node.run_options.engine.unwrap(),
            EngineKind::Internal(SystemShell {
                interpreter: "sh".to_string(),
                args: vec!["echo".to_string(), "hello world".to_string()]
            })
        );
    }
}

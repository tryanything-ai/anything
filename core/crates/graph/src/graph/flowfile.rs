use anything_runtime::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

use crate::error::{GraphError, GraphResult};

use super::{flow::Flow, node::Task, trigger::Trigger};

fn default_version() -> Option<String> {
    Some("0.1".to_string())
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Flowfile {
    #[serde(default)]
    pub flow_id: String,

    #[serde(default)]
    flow: Flow,

    #[serde(default)]
    pub name: String,

    #[serde(default = "default_version")]
    pub version: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub variables: RawVariables,

    #[serde(default)]
    pub environment: RawEnvironment<String>,

    #[serde(default)]
    pub trigger: Trigger,

    #[serde(default)]
    pub nodes: Vec<Task>,
}

#[allow(unused)]
impl Flowfile {
    pub fn from_file(path: PathBuf) -> GraphResult<Self> {
        if !path.exists() {
            return Err(crate::error::GraphError::FileDoesNotExist(
                path.to_string_lossy().to_string(),
            ));
        }

        let parsed_contents = std::fs::read_to_string(path)?;

        Self::from_string(parsed_contents)
    }

    pub fn from_string(parsed_contents: String) -> GraphResult<Self> {
        Ok(Self::from_str(&parsed_contents)?)
    }
}

impl FromStr for Flowfile {
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str::<Flowfile>(s).map_err(|e| GraphError::FlowFileParsingError(e))
    }
}

impl TryFrom<String> for Flowfile {
    type Error = crate::error::GraphError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl Into<String> for Flowfile {
    fn into(self) -> String {
        let mut flow = self.flow;
        flow.flow_id = self.flow_id;
        flow.name = self.name;
        flow.version = self.version.unwrap_or(default_version().unwrap());
        flow.description = self.description.unwrap_or("".to_string());
        flow.variables = self.variables;
        flow.environment = self.environment;
        flow.trigger = self.trigger;
        flow.nodes = self.nodes;

        toml::to_string(&flow).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::trigger::EmptyTrigger;

    use super::*;

    #[test]
    fn test_parsing_node() {}

    // Node parsing
    #[test]
    fn test_node_from_toml_slim() {
        let toml = r#"
        name = "SimpleFlow"
        version = "0.1"
        description = "A simple flow that echos holiday cheer"
        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(flow.version, Some("0.1".to_string()));
        assert_eq!(
            flow.description,
            Some("A simple flow that echos holiday cheer".to_string())
        );
    }

    #[test]
    fn test_node_from_toml_with_variables() {
        let toml = r#"
        name = "SimpleFlow"
        version = "0.1"
        description = "A simple flow that echos holiday cheer"

        [variables]
        name = "bob"
        age = 42
        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(
            flow.variables.get("name").unwrap().to_owned(),
            ValueKind::String("bob".to_string()),
        );
        assert_eq!(
            flow.variables.get("age").unwrap().to_owned(),
            ValueKind::Int(42),
        );
    }

    #[test]
    fn test_node_from_toml_with_an_environment() {
        let toml = r#"
        name = "SimpleFlow"
        version = "0.1"
        description = "A simple flow that echos holiday cheer"

        [environment]
        PATH="/usr/bin/sbin"
        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(
            flow.environment.get("PATH"),
            Some(&"/usr/bin/sbin".to_string()),
        );
    }

    #[test]
    fn test_node_from_toml_with_a_trigger() {
        let toml = r#"
        name = "SimpleFlow"
        version = "0.1"
        description = "A simple flow that echos holiday cheer"

        [trigger]
        type = "empty"
        settings = {name = "ok"}
        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(
            flow.trigger,
            Trigger::Empty(EmptyTrigger {
                settings: Some(indexmap::indexmap! {
                    "name".to_string() => ValueKind::String("ok".to_string()),
                })
            })
        );
    }

    #[test]
    fn test_node_from_toml_with_a_single_node() {
        let toml = r#"
        name = "SimpleFlow"
        [[nodes]]
        name = "echo"

        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(flow.nodes.len(), 1);
        assert_eq!(flow.nodes[0].name, "echo".to_string());
    }

    #[test]
    fn test_node_from_toml_with_multiple_nodes() {
        let toml = r#"
        name = "SimpleFlow"
        [[nodes]]
        name = "echo"

        [[nodes]]
        name = "echo2"
        depends_on = ["echo"]

        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(flow.nodes.len(), 2);
        assert_eq!(flow.nodes[0].name, "echo".to_string());
        assert_eq!(flow.nodes[1].name, "echo2".to_string());
    }

    #[test]
    fn test_node_from_toml_with_multiple_nodes_with_actions() {
        let toml = r#"
        name = "SimpleFlow"

        [[nodes]]
        name = "echo"

        [nodes.engine]
        engine = "bash"
        args = ["echo", "hello world"]

        [[nodes]]
        name = "echo2"
        depends_on = ["echo"]

        [nodes.engine]
        engine = "deno"
        options = {bob = "barky"}

        "#;
        let flow = Flowfile::from_str(toml).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(flow.nodes.len(), 2);
        assert_eq!(flow.nodes[0].name, "echo".to_string());
        assert_eq!(flow.nodes[1].name, "echo2".to_string());

        assert_eq!(
            flow.nodes[0].run_options.engine,
            Some(EngineKind::Internal(SystemShell {
                interpreter: "bash".to_string(),
                args: vec!["echo".to_string(), "hello world".to_string()],
            }))
        );

        assert_eq!(
            flow.nodes[1].run_options.engine,
            Some(EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: None,
                options: indexmap::indexmap! {
                    "bob".to_string() => EngineOption::from("barky".to_string())
                },
            }))
        );
    }

    #[test]
    fn test_loads_flow_from_file() {
        let fixture_directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("resources");
        let simple_flow = fixture_directory.join("simple.toml");

        let flow = Flowfile::from_file(simple_flow).unwrap();
        assert_eq!(flow.name, "SimpleFlow".to_string());
        assert_eq!(flow.nodes.len(), 3);
    }
}

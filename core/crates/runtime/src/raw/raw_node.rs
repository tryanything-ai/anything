use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    utils::de::option_string_or_struct, EngineKind, RawEnvironment, RawVariables, RuntimeError,
};

#[derive(Debug, Clone, Default)]
pub enum Kind {
    #[default]
    None,
    Args,
    Global,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct RawNode {
    #[serde(default)]
    pub variables: RawVariables,

    #[serde(default)]
    pub environment: RawEnvironment<String>,

    #[serde(
        default,
        alias = "runtime",
        deserialize_with = "option_string_or_struct"
    )]
    #[serde(alias = "interpreter")]
    pub engine: Option<EngineKind>,
}

impl FromStr for RawNode {
    type Err = RuntimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = toml::from_str::<RawNode>(s).map_err(|_e| RuntimeError::ParseError);
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::{PluginEngine, SystemShell, ValueKind};

    use super::*;

    #[test]
    fn test_load_node_from_raw_str_with_simple_values() {
        let raw_toml = r#"

        [variables]
        name ="bob"
        age = 12
        favorite_flavors = ["chocolate", "strawberry"]
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.variables.get("name").unwrap().to_owned(),
            ValueKind::String("bob".to_string())
        );
        assert_eq!(
            node.variables.get("age").unwrap().to_owned(),
            ValueKind::Int(12)
        );
        assert_eq!(
            node.variables.get("favorite_flavors").unwrap().to_owned(),
            ValueKind::List(vec![
                ValueKind::String("chocolate".to_string()),
                ValueKind::String("strawberry".to_string())
            ])
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_environment() {
        let raw_toml = r#"

        [environment]
        PATH="/usr/local/bin"
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.environment.get("PATH").unwrap().to_owned(),
            "/usr/local/bin".to_string()
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_default_shell_interpreter() {
        let raw_toml = r#"
        id = 1
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(node.engine, None);
    }

    #[test]
    fn test_load_node_from_raw_str_with_for_internal_engine() {
        let raw_toml = r#"
        engine = "system-shell"
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::Internal(SystemShell {
                interpreter: "sh".to_string(),
                args: vec!["-c".to_string()]
            }))
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_explicit_shell_interpreter() {
        let raw_toml = r#"
        engine = "bash"
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::PluginEngine(PluginEngine {
                engine: "system-shell".to_string(),
                args: Some(vec![]),
                options: indexmap::indexmap! {
                    "shell".to_string() => crate::EngineOption::String("bash".to_string())
                }
            }))
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_explicit_shell_interpreter_as_option() {
        let raw_toml = r#"
        id = 1
        name = "echo-cheer"

        [engine]
        interpreter = "bash"
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::PluginEngine(PluginEngine {
                engine: "system-shell".to_string(),
                args: None,
                options: indexmap::indexmap! {
                    "shell".to_string() => crate::EngineOption::String("bash".to_string())
                }
            }))
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_plugin_system_shell_as_runtime() {
        let raw_toml = r#"
        id = 1
        name = "echo-cheer"

        [engine]
        runtime = "system-shell"
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::Internal(SystemShell {
                interpreter: "sh".to_string(),
                args: vec!["-c".to_string()]
            }))
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_explicit_plugin_interpreter_as_option() {
        let raw_toml = r#"
        id = 1
        name = "echo-cheer"

        [engine]
        runtime = "deno"
        options = {script = "echo-hello.ts"}
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: None,
                options: indexmap::indexmap! {
                    "script".to_string() => crate::EngineOption::String("echo-hello.ts".to_string())
                }
            }))
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_explicit_plugin_interpreter_as_string() {
        let raw_toml = r#"
        runtime = "deno"
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: Some(vec![]),
                options: indexmap::indexmap! {}
            }))
        );
    }

    #[test]
    fn test_load_node_from_raw_str_with_plugin_interpreter_with_args() {
        let raw_toml = r#"
        [runtime]
        interpreter = "deno"
        args = ["bob"]
        "#;

        let node = RawNode::from_str(raw_toml);
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(
            node.engine,
            Some(EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: Some(vec!["bob".to_string()]),
                options: indexmap::indexmap! {}
            }))
        );
    }
}

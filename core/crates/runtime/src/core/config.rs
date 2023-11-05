use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    plugins::options::PluginOption, EngineKind, RawEnvironment, RawVariables, RuntimeError,
};

/// The execute configuration is used to configure the execution of a single
/// node in the flow
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option), default)]
pub struct ExecuteConfig {
    pub plugin_name: String,
    pub runtime: String,
    pub args: Vec<String>,
    pub options: indexmap::IndexMap<String, PluginOption>,
}

impl Default for ExecuteConfig {
    fn default() -> Self {
        Self {
            plugin_name: "system-shell".to_string(),
            runtime: "bash".to_string(),
            args: vec!["-c".to_string()],
            options: indexmap::IndexMap::new(),
        }
    }
}

impl TryFrom<EngineKind> for ExecuteConfig {
    type Error = RuntimeError;

    fn try_from(value: EngineKind) -> Result<Self, Self::Error> {
        match value {
            EngineKind::Internal(internal_shell) => Ok(Self {
                plugin_name: "system-shell".to_string(),
                runtime: internal_shell.to_string(),
                args: vec!["-c".to_string()],
                options: indexmap::IndexMap::new(),
            }),
            EngineKind::PluginEngine(plugin_interpreter) => {
                let mut cfg = ExecuteConfig::default();
                cfg.args = plugin_interpreter.args.unwrap_or(vec![]);
                cfg.options = plugin_interpreter.options.iter().fold(
                    indexmap::indexmap! {},
                    |mut acc, (k, v)| {
                        acc.insert(k.into(), v.to_owned().into());
                        acc
                    },
                );
                Ok(cfg)
            }
        }
    }
}

/// The runtime configuration is used to configure the runtime environment
/// for the execution of an entire flow
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RuntimeConfig {
    pub base_dir: Option<PathBuf>,
    pub current_dir: Option<PathBuf>,
    pub plugins: indexmap::IndexMap<String, PathBuf>,

    #[serde(default)]
    pub variables: RawVariables,

    #[serde(default)]
    pub environment: RawEnvironment<String>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        let base_dir = tempfile::tempdir().unwrap();
        Self {
            base_dir: Some(base_dir.path().to_path_buf()),
            current_dir: None,
            plugins: indexmap::IndexMap::new(),
            variables: RawVariables::default(),
            environment: RawEnvironment::default(),
        }
    }
}

impl RuntimeConfig {
    pub fn base_dir(&self) -> &PathBuf {
        self.base_dir.as_ref().unwrap()
    }

    pub fn database_dir(&self) -> PathBuf {
        self.base_dir().join("database")
    }

    pub fn flow_dir(&self) -> PathBuf {
        self.base_dir().join("flow")
    }
}

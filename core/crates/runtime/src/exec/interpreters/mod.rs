pub mod plugin;
pub mod shebang;
pub mod system;

use std::{cell::RefCell, fmt::Display, rc::Rc, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    constants::POSSIBLE_SHELL_NAMES,
    core::config::ExecuteConfig,
    errors::{RuntimeError, RuntimeResult},
    plugins::manager::PluginManager,
};

use self::{plugin::PluginEngine, system::SystemShell};

use super::scope::{ExecutionResult, Scope};

// Deserialize
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum EngineKind {
    PluginEngine(PluginEngine),
    Internal(SystemShell),
}

impl Default for EngineKind {
    fn default() -> Self {
        Self::Internal(SystemShell::default())
    }
}

impl From<String> for EngineKind {
    fn from(v: String) -> Self {
        let mut parts = v.split(|c| c == ' ' || c == '\t');
        let first_part = parts.next().unwrap_or_default();
        if POSSIBLE_SHELL_NAMES.contains(&first_part) {
            // We have an Internal interpreter
            if let Some(shell) = SystemShell::get_from_string(&v) {
                EngineKind::Internal(shell)
            } else {
                EngineKind::default()
            }
        } else if PluginManager::find_plugin_library(first_part).is_ok() {
            // We have a Plugin interpreter
            if let Some(interpreter) = PluginEngine::get_from_string(&v) {
                EngineKind::PluginEngine(interpreter)
            } else {
                EngineKind::default()
            }
        } else {
            EngineKind::default()
        }
    }
}

impl EngineKind {
    pub fn add_arg(&mut self, arg: &str) {
        match self {
            Self::Internal(shell) => shell.args.push(arg.to_string()),
            Self::PluginEngine(interpreter) => {
                if let Some(args) = &mut interpreter.args {
                    args.push(arg.to_string());
                } else {
                    interpreter.args = Some(vec![arg.to_string()]);
                }
            }
        }
    }

    pub fn add_option(&mut self, key: &str, value: toml::Value) {
        match self {
            Self::PluginEngine(interpreter) => {
                interpreter.options.insert(key.to_string(), value.into());
            }
            _ => {}
        }
    }

    pub fn execute(
        &self,
        code: &str,
        scope: &Scope,
        config: &ExecuteConfig,
        registry: Rc<RefCell<PluginManager>>,
    ) -> RuntimeResult<ExecutionResult> {
        match self {
            EngineKind::Internal(internal_shell) => internal_shell.execute(code, scope, config),
            EngineKind::PluginEngine(interpreter) => {
                let manager = (*registry)
                    .try_borrow_mut()
                    .map_err(|_| RuntimeError::PluginManagerError)?;

                let plugin = manager.get_plugin(&interpreter.engine)?;
                let execution_config: ExecuteConfig = interpreter
                    .try_into()
                    .map_err(|_| RuntimeError::PluginOptionError)?;

                match plugin.execute(&scope, &execution_config) {
                    Err(e) => Err(RuntimeError::PluginError(*e)),
                    Ok(v) => Ok(v),
                }

                // Ok(plugin.execute(&scope, &execution_config))
            }
        }
    }
}

impl Display for EngineKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineKind::Internal(internal_shell) => write!(f, "{}", internal_shell),
            EngineKind::PluginEngine(plugin_interpreter) => {
                write!(f, "{}", plugin_interpreter)
            }
        }
    }
}

impl TryFrom<&PluginEngine> for ExecuteConfig {
    type Error = RuntimeError;

    fn try_from(value: &PluginEngine) -> Result<Self, Self::Error> {
        Ok(Self {
            plugin_name: "system-shell".to_string(),
            runtime: value.engine.clone(),
            args: value.args.clone().unwrap_or_default(),
            options: value
                .options
                .clone()
                .into_iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        })
    }
}

impl TryInto<ExecuteConfig> for PluginEngine {
    type Error = RuntimeError;

    fn try_into(self) -> Result<ExecuteConfig, Self::Error> {
        let mut cfg = ExecuteConfig::default();
        cfg.args = self.args.unwrap_or(vec![]);
        cfg.options = self
            .options
            .iter()
            .fold(indexmap::indexmap! {}, |mut acc, (k, v)| {
                acc.insert(k.into(), v.to_owned().into());
                acc
            });
        Ok(cfg)
    }
}

impl FromStr for EngineKind {
    type Err = RuntimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if POSSIBLE_SHELL_NAMES.contains(&s.to_lowercase().as_str()) {
            if let Some(shell) = SystemShell::get_from_string(s) {
                Ok(Self::Internal(shell))
            } else {
                Err(RuntimeError::ParseError)
            }
        } else {
            if let Some(interpreter) = PluginEngine::get_from_string(s) {
                Ok(Self::PluginEngine(interpreter))
            } else {
                // Err(RuntimeError::ParseError)
                Ok(Self::default())
            }
        }
    }
}

pub mod plugin;
pub mod shebang;
pub mod system;

use std::{cell::RefCell, fmt::Display, rc::Rc, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    core::config::ExecuteConfig,
    errors::{RuntimeError, RuntimeResult},
    plugins::manager::PluginManager,
    EngineOption, POSSIBLE_SHELL_NAMES,
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
        Self::from_str(&v).unwrap_or_default()
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
            context: Default::default()
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
        if &s.to_lowercase() == "system-shell" {
            Ok(Self::default())
        } else {
            if let Some(interpreter) = PluginEngine::get_from_string(s) {
                match interpreter.engine.as_str() {
                    "system-shell" => {
                        let mut cfg: ExecuteConfig = interpreter.try_into()?;
                        cfg.plugin_name = "system-shell".to_string();
                        Ok(Self::Internal(SystemShell::try_from(cfg)?))
                    }
                    possible_shell => {
                        if POSSIBLE_SHELL_NAMES.contains(&possible_shell) {
                            // If the interpreter is a shell, we need to set the runtime to system-shell
                            // This is a little hacky, but it allows the user to set the specific shell
                            // to execute in the system plugin
                            let mut system_plugin = PluginEngine::get_from_string("system-shell")
                                .ok_or(RuntimeError::InvalidInterpreter)?;
                            let value = EngineOption::String(possible_shell.to_string());
                            system_plugin.options.insert("shell".to_string(), value);
                            Ok(Self::PluginEngine(system_plugin))
                        } else if let Some(plugin) = PluginEngine::get_from_string(possible_shell) {
                            Ok(Self::PluginEngine(plugin))
                        } else {
                            Err(RuntimeError::InvalidInterpreter)
                        }
                    }
                }
            } else {
                // Err(RuntimeError::ParseError)
                Ok(Self::default())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deno_plugin_is_created_when_transforming_from_str() {
        let engine = EngineKind::from_str("deno").unwrap();
        assert_eq!(
            engine,
            EngineKind::PluginEngine(PluginEngine {
                engine: "deno".to_string(),
                args: Some(vec![]),
                options: indexmap::indexmap! {}
            })
        );
    }

    #[test]
    fn test_system_plugin_is_created_when_transforming_from_str_with_explicit_shell() {
        let engine = EngineKind::from_str("bash").unwrap();
        assert_eq!(
            engine,
            EngineKind::PluginEngine(PluginEngine {
                engine: "system-shell".to_string(),
                args: Some(vec![]),
                options: indexmap::indexmap! {
                    "shell".to_string() => EngineOption::String("bash".to_string())
                }
            })
        );
    }

    #[test]
    fn test_system_shell_is_created_when_explicitly_system_shell() {
        let engine = EngineKind::from_str("system-shell").unwrap();
        assert_eq!(
            engine,
            EngineKind::Internal(SystemShell {
                interpreter: "sh".to_string(),
                args: vec!["-c".to_string()]
            })
        );
    }
}

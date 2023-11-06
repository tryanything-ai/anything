use std::{ffi::OsStr, sync::Arc};

use tokio::sync::Mutex;
use tracing::debug;

use super::scope::Scope;
use crate::{
    core::config::RuntimeConfig,
    errors::{RuntimeError, RuntimeResult},
    exec::scope::child_scope,
    plugins::{built_in::BUILT_IN_PLUGINS, manager::PluginManager},
    ExecuteConfig, ExecutionResult,
};

#[derive(Debug, Clone)]
/// The `Runner` struct represents a runtime environment for executing code, with a global scope, plugin
/// manager, and configuration.
///
/// Properties:
///
/// * `global_scope`: The `global_scope` property is a shared reference to a `Scope` struct wrapped in
/// an `Arc` (atomic reference counting) and `Mutex` (mutual exclusion) to allow for concurrent access
/// and modification of the scope. A `Scope` is typically used to store variables and their values
/// * `plugin_manager`: The `plugin_manager` property is of type `Arc<Mutex<PluginManager>>`.
/// * `config`: The `config` property is of type `RuntimeConfig`. It stores the runtime configuration
/// settings for the `Runner` struct.
pub struct Runner {
    pub global_scope: Arc<Mutex<Scope>>,
    pub plugin_manager: Arc<Mutex<PluginManager>>,
    pub config: RuntimeConfig,
}

impl Runner {
    pub fn new(config: RuntimeConfig) -> Self {
        let global_scope = Arc::new(Mutex::new(Scope {
            name: "anything-runtime".to_string(),
            ..Default::default()
        }));
        let plugin_manager = Arc::new(Mutex::new(PluginManager::new()));

        Self {
            global_scope,
            plugin_manager,
            config,
        }
    }

    /// The function adds a global variable to the global scope.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the name of the global variable to be added.
    /// * `value`: The `value` parameter is a string that represents the value of the global variable.
    pub fn add_global_variable(&mut self, name: &str, value: &str) {
        let mut global_scope = self
            .global_scope
            .try_lock()
            .expect("Failed to lock global scope");
        // global_scope.insert_variable(name.into(), value);
        global_scope.insert_binding(name.into(), value, None).ok();
    }

    /// The function adds a global environment variable to the current scope.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the name of the environment variable to be added to the global
    /// scope.
    /// * `value`: The `value` parameter is an optional `String` that represents the value of the
    /// environment variable. If `value` is `Some`, it means the environment variable has a value. If
    /// `value` is `None`, it means the environment variable does not have a value.
    pub fn add_global_environment(&mut self, name: &str, value: Option<String>) {
        let mut global_scope = self
            .global_scope
            .try_lock()
            .expect("Failed to lock global scope");
        global_scope.insert_environment_variable(name.into(), value);
    }

    /// The function executes a plugin by name, creates a child scope, executes the plugin with the
    /// scope and execution configuration, inserts the result into the scope, and returns the updated
    /// global scope.
    ///
    /// Arguments:
    ///
    /// * `stage_name`: A string representing the name of the stage being executed.
    /// * `execution_config`: The `execution_config` parameter is of type `ExecuteConfig`. It contains
    /// configuration information for the execution, such as the plugin name to use for execution.
    ///
    /// Returns:
    ///
    /// a `RuntimeResult` which contains an `Arc<Mutex<Scope>>`.
    pub fn execute(
        &mut self,
        stage_name: String,
        execution_config: ExecuteConfig,
    ) -> RuntimeResult<Arc<Mutex<Scope>>> {
        // Load the plugin we're using to execute by name
        let plugin_name = execution_config.plugin_name.clone();
        let pm = self
            .plugin_manager
            .try_lock()
            .map_err(|_| RuntimeError::RuntimeError)?;

        let plugin = pm.get_plugin(&plugin_name)?;

        let mut scope = child_scope(Arc::clone(&self.global_scope), &stage_name);

        match plugin.execute(&scope, &execution_config) {
            Ok(res) => {
                scope.insert_result(stage_name, res.clone())?;
                self.global_scope = Arc::new(Mutex::new(scope));

                Ok(self.global_scope.clone())
            }
            Err(e) => Err(RuntimeError::PluginError(*e)),
        }
    }

    /// The function `get_scope` returns a clone of the global scope if it can be locked, otherwise it
    /// returns a `RuntimeError`.
    ///
    /// Returns:
    ///
    /// The function `get_scope` returns a `RuntimeResult` containing a `Scope` object.
    pub fn get_scope(&self) -> RuntimeResult<Scope> {
        let global_scope = self
            .global_scope
            .try_lock()
            .map_err(|_| RuntimeError::RuntimeError)?;
        Ok(global_scope.clone())
    }

    /// The function `get_results` returns the execution result for a given stage name, or an error if
    /// no result is found.
    ///
    /// Arguments:
    ///
    /// * `stage_name`: The `stage_name` parameter is a `String` that represents the name of a stage.
    ///
    /// Returns:
    ///
    /// The function `get_results` returns a `RuntimeResult<ExecutionResult>`.
    pub fn get_results(&self, stage_name: String) -> RuntimeResult<ExecutionResult> {
        let global_scope = self
            .global_scope
            .try_lock()
            .map_err(|_| RuntimeError::RuntimeError)?;
        match global_scope.results.get(&stage_name) {
            Some(e) => Ok(e.clone()),
            None => Err(RuntimeError::NoExecutionResultFound),
        }
    }

    /// The `load_plugins` method in the `Runner` struct is responsible for loading all plugins
    /// specified in the runtime configuration. It iterates over the built-in plugins and calls the
    /// `load_plugin` method for each plugin. This method is used to load a specific plugin by name and
    /// path. The loaded plugins are stored in the `PluginManager` instance associated with the
    /// `Runner`.
    pub fn load_plugins(&mut self) -> RuntimeResult<()> {
        let runtime_config = self.config.clone();
        for (name, path) in BUILT_IN_PLUGINS.iter() {
            debug!("Loading plugin: {} from {}", name, path.display());
            self.load_plugin(name, path, &runtime_config)?;
        }
        Ok(())
    }

    pub fn load_plugin<P: AsRef<OsStr>>(
        &mut self,
        name: &str,
        path: P,
        config: &RuntimeConfig,
    ) -> RuntimeResult<()> {
        let mut plugin_registry = (*self.plugin_manager)
            .try_lock()
            .map_err(|_| RuntimeError::RuntimeError)?;

        match plugin_registry.get_plugin(name) {
            Ok(_) => {
                debug!("Plugin {} already loaded", name);
                return Ok(());
            }
            Err(_) => {
                debug!("Plugin {} not loaded", name);
                unsafe {
                    plugin_registry
                        .load_plugin(name, path, config)
                        .map_err(|e| {
                            debug!("Error loading plugin: {}", e);
                            RuntimeError::RuntimeError
                        })?
                }
            }
        }

        debug!("Loaded all plugins");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::variables::VariableKey;

    use super::*;

    #[test]
    fn test_new_executor_can_be_created() {
        let _res = Runner::new(RuntimeConfig::default());
    }

    #[test]
    fn test_can_add_global_variables_to_scope() {
        let mut exec = Runner::new(RuntimeConfig::default());
        exec.add_global_variable("test", "test".into());
        let res = exec.get_scope();
        assert!(res.is_ok());
        let scope = res.unwrap();

        let key: VariableKey = "test".try_into().to_owned().unwrap();
        let test_var = scope.get_variable(&key);
        assert!(test_var.is_some());
        let test_var = test_var.unwrap();
        assert_eq!(test_var.original, "test".to_owned());
    }

    #[test]
    fn test_can_add_global_environment_to_scope() {
        let mut exec = Runner::new(RuntimeConfig::default());
        exec.add_global_environment("test", Some("test".to_string()));
        let res = exec.get_scope();
        assert!(res.is_ok());
        let scope = res.unwrap();

        let test_var = scope.get_env("test");
        assert!(test_var.is_some());
        let test_var = test_var.unwrap();
        assert_eq!(test_var.to_owned(), "test".to_owned());
    }

    #[test]
    fn test_can_load_built_in_plugins() {
        let mut exec = Runner::new(RuntimeConfig::default());
        let res = exec.load_plugins();
        assert!(res.is_ok());
        let loaded_plugins = exec.plugin_manager.try_lock().unwrap();
        let is_loaded = loaded_plugins.get_plugin("system-shell");
        assert!(is_loaded.is_ok());
    }

    #[test]
    fn test_executor_can_execute_with_args() {
        let mut exec = Runner::new(RuntimeConfig::default());
        let res = exec.load_plugins();
        assert!(res.is_ok());
        let mut execute_config = ExecuteConfig::default();

        let stage_name = "test-stage".to_string();
        execute_config
            .args
            .push(r#"echo "Hello World""#.to_string());
        let res = exec.execute(stage_name.clone(), execute_config);
        assert!(res.is_ok());
        let res = exec.get_results(stage_name);
        assert!(res.is_ok());
        let exec_result = res.unwrap();
        assert_eq!(exec_result.stdout, "Hello World");
    }

    #[test]
    fn test_executor_can_execute_with_option() {
        let mut exec = Runner::new(RuntimeConfig::default());
        let res = exec.load_plugins();
        assert!(res.is_ok());
        let mut execute_config = ExecuteConfig::default();
        execute_config.args = vec![];

        execute_config.plugin_name = "deno".to_string();
        let stage_name = "test-stage".to_string();

        let code = r#"
        export default function() {
            return "Hello World";
        }
        "#;
        execute_config.options.insert(
            "code".to_string(),
            crate::PluginOption::String(code.to_string()),
        );
        let res = exec.execute(stage_name.clone(), execute_config);
        assert!(res.is_ok());
        let res = exec.get_results(stage_name);
        assert!(res.is_ok());
        let exec_result = res.unwrap();
        assert_eq!(exec_result.stdout, "\"Hello World\"");
    }

    #[test]
    fn test_executor_can_execute_with_path_set_in_args() {
        let mut exec = Runner::new(RuntimeConfig::default());
        let res = exec.load_plugins();
        assert!(res.is_ok());
        let mut execute_config = ExecuteConfig::default();

        let fixtures_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures");
        let code_path = fixtures_directory
            .join("simple.js")
            .into_os_string()
            .into_string()
            .unwrap();

        execute_config.args = vec![code_path];
        execute_config.plugin_name = "deno".to_string();

        let stage_name = "test-stage".to_string();
        let res = exec.execute(stage_name.clone(), execute_config);
        assert!(res.is_ok());
        let res = exec.get_results(stage_name);
        assert!(res.is_ok());
        let exec_result = res.unwrap();
        assert_eq!(exec_result.stdout, "\"Hello World\"");
    }
}

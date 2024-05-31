use serde_json::Value;
use std::{any::Any, ffi::OsStr, path::PathBuf};

use libloading::{Library, Symbol};
use tracing::debug;

use std::fs;

use super::built_in::BUILT_IN_PLUGINS;
use crate::{
    core::config::{ExecuteConfig, RuntimeConfig},
    errors::{PluginError, PluginResult},
    exec::scope::ExecutionResult,
    Scope,
};

pub trait Extension: Any + Send + Sync {
    /// Plugin name
    fn name(&self) -> &'static str;

    /// When the plugin loads
    fn on_load(&mut self, _config: RuntimeConfig) {}

    /// Opportunity to clean up
    fn on_unload(&self) {}

    fn register_action(&self) -> &'static str;

    //TODO: register config object
    //TODO: register config info//required?
    //TODO: register config defaults
    //TODO: register config validation function
    //TODO: register a system config object? like how vscode lets you configure extensions
    //TODO: How do we make it so a heavy model can be downloaded and used by the plugin?
}

pub trait ExecutionPlugin: Extension {
    fn execute(
        &self,
        scope: &Scope,
        config: &ExecuteConfig,
    ) -> Result<ExecutionResult, Box<PluginError>>;
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::ExecutionPlugin {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::ExecutionPlugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

#[derive(Default)]
pub struct PluginManager {
    plugins: Vec<(String, Box<dyn ExecutionPlugin>)>,
    loaded_libraries: Vec<Library>,
    action_folder: PathBuf,
}

impl std::fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginManager").finish()
    }
}

impl PluginManager {
    // pub fn new() -> Self {
    pub fn new(runtime: &RuntimeConfig, action_folder: PathBuf) -> Self {
        // println!("Creating new PluginManager");
        // Self::default()
        let mut manager = Self::default();
        manager.action_folder = action_folder;
        manager.load_plugins(runtime).unwrap_or_else(|e| {
            eprintln!("Error loading plugins: {}", e);
        });

        // let _ = manager.hydrate_plugin_actions();
        manager
    }

    // pub fn hydrate_plugin_actions(&mut self) -> PluginResult<()> {
    //     // for plugin in &self.plugins {
    //     //     // Your code to loop through the plugins goes here
    //     //     // For example:
    //     //     plugin.register_action();
    //     // }

    //     for (_, plugin) in &self.plugins {
    //         // plugin.register_action();

    //         let plugin_config = plugin.register_action();
    //         debug!("Plugin config: {:?}", plugin_config);
    //         //this is really the base folder
    //         let plugin_folder = self.action_folder.join("actions");
    //         let config_folder = plugin_folder.join(plugin.name());
    //         debug!("Plugin folder: {:?}", plugin_folder);
    //         // Create the plugin folder if it doesn't exist
    //         fs::create_dir_all(&plugin_folder)?;
    //         debug!("Made plugin folder: {:?}", config_folder);
    //         // Create the actions folder if it doesn't exist
    //         fs::create_dir_all(&config_folder)?;
    //         debug!("Made Config folder: {:?}", config_folder);

    //         // Write the JSON file inside the actions folder
    //         let config_path = config_folder.join("config.json");

    //         println!("Config path created: {:?}", config_path);
    //     }

    //     debug!("Hydrated plugin actions");
    //     // fs::write(config_path, serde_json::to_string_pretty(&plugin_config)?)?;
    //     Ok(())
    // }

    /// The `load_plugins` method in the `Runner` struct is responsible for loading all plugins
    /// specified in the runtime configuration. It iterates over the built-in plugins and calls the
    /// `load_plugin` method for each plugin. This method is used to load a specific plugin by name and
    /// path. The loaded plugins are stored in the `PluginManager` instance associated with the
    /// `Runner`.
    pub fn load_plugins(&mut self, config: &RuntimeConfig) -> PluginResult<()> {
        // pub fn load_plugins(&mut self) -> PluginResult<()> {
        // let runtime_config = self.config.clone();
        //TODO: update how we load plugins and load them from new source in WASM
        for (name, path) in BUILT_IN_PLUGINS.iter() {
            debug!("Loading plugin: {} from {}", name, path.display());
            println!("Loading plugin: {} from {}", name, path.display());
            // self.load_plugin(name, path)?;
            unsafe {
                match self.load_plugin(name, path, config) {
                    Ok(_) => {
                        debug!("Loaded plugin: {}", name);
                    }
                    Err(e) => {
                        debug!("Error loading plugin: {}", e);
                        // PluginError::PluginError;
                    }
                }
            }
        }
        Ok(())
    }

    // pub fn load_plugin<P: AsRef<OsStr>>(
    //     &mut self,
    //     name: &str,
    //     path: P,
    //     // config: &RuntimeConfig,
    // ) -> PluginResult<()> {
    //     // let mut plugin_registry = (*self).try_lock().map_err(|_| PluginResult::PluginError)?;

    //     match self.get_plugin(name) {
    //         Ok(_) => {
    //             debug!("Plugin {} already loaded", name);
    //             return Ok(());
    //         }
    //         Err(_) => {
    //             debug!("Plugin {} not loaded", name);
    //             unsafe {
    //                 self.load_plugin(name, path, config).map_err(|e| {
    //                     debug!("Error loading plugin: {}", e);
    //                     PluginManager::RuntimeError
    //                 })?
    //             }
    //         }
    //     }

    //     debug!("Loaded all plugins");
    //     println!("Loaded all plugins");

    //     Ok(())
    // }

    /// Find the plugin by name in either the local default path
    /// or the workspace path
    /// This currently only loads plugins from the system path
    /// but may be extended to load from a remote source
    pub fn find_plugin_library(plugin_name: &str) -> PluginResult<&PathBuf> {
        for (name, plugin) in BUILT_IN_PLUGINS.iter() {
            if *name == plugin_name {
                return Ok(plugin);
            }
        }
        // TODO: Load from workspace
        // TODO: Load from url
        // TODO: search marketplace
        Err(PluginError::NotFound(plugin_name.to_string()))
    }

    ///Old version
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(
        &mut self,
        name: &str,
        path: P,
        config: &RuntimeConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.get_plugin(name).is_ok() {
            return Ok(());
        }

        type PluginCreate = unsafe fn() -> *mut dyn ExecutionPlugin;

        debug!("Loading plugin: {} from {:?}", name, path.as_ref());
        // Load and initialize library
        #[cfg(target_os = "linux")]
        let lib: Library = {
            // Load library with `RTLD_NOW | RTLD_NODELETE` to fix a SIGSEGV
            ::libloading::os::unix::Library::open(Some(path.as_ref()), 0x2 | 0x1000)?.into()
        };
        #[cfg(not(target_os = "linux"))]
        let lib = Library::new(path.as_ref()).map_err(|e| PluginError::LoadingError(e))?;
        debug!("Loaded library: {:?}", lib);

        self.loaded_libraries.push(lib);

        // debug!("Loaded plugin: {}", name);
        // println!("Loaded plugin: {}", name);

        let lib = self.loaded_libraries.last().unwrap();
        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;
        // let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func")?;
        let boxed_raw = constructor();

        let mut plugin = Box::from_raw(boxed_raw);

        debug!("Loaded plugin: {}", name);

        let plugin_action = plugin.register_action();
        debug!("Plugin config: {:?}", plugin_action);

        //parse the plugin_action to JSON value
        // let json_data = serde_json::from_str(&plugin_action)?;
        //this is really the base folder
        let plugin_folder = self.action_folder.join("actions");
        let config_folder = plugin_folder.join(plugin.name());
        debug!("Plugin folder: {:?}", plugin_folder);
        // Create the plugin folder if it doesn't exist
        fs::create_dir_all(&plugin_folder)?;
        debug!("Made plugin folder: {:?}", config_folder);
        // Create the actions folder if it doesn't exist
        fs::create_dir_all(&config_folder)?;
        debug!("Made Config folder: {:?}", config_folder);

        // Write the JSON file inside the actions folder
        let config_path = config_folder.join("config.json");

        //write the json to file
        // let nice_json_data = serde_json::to_string_pretty(&json_data)?;
        fs::write(config_path, plugin_action)?;

        // println!("Config path created: {:?}", config_path);

        debug!(
            "Loaded plugin `{}` as `{}` from {:?}",
            plugin.name(),
            name,
            path.as_ref()
        );

        plugin.on_load(config.clone());

        self.plugins.push((name.to_string(), plugin));
        Ok(())
    }

    /// The `unload` function unloads plugins and drops loaded libraries.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for (name, plugin) in self.plugins.drain(..) {
            debug!("Unloading plugin `{}`", name);
            plugin.on_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }

    /// Get a plugin by name
    /// that is already loaded
    ///
    /// Errors if the plugin is not found
    pub fn get_plugin(&self, plugin_name: &str) -> PluginResult<&Box<dyn ExecutionPlugin>> {
        for (name, plugin) in &self.plugins {
            if name == plugin_name {
                return Ok(plugin);
            }
        }

        Err(PluginError::NotFound(plugin_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_find_an_existing_built_in_plugin() {
        // let manager = PluginManager::new();
        let p = PluginManager::find_plugin_library("system-shell");
        assert!(p.is_ok());
        assert!(p
            .unwrap()
            .to_str()
            .unwrap()
            .contains("plugins/artifacts/libanything_plugin_system_shell.dylib"));
    }

    #[test]
    fn test_errors_with_non_existing_plugin() {
        // let manager = PluginManager::new();
        let p = PluginManager::find_plugin_library("bonkders");
        assert!(p.is_err());
    }

    // #[test]
    // fn test_load_simple_plugin() {
    //     let mut manager = PluginManager::new();
    //     let config = RuntimeConfig::default();
    //     let plugin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("../plugins/artifacts")
    //         .join("libanything_plugin_system_shell.dylib");

    //     unsafe {
    //         manager
    //             .load_plugin(
    //                 "system-shell",
    //                 plugin_path.to_owned().into_os_string(),
    //                 &config,
    //             )
    //             .unwrap();
    //     }
    //     assert_eq!(manager.plugins.len(), 1);
    //     assert_eq!(manager.loaded_libraries.len(), 1);
    //     let p = manager.get_plugin("system-shell");
    //     assert!(p.is_ok());
    // }
}

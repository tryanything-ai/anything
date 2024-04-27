// TODO: make this better
pub use crate::{
    core::compute::Computable,
    core::config::{ExecuteConfig, RuntimeConfig},
    core::environment::{Environment, RawEnvironment},
    core::variables::{RawVariables, ValueKind},
    errors::*,
    errors::{PluginError, PluginResult},
    // exec::{cmd::CommandExt, runner::Runner, scope::ExecutionResult, scope::Scope},
    exec::{cmd::CommandExt, scope::ExecutionResult, scope::Scope},
    plugins::manager::{ExecutionPlugin, Extension},
    plugins::options::PluginOption,
};

pub use crate::raw::*;

pub use crate::exec::interpreters::{
    plugin::{EngineOption, PluginEngine},
    system::SystemShell,
    EngineKind,
};
pub use crate::exec::template::render_string;

pub use crate::declare_plugin;
pub use crate::utils::de::*;

#[cfg(debug_assertions)]
pub use crate::core::config::ExecuteConfigBuilder;

use std::num::NonZeroUsize;

use anything_runtime::RuntimeConfig;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize, PartialEq)]
#[builder(setter(into, strip_option), default)]
pub struct DatabaseConfig {
    pub uri: Option<String>,
    pub max_connections: Option<u32>,
}

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize, PartialEq)]
#[builder(setter(into, strip_option), default)]
pub struct ExecutionConfig {
    pub max_parallelism: Option<NonZeroUsize>,
}

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize, PartialEq)]
#[builder(setter(into, strip_option), default)]
pub struct AnythingConfig {
    runtime_config: RuntimeConfig,
    database: DatabaseConfig,
    execution: ExecutionConfig,
}

impl AnythingConfig {
    pub fn new(runtime_config: RuntimeConfig) -> Self {
        Self {
            runtime_config,
            database: DatabaseConfig::default(),
            execution: ExecutionConfig::default(),
        }
    }

    pub fn runtime_config(&self) -> &RuntimeConfig {
        &self.runtime_config
    }

    pub fn update_runtime_config(&mut self, new_config: RuntimeConfig) {
        self.runtime_config = new_config;
    }

    pub fn database_config(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn execution_config(&self) -> &ExecutionConfig {
        &self.execution
    }
}

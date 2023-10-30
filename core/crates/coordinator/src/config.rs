use anything_runtime::RuntimeConfig;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize, PartialEq)]
#[builder(setter(into, strip_option), default)]
pub struct AnythingConfig {
    runtime_config: RuntimeConfig,
}

impl AnythingConfig {
    pub fn new(runtime_config: RuntimeConfig) -> Self {
        Self { runtime_config }
    }

    pub fn runtime_config(&self) -> &RuntimeConfig {
        &self.runtime_config
    }

    pub fn update_runtime_config(&mut self, new_config: RuntimeConfig) {
        self.runtime_config = new_config;
    }
}

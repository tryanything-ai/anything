use anything_runtime::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

use crate::error::{GraphError, GraphResult};

use super::{flow::Flow, node::Task, trigger::Trigger};

fn default_version() -> Option<String> {
    Some("0.0.1".to_string())
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
        // flow.nodes = self.nodes;

        toml::to_string(&flow).unwrap()
    }
}

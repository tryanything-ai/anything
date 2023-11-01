use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

use crate::plugins::options::PluginOption;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum EngineOption {
    #[default]
    None,
    String(String),
    List(Vec<EngineOption>),
    Map(indexmap::IndexMap<String, EngineOption>),
}

impl From<String> for EngineOption {
    fn from(value: String) -> Self {
        Self::from(toml::Value::from(value))
    }
}

impl From<toml::Value> for EngineOption {
    fn from(value: toml::Value) -> Self {
        match value {
            toml::Value::Boolean(b) => Self::String(b.to_string()),
            toml::Value::Integer(n) => Self::String(n.to_string()),
            toml::Value::Float(n) => Self::String(n.to_string()),
            toml::Value::String(s) => Self::String(s),
            toml::Value::Array(a) => Self::List(a.into_iter().map(|v| v.into()).collect()),
            toml::Value::Table(o) => Self::Map(o.into_iter().map(|(k, v)| (k, v.into())).collect()),
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct PluginEngine {
    pub engine: String,
    pub args: Option<Vec<String>>,
    pub options: indexmap::IndexMap<String, EngineOption>,
}

impl Hash for PluginEngine {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.engine.hash(state);
        self.args.hash(state);
    }
}

impl Eq for PluginEngine {}

impl PluginEngine {
    pub fn get_from_string(input: &str) -> Option<Self> {
        let mut parts = input.split(' ');
        let mut args: Vec<String> = Vec::new();
        let options = indexmap::IndexMap::new();

        if let Some(value) = parts.next() {
            while let Some(arg) = parts.next() {
                args.push(arg.to_string());
            }

            Some(Self {
                engine: value.to_string(),
                args: Some(args),
                options,
            })
        } else {
            None
        }
    }
}

impl Display for PluginEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(args) = &self.args.as_ref() {
            write!(f, "{} {}", self.engine, args.join(" "))
        } else {
            write!(f, "{}", self.engine)
        }
    }
}

impl From<EngineOption> for PluginOption {
    fn from(value: EngineOption) -> Self {
        match value {
            EngineOption::None => PluginOption::None,
            EngineOption::String(s) => PluginOption::String(s),
            EngineOption::List(l) => PluginOption::List(l.into_iter().map(|v| v.into()).collect()),
            EngineOption::Map(m) => {
                PluginOption::Map(m.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {}

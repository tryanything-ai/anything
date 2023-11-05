use std::hash::Hash;

use regex::Match;
use serde::{Deserialize, Serialize};

use crate::{
    core::compute::Computable,
    errors::{RuntimeError, RuntimeResult},
    exec::{scope::Scope, template::render_string},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Http {
    fetch: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct File {
    file: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(untagged)]
pub enum ValueKind {
    #[default]
    None,
    String(String),
    Http(Http),
    File(File),
    Int(i64),
    Computable(Computable),
    List(Vec<ValueKind>),
}

impl Into<String> for ValueKind {
    fn into(self) -> String {
        match self {
            ValueKind::None => "".to_string(),
            ValueKind::String(s) => s,
            ValueKind::Http(h) => h.fetch,
            ValueKind::File(f) => f.file,
            ValueKind::Int(i) => i.to_string(),
            ValueKind::Computable(c) => c.into(),
            ValueKind::List(l) => l
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

impl From<Match<'_>> for ValueKind {
    fn from(value: Match) -> Self {
        Self::String(value.as_str().to_string())
    }
}

impl From<String> for ValueKind {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RawVariables {
    #[serde(flatten)]
    pub vars: indexmap::IndexMap<String, ValueKind>,
}

impl Hash for RawVariables {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (var_name, _value) in self.vars.iter() {
            var_name.hash(state)
        }
    }
}

impl IntoIterator for ValueKind {
    type IntoIter = std::vec::IntoIter<ValueKind>;
    type Item = ValueKind;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueKind::List(l) => l
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<ValueKind>>()
                .into_iter(),
            _ => vec![self].into_iter(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct VariableKey {
    pub original: String,
    pub name: String,
    pub set_global: bool,
    pub raw: bool,
}

impl VariableKey {
    pub fn new(name: &str, is_global: Option<bool>) -> RuntimeResult<Self> {
        parse_key(name, is_global)
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl TryFrom<String> for VariableKey {
    type Error = RuntimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        parse_key(&value, None)
    }
}

impl From<&str> for VariableKey {
    fn from(value: &str) -> Self {
        VariableKey::new(value, None).unwrap()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct VariableValue {
    pub name: String,
    pub original: String,
    pub rendered: Option<String>,
    pub raw: bool,
}

impl VariableValue {
    pub fn new(name: &str, raw: &str) -> Self {
        Self {
            name: name.to_string(),
            original: raw.to_string(),
            rendered: None,
            raw: false,
        }
    }

    pub fn get_rendered_value(&self) -> String {
        if self.raw {
            return self.original.clone();
        }
        self.rendered.clone().unwrap_or_default()
    }

    pub fn with_rendered_value(mut self, scope: &Scope) -> RuntimeResult<Self> {
        let rendered = render_string(&self.name, &self.original, scope)?;
        self.rendered = Some(rendered);
        Ok(self)
    }

    pub fn as_raw(mut self) -> RuntimeResult<Self> {
        self.raw = true;
        Ok(self)
    }
}

impl TryFrom<String> for VariableValue {
    type Error = RuntimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value, &value).with_rendered_value(&Scope::default())
    }
}

impl From<&str> for VariableValue {
    fn from(value: &str) -> Self {
        VariableValue::new(value, value)
    }
}

impl RawVariables {
    /// Add a variable from a string into the variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anything_runtime::prelude::*;
    ///
    /// let mut raw_variables = RawVariables::default();
    /// raw_variables.add_variables("name=bob".to_string());
    /// ````
    pub fn add_variables(&mut self, input: String) -> RuntimeResult<()> {
        for line in input.lines() {
            // Split the line by possible delimiters
            let parts: Vec<&str> = line
                .split(|c| c == '=' || c == ':')
                .map(|part| part.trim()) // Trim whitespace
                .collect();

            if parts.len() == 2 {
                let key = parts[0];
                let value = ValueKind::String(parts[1].to_string());
                self.add(key, value);
            }
        }
        Ok(())
    }

    pub fn add(&mut self, name: &str, value: ValueKind) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&ValueKind> {
        self.vars.get(name)
    }
}

impl From<String> for RawVariables {
    fn from(value: String) -> Self {
        let mut raw_variables = RawVariables::default();
        let _ = raw_variables.add_variables(value);
        raw_variables
    }
}

impl From<Vec<String>> for RawVariables {
    fn from(items: Vec<String>) -> Self {
        Self {
            vars: items
                .into_iter()
                .map(|item| (item.clone(), ValueKind::String(item)))
                .collect::<indexmap::IndexMap<String, ValueKind>>(),
        }
    }
}

impl Default for RawVariables {
    fn default() -> Self {
        Self {
            vars: indexmap::IndexMap::new(),
        }
    }
}

impl From<Vec<(String, String)>> for RawVariables {
    fn from(items: Vec<(String, String)>) -> Self {
        Self {
            vars: items
                .into_iter()
                .map(move |(k, v)| (k, ValueKind::String(v)))
                .collect::<indexmap::IndexMap<String, ValueKind>>(),
        }
    }
}

pub type Variables = indexmap::IndexMap<VariableKey, VariableValue>;

fn parse_key(value: &str, is_global: Option<bool>) -> RuntimeResult<VariableKey> {
    let mut parts = value.split(|c| c == ' ' || c == '\t');

    let name = parts.next().ok_or(RuntimeError::InvalidVariableName)?;

    let mut key = VariableKey {
        original: value.to_string(),
        name: name.to_string(),
        set_global: is_global.unwrap_or(false),
        raw: false,
    };

    for part in parts {
        if part == "raw" {
            key.raw = true;
        } else {
            return Err(RuntimeError::InvalidVariableName);
        }
    }

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_variable_key() {
        let res = parse_key("name", None).unwrap();

        let expected = VariableKey {
            original: "name".to_string(),
            name: "name".to_string(),
            set_global: false,
            raw: false,
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn test_invalid_key() {
        let res = parse_key("bob notraw", None);
        assert!(res.is_err());
    }

    #[test]
    fn test_from_string() {
        let mut raw_variables = RawVariables::default();

        let _ = raw_variables.add_variables("name=bob".to_string());
        assert_eq!(
            raw_variables.get("name"),
            Some(&ValueKind::String("bob".to_string()))
        );
        let _ = raw_variables.add_variables(
            r#"
        name= Ari
        friend_one = Carl
        friend_two: Chris
        "#
            .to_string(),
        );
        assert_eq!(
            raw_variables.get("name"),
            Some(&ValueKind::String("Ari".to_string()))
        );
        assert_eq!(
            raw_variables.get("friend_one"),
            Some(&ValueKind::String("Carl".to_string()))
        );
        assert_eq!(
            raw_variables.get("friend_two"),
            Some(&ValueKind::String("Chris".to_string()))
        );
    }

    #[test]
    fn test_from_vec() {
        let raw_variables = RawVariables::from(vec![
            ("name".to_string(), "bob".to_string()),
            ("friend_one".to_string(), "Carl".to_string()),
            ("friend_two".to_string(), "Chris".to_string()),
        ]);

        assert_eq!(
            raw_variables.get("name"),
            Some(&ValueKind::String("bob".to_string()))
        );
        assert_eq!(
            raw_variables.get("friend_one"),
            Some(&ValueKind::String("Carl".to_string()))
        );
        assert_eq!(
            raw_variables.get("friend_two"),
            Some(&ValueKind::String("Chris".to_string()))
        );
    }

    #[test]
    fn test_raw_variables_from_string() {
        let raw_variables = RawVariables::from("name=bob".to_string());

        assert_eq!(
            raw_variables.get("name"),
            Some(&ValueKind::String("bob".to_string()))
        );
    }
}

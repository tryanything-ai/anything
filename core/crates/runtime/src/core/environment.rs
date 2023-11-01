use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::{exec::scope::Scope, RuntimeResult};

use super::evaluate::Evaluatable;

pub type Environment<T> = indexmap::IndexMap<T, String>;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct RawEnvironment<T> {
    #[serde(flatten)]
    pub vars: indexmap::IndexMap<String, T>,
}

impl<T> Hash for RawEnvironment<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (name, _var) in self.vars.iter() {
            name.hash(state);
        }
    }
}

impl<T> Default for RawEnvironment<T> {
    fn default() -> Self {
        Self {
            vars: indexmap::IndexMap::new(),
        }
    }
}

impl<T> RawEnvironment<T>
where
    T: From<String> + Evaluatable,
{
    pub fn add(&mut self, name: &str, val: String) {
        self.vars.insert(name.to_string(), val.into());
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    pub fn evaluate(&self, parent_scope: &Scope) -> RuntimeResult<Environment<String>> {
        let local_scope = parent_scope.clone();
        let mut vars = Environment::default();
        for (key, value) in &self.vars {
            match value.evaluate(key, &local_scope) {
                Ok(value) => {
                    // local_scope.insert_var(&key, &value);
                    vars.insert(key.clone(), value)
                }
                Err(error) => return Err(error),
            };
        }
        Ok(vars)
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        match self.contains_key(name) {
            true => self.vars.get(name),
            false => None,
        }
    }
}

// impl<T> IntoIterator for RawEnvironment<T> {
//     type IntoIter = std::collections::hash_map::IntoIter<String, T>;
//     type Item = (String, T);

//     fn into_iter(self) -> Self::IntoIter {
//         self.vars.into_iter()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_environment() {
        let mut env: RawEnvironment<String> = RawEnvironment::default();
        env.add("AWS_ACCESS_KEY_ID", "ABC123".to_string());
        env.add("GIT_AUTH_TOKEN", "ABC123".to_string());
        assert!(env.vars.contains_key("AWS_ACCESS_KEY_ID"));
        assert!(env.vars.contains_key("GIT_AUTH_TOKEN"));
    }

    #[test]
    fn test_environment_retrieve() {
        let mut env: RawEnvironment<String> = RawEnvironment::default();
        env.add("AWS_ACCESS_KEY_ID", "ABC123".to_string());
        env.add("GIT_AUTH_TOKEN", "ABC123".to_string());
        assert_eq!(
            env.vars.get("AWS_ACCESS_KEY_ID"),
            Some(&"ABC123".to_string())
        );
    }

    #[test]
    fn test_environment_get_variable() {
        let mut env: RawEnvironment<String> = RawEnvironment::default();
        env.add("AWS_ACCESS_KEY_ID", "ABC123".to_string());
        assert_eq!(env.get("AWS_ACCESS_KEY_ID"), Some(&"ABC123".to_string()));
    }
}

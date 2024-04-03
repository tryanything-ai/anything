use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    core::{
        environment::RawEnvironment,
        evaluate::Evaluatable,
        variables::{self, RawVariables, ValueKind, VariableKey, VariableValue, Variables},
    },
    errors::{RuntimeError, RuntimeResult},
    RuntimeConfig,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
    pub result: Value,
}

#[derive(Debug, Default)]
pub struct Scope {
    pub name: String,
    pub runtime_config: RuntimeConfig,
    pub variables: Variables,
    pub environment: indexmap::IndexMap<String, String>,
    pub results: indexmap::IndexMap<String, ExecutionResult>,
    pub parent: Option<Arc<Mutex<Scope>>>,
    pub root: Option<Arc<Mutex<Scope>>>,
}

impl Scope {
    pub fn insert_variable(
        &mut self,
        key: VariableKey,
        value: VariableValue,
    ) -> Option<VariableValue> {
        self.variables.insert(key, value)
    }

    pub fn insert_result(&mut self, key: String, result: ExecutionResult) -> RuntimeResult<()> {
        self.results.insert(key, result);

        Ok(())
    }

    pub fn insert_binding(
        &mut self,
        key: &str,
        value: &str,
        is_global: Option<bool>,
    ) -> RuntimeResult<()> {
        self.variables.insert(
            VariableKey::new(key, is_global)?,
            VariableValue::new(key, value).with_rendered_value(self)?,
        );
        Ok(())
    }

    pub fn set_runtime_config(&mut self, config: &RuntimeConfig) {
        self.runtime_config = config.clone();
    }

    pub fn insert_environment_variable(
        &mut self,
        key: String,
        value: Option<String>,
    ) -> Option<String> {
        match value {
            Some(value) => {
                self.environment.insert(key.clone(), value.clone());
                Some(value)
            }
            None => {
                if let Some(value) = std::env::var_os(&key) {
                    let value = value.to_string_lossy().to_string();
                    self.environment.insert(key.clone(), value.clone());
                    Some(value)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_variable(&self, key: &VariableKey) -> Option<&VariableValue> {
        self.variables.get(key)
    }

    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.environment.get(key)
    }

    pub fn get_result(&self, key: &str) -> Option<&ExecutionResult> {
        self.results.get(key)
    }

    pub fn process_raw_vars(&mut self, variables: &RawVariables) -> RuntimeResult<()> {
        for (orig_key, orig_val) in &variables.vars {
            match orig_val {
                variables::ValueKind::None => return Err(RuntimeError::EmptyVariableValue),
                ValueKind::String(inner) => {
                    let key = VariableKey::try_from(orig_key.clone())?;
                    let val = if key.raw {
                        VariableValue::new(&orig_key, &inner).as_raw()?
                    } else {
                        VariableValue::new(orig_key, inner).with_rendered_value(self)?
                    };

                    self.variables.insert(key.clone(), val.clone());

                    if key.set_global && self.root.is_some() {
                        let mut root_scope = (**(self.root.as_ref().unwrap()))
                            .try_lock()
                            .map_err(|_| RuntimeError::RuntimeError)?;
                        root_scope.insert_variable(key, val);
                    }
                }
                _ => return Err(RuntimeError::Unimplemented),
            }
        }
        Ok(())
    }

    pub fn process_raw_env(&mut self, variables: &RawEnvironment<String>) -> RuntimeResult<()> {
        for (key, val) in &variables.vars {
            let res = val.evaluate(&key, self);
            match res {
                Ok(rendered) => {
                    self.environment.insert(key.to_owned(), rendered.to_owned());
                }
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    pub fn compute_execution_scope(&self) -> RuntimeResult<Scope> {
        let mut scope = Scope {
            name: format!("execution_scope: {:}", self.name),
            ..Default::default()
        };

        let mut ancestors = Vec::new();
        let mut parent_link = self.parent.clone();

        loop {
            if parent_link.is_none() {
                break;
            }

            ancestors.push(Arc::clone(&parent_link.as_ref().unwrap()));
            parent_link = {
                let parent_scope = (**(parent_link.as_ref().unwrap()))
                    .try_lock()
                    .map_err(|_| RuntimeError::ScopeError)?;

                if parent_scope.parent.is_none() {
                    break;
                }
                parent_scope.parent.clone()
            }
        }

        ancestors.reverse();

        for ancestor in ancestors {
            let ancestor_scope = ancestor.try_lock().map_err(|_| RuntimeError::ScopeError)?;

            for (key, value) in &ancestor_scope.environment {
                scope.insert_environment_variable(key.clone(), Some(value.clone()));
            }

            for (key, value) in &ancestor_scope.variables {
                scope.insert_variable(key.clone(), value.clone());
            }
            for (key, value) in &ancestor_scope.results {
                let _ = scope.insert_result(key.clone(), value.clone());
            }
        }

        for (key, value) in &self.variables {
            scope.insert_variable(key.clone(), value.clone());
        }

        for (key, value) in &self.environment {
            scope.insert_environment_variable(key.clone(), Some(value.clone()));
        }
        for (key, value) in &self.results {
            let _ = scope.insert_result(key.clone(), value.clone());
        }

        Ok(scope)
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        let mut scope = Scope {
            parent: self.parent.clone(),
            ..Default::default()
        };

        for (n, v) in self.variables.iter() {
            scope.insert_variable(n.clone(), v.clone());
        }

        for (n, v) in &self.environment {
            scope.insert_environment_variable(n.clone(), Some(v.clone()));
        }

        scope
    }
}

impl TryFrom<&Scope> for tera::Context {
    type Error = crate::errors::RuntimeError;

    fn try_from(source: &Scope) -> Result<Self, Self::Error> {
        let mut context = tera::Context::new();

        let mut ancestors = Vec::new();
        let mut parent_link = source.parent.clone();

        loop {
            if parent_link.is_none() {
                break;
            }
            ancestors.push(Arc::clone(&parent_link.as_ref().unwrap()));

            parent_link = {
                let parent_scope = (**(parent_link.as_ref().unwrap()))
                    .try_lock()
                    .map_err(|_| RuntimeError::ScopeError)?;

                if parent_scope.parent.is_none() {
                    break;
                }
                parent_scope.parent.clone()
            }
        }

        ancestors.reverse();

        for ancestor in ancestors {
            let scope = ancestor.try_lock().map_err(|_| RuntimeError::ScopeError)?;

            for (key, value) in &scope.environment {
                context.insert(key.clone(), value);
            }

            for (key, value) in &scope.variables {
                context.insert(key.get_name(), &value.get_rendered_value());
            }

            for (key, value) in &scope.results {
                context.insert(key.clone(), value);
            }

            for (key, value) in &scope.results {
                context.insert(key.clone(), value);
            }
        }

        for (key, value) in &source.environment {
            context.insert(key.clone(), value);
        }

        for (key, value) in &source.variables {
            context.insert(key.get_name(), &value.get_rendered_value());
        }
        for (key, value) in &source.results {
            context.insert(key.clone(), value);
        }

        Ok(context)
    }
}

#[allow(unused)]
pub fn child_scope(scope_ref: Arc<Mutex<Scope>>, name: &str) -> Scope {
    let binding = (*scope_ref)
        .try_lock()
        .map_err(|_e| RuntimeError::RuntimeError)
        .unwrap();
    let scope = &binding;

    let root = if scope.root.is_some() {
        scope.root.clone()
    } else {
        Some(Arc::clone(&scope_ref))
    };

    Scope {
        name: name.to_string(),
        parent: Some(Arc::clone(&scope_ref)),
        root,
        ..Default::default()
    }
}

#[allow(unused)]
pub fn child_scope_link(scope_ref: Arc<Mutex<Scope>>, name: &str) -> Arc<Mutex<Scope>> {
    Arc::new(Mutex::new(child_scope(scope_ref, name)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_result_succeeds() {
        let mut scope = Scope::default();
        let key = "test-save-result".to_string();

        let mut res = ExecutionResult::default();
        res.stdout = "got it".to_string();

        let _ = scope.insert_result(key.clone(), res);
        let ex = scope.get_result(&key);
        assert!(ex.is_some());
        assert_eq!(ex.unwrap().stdout, "got it".to_string());
    }
}

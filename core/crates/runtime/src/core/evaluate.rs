use crate::{
    errors::RuntimeResult,
    exec::{scope::Scope, template::render_string},
};

pub trait Evaluatable {
    fn evaluate(&self, name: &str, parent_scope: &Scope) -> RuntimeResult<String>;
}

impl Evaluatable for String {
    fn evaluate(&self, name: &str, parent_scope: &Scope) -> RuntimeResult<String> {
        render_string(name, &self, parent_scope)
    }
}

impl Evaluatable for &String {
    fn evaluate(&self, name: &str, parent_scope: &Scope) -> RuntimeResult<String> {
        render_string(name, self, parent_scope)
    }
}

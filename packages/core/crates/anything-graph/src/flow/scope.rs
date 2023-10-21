//! Scope for the graph
//!
//! This scoping enables us to have working info through
//! execution within the context of a flow execution
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Scope {
    pub name: String,
    pub environment: indexmap::IndexMap<String, String>,
    pub parent: Option<Rc<RefCell<Scope>>>,
    pub root: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn insert_enviroment(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
    }
}

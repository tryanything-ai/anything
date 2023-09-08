//! A flowfile description build into a flow
//!
//!

use std::{cell::RefCell, rc::Rc};

use super::{flow::Flow, scope::Scope};

#[derive(Clone, Debug, Default)]
pub struct Flowfile {
    pub flow: Flow,
    pub scope: Rc<RefCell<Scope>>,
}

impl Flowfile {
    pub fn with_scope(scope: Rc<RefCell<Scope>>) -> Self {
        Self {
            scope,
            ..Default::default()
        }
    }
}

use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc},
};

use self::state::{Input, Output};
use crate::error::RunError;

pub mod action;
pub mod state;

/// Example task, which can be used without requiring a custom implementation
#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    id: usize,
    /// Name of the task
    pub name: String,
    /// The group of tasks this task belonds to
    pub group: usize,

    action: Arc<dyn IAction + Send + Sync>,
    dependencies: Vec<usize>,
    variables: Variables,
}

impl Task {
    pub fn new(action: impl IAction + 'static + Send + Sync, name: &str) -> Self {
        Self {
            id: ID_ALLOCATOR.alloc(),
            group: 0,
            name: name.to_owned(),
            action: Arc::new(action),
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependencies(&mut self, dependency_ids: &[usize]) {
        self.dependencies
            .extend(dependency_ids.into_iter().map(|id| id))
    }
}

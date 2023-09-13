use crate::{context::ExecutionContext, error::EngineResult, types::Process};

use super::Engine;
use anything_graph::flow::action::ShellAction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct RestEngine {
    pub config: ShellAction,
    pub process: Option<Process>,
}

impl RestEngine {
    pub fn new() -> Self {
        Self {
            config: ShellAction::default(),
            process: Some(Process::default()),
        }
    }
}

impl Engine for RestEngine {
    /// Execute this engine with the given execution context
    fn run(&mut self, _context: &ExecutionContext) -> EngineResult<()> {
        // REST Call goes here
        self.process = Some(Process::default());
        Ok(())
    }
    fn config(&self) -> &dyn std::any::Any {
        &self.config
    }
    fn process(&self) -> Option<Process> {
        self.process.clone()
    }
}

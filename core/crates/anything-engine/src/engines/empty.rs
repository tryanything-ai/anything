use crate::{context::ExecutionContext, error::EngineResult, types::Process};

use super::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct EmptyEngine {
    pub process: Option<Process>,
}

impl EmptyEngine {
    pub fn new() -> Self {
        Self {
            process: Some(Process::default()),
        }
    }
}

impl Engine for EmptyEngine {
    fn get_process(&self) -> crate::error::EngineResult<Process> {
        match self.process.clone() {
            None => Err(crate::error::EngineError::ShellProcessHasNotRunError),
            Some(v) => Ok(v),
        }
    }
    fn run(&mut self, context: &ExecutionContext) -> EngineResult<()> {
        self.process = Some(Process::default());
        Ok(())
    }
}

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    error::EngineResult,
    types::Process,
};

use super::Engine;
use anything_graph::flow::{action::ShellAction, node::Node};
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
    fn run(&mut self, _context: &NodeExecutionContext) -> EngineResult<()> {
        self.process = Some(Process::default());
        Ok(())
    }
    fn process(&self) -> Option<Process> {
        self.process.clone()
    }
    fn render(
        &mut self,
        node: &Node,
        _global_context: &ExecutionContext,
    ) -> EngineResult<NodeExecutionContext> {
        Ok(NodeExecutionContext {
            node: node.clone(),
            status: None,
            process: None,
        })
    }
}

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    error::EngineResult,
    types::Process,
};

use super::Engine;
use anything_graph::flow::{action::EmptyAction, node::Node};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct EmptyEngine {
    pub config: EmptyAction,
    pub process: Option<Process>,
}

impl EmptyEngine {
    pub fn new(config: EmptyAction) -> Self {
        Self {
            config,
            process: Some(Process::default()),
        }
    }
}

impl Engine for EmptyEngine {
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

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

#[async_trait::async_trait]
impl Engine for EmptyEngine {
    async fn run(&mut self, _context: &NodeExecutionContext) -> EngineResult<Process> {
        self.process = Some(Process::default());
        Ok(self.process.clone().unwrap())
    }
    fn process(&self) -> Option<Process> {
        self.process.clone()
    }
    fn render(
        &mut self,
        node: &Node,
        _global_context: &ExecutionContext,
    ) -> EngineResult<NodeExecutionContext> {
        let exec_context = NodeExecutionContext {
            node: node.clone(),
            status: None,
            process: None,
        };

        Ok(exec_context)
    }
}

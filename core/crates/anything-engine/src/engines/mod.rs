use anything_graph::flow::{action::ActionType, node::Node};

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    error::EngineResult,
    types::Process,
};

use self::{empty::EmptyEngine, rest::RestEngine, shell::ShellEngine};

mod empty;
mod rest;
mod shell;

pub fn get_engine(node: Node) -> Box<dyn Engine + Send + Sync> {
    match node.node_action.action_type {
        ActionType::Shell(config) => Box::new(ShellEngine::new(config.clone())),
        ActionType::Rest(config) => Box::new(RestEngine::new(config.clone())),
        ActionType::Empty(config) => Box::new(EmptyEngine::new(config)),
    }
}

#[async_trait::async_trait]
pub trait Engine {
    /// Run command is called for when it's time to execute the action
    async fn run(&mut self, context: &NodeExecutionContext) -> EngineResult<Process>;
    /// Config retrieves the configuration for the action
    // fn config(&self) -> &dyn std::any::Any;
    /// Process retrieves the process for the action
    fn process(&self) -> Option<Process>;
    /// Render renders the current execution context for this node
    fn render(
        &mut self,
        node: &Node,
        global_context: &ExecutionContext,
    ) -> EngineResult<NodeExecutionContext>;
}

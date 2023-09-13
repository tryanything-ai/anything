use anything_graph::flow::{action::ActionType, node::Node};

use crate::{context::ExecutionContext, error::EngineResult, types::Process};

use self::{empty::EmptyEngine, shell::ShellEngine};

mod empty;
mod rest;
mod shell;

pub fn get_engine(node: Node) -> Box<dyn Engine> {
    match node.node_action.action_type {
        ActionType::Shell(config) => Box::new(ShellEngine::new(config.clone())),
        ActionType::Empty(config) => Box::new(EmptyEngine::new(config)),
    }
}

pub trait Engine {
    fn run(&mut self, context: &ExecutionContext) -> EngineResult<()>;
    fn config(&self) -> &dyn std::any::Any;
    fn process(&self) -> Option<Process>;
}

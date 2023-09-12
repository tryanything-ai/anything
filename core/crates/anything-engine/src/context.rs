use anything_graph::flow::{
    flow::Flow,
    node::{Node, NodeState},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::Process;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ExecutionContext {
    pub uuid: Uuid,
    pub flow: Flow,
    pub executed: Vec<NodeExecutionContext>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NodeExecutionContext {
    pub node: Node,
    pub status: Option<NodeState>,
    pub process: Option<Process>,
}

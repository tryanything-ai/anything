use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::node::{Node, NodeList, NodeState};

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Action {
    id: String,
    valid: bool,
    pub display_name: String,
    pub action_type: ActionType,
    // pub input: Vec<String>,
    // state: NodeState,
    run_result: Option<ActionResult>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionResult {
    pub duration: Duration,
    pub node_execution_error: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub return_code: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ActionType {
    Empty,
    Shell(ShellAction),
    // Request(RequestAction),
    // Native(NativeAction),
    // Wasm(WasmAction),
}

impl Default for ActionType {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ShellAction {
    pub executor: Option<String>,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowTransition {
    pub from: Option<ExecutionState>,
    pub to: ExecutionState,
}

impl FlowTransition {
    pub fn new(from: Option<ExecutionState>, to: ExecutionState) -> Self {
        Self { from, to }
    }
}

pub type FlowSnapshot = Vec<Node>;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeTransition {
    pub node_name: String,
    pub from_state: NodeState,
    pub to_state: NodeState,
}

/// State of execution for Flow transitions
/// demonstrates the state of the flow
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    Started,
    Running,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Transition {
    Flow(FlowTransition),
    Node(NodeTransition),
}

#[derive(Debug, Clone)]
pub struct ExecutionUpdate {
    pub execution_state: ExecutionState,
    pub flow_snapshot: FlowSnapshot,
    pub transition: Transition,
}

impl ExecutionUpdate {
    pub fn new(
        execution_state: ExecutionState,
        flow_snapshot: FlowSnapshot,
        transition: Transition,
    ) -> Self {
        Self {
            execution_state,
            flow_snapshot,
            transition,
        }
    }
}

pub fn get_node_snapshot(node_list: &NodeList) -> FlowSnapshot {
    node_list
        .nodes
        .iter()
        .flat_map(|node_group| node_group.iter().map(|node| node.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{flow::node::NodeList, test_helpers::test_helpers::*};

    #[test]
    fn test_get_node_snapshot_from_list() {
        let sg1 = (0..3)
            .into_iter()
            .map(|s| make_node(&format!("node{}", s), &vec![]))
            .collect();
        let sl = NodeList::new_with_list(sg1).ok();
        assert!(sl.is_some());
        let sl = sl.unwrap();
        assert_eq!(sl.nodes.len(), 1);
    }
}

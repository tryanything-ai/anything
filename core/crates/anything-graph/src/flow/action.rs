use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::node::{Node, StepList, StepState};

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Action {
    id: String,
    valid: bool,
    pub display_name: String,
    pub action_type: ActionType,
    // pub input: Vec<String>,
    // state: StepState,
    run_result: Option<ActionResult>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionResult {
    pub duration: Duration,
    pub step_execution_error: Option<String>,
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
pub struct StepTransition {
    pub step_name: String,
    pub from_state: StepState,
    pub to_state: StepState,
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
    Step(StepTransition),
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

pub fn get_step_snapshot(step_list: &StepList) -> FlowSnapshot {
    step_list
        .steps
        .iter()
        .flat_map(|step_group| step_group.iter().map(|step| step.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{flow::node::StepList, test_helpers::test_helpers::*};

    #[test]
    fn test_get_step_snapshot_from_list() {
        let sg1 = (0..3)
            .into_iter()
            .map(|s| make_node(&format!("step{}", s), &vec![]))
            .collect();
        let sl = StepList::new_with_list(sg1).ok();
        assert!(sl.is_some());
        let sl = sl.unwrap();
        assert_eq!(sl.steps.len(), 1);
    }
}

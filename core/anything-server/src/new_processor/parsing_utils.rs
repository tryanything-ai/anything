use serde_json::Value;

use crate::workflow_types::{Action, WorkflowVersionDefinition};
use crate::task_types::{ActionType, Task};

pub fn get_trigger_node(workflow: &WorkflowVersionDefinition) -> Option<&Action> {
    workflow
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
}

pub fn get_bundle_context_inputs(task: &Task) -> (String, String, Option<&Value>, Option<&Value>) {
    (
        task.account_id.to_string(),
        task.flow_session_id.to_string(), 
        task.config.get("variables"),
        task.config.get("input")
    )
}

use serde_json::Value;

use crate::types::{
    action_types::{Action, ActionType},
    task_types::Task,
    workflow_types::WorkflowVersionDefinition,
};

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
        task.config.get("input"),
    )
}

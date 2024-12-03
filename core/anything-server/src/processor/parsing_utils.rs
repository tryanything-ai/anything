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

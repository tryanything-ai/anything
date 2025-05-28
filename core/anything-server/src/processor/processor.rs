use crate::types::task_types::Task;
use crate::types::workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition};
use std::collections::HashMap;
use uuid::Uuid;
    
#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub workflow_version: DatabaseFlowVersion,
    pub workflow_definition: WorkflowVersionDefinition,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
    pub task_id: Option<Uuid>,               // Add task_id for tracing
    pub existing_tasks: HashMap<Uuid, Task>, // Add any existing tasks from hydration
    // pub workflow_graph: HashMap<String, Vec<String>>, // Pre-computed workflow graph
}

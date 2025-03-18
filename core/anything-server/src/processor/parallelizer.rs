use crate::AppState;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::processor::flow_session_cache::FlowSessionData;
use crate::processor::processor::ProcessorMessage;
use crate::processor::processor_utils::create_task;

use crate::processor::path_processor::process_task_and_branches;
use crate::status_updater::{Operation, StatusUpdateMessage};
use crate::types::task_types::{FlowSessionStatus, TriggerSessionStatus};
use crate::types::workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition};

#[derive(Clone)]
pub struct ProcessingContext {
    pub state: Arc<AppState>,
    pub client: postgrest::Postgrest,
    pub flow_session_id: Uuid,
    pub workflow_id: Uuid,
    pub trigger_task_id: String,
    pub trigger_session_id: Uuid,
    pub workflow: Arc<DatabaseFlowVersion>,
    pub workflow_def: Arc<WorkflowVersionDefinition>,
}

pub async fn process_workflow(
    state: Arc<AppState>,
    client: postgrest::Postgrest,
    processor_message: ProcessorMessage,
) {
    println!(
        "[PROCESSOR] Processing workflow for flow session: {}",
        processor_message.flow_session_id
    );

    // Initialize flow session cache
    let flow_session_data = FlowSessionData {
        tasks: HashMap::new(),
    };

    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(&processor_message.flow_session_id, flow_session_data);
    }

    let ctx = ProcessingContext {
        state: state.clone(),
        client,
        flow_session_id: processor_message.flow_session_id,
        workflow_id: processor_message.workflow_id,
        trigger_task_id: processor_message.trigger_task.clone().unwrap().trigger_id,
        trigger_session_id: processor_message.trigger_session_id,
        workflow: Arc::new(processor_message.workflow_version.clone()),
        workflow_def: Arc::new(processor_message.workflow_version.flow_definition.clone()),
    };

    if let Some(task) = processor_message.trigger_task {
        if let Err(e) = create_task(&ctx, &task).await {
            println!("[PROCESSOR] Failed to create first task: {}", e);
            return;
        }

        process_task_and_branches(Arc::new(ctx), task).await;
    }

    // Update flow session status to completed
    let task_message = StatusUpdateMessage {
        operation: Operation::CompleteWorkflow {
            flow_session_id: processor_message.flow_session_id,
            status: FlowSessionStatus::Completed,
            trigger_status: TriggerSessionStatus::Completed,
        },
    };
    let _ = state.task_updater_sender.send(task_message).await;
}

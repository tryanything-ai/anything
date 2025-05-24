use crate::AppState;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info, instrument, Span};
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

// #[instrument(skip(state, client, processor_message))]
pub async fn process_workflow(
    state: Arc<AppState>,
    client: postgrest::Postgrest,
    processor_message: ProcessorMessage,
) {
    let flow_session_id = processor_message.flow_session_id;
    let workflow_id = processor_message.workflow_id;
    let workflow_version_id = processor_message.workflow_version.flow_version_id;
    let task_id = processor_message.task_id;

    // Create root span with task_id for tracing
    let root_span = tracing::info_span!("process_workflow",
        flow_session_id = %flow_session_id,
        workflow_id = %workflow_id,
        workflow_version_id = %workflow_version_id,
        task_id = task_id.map(|id| id.to_string()).as_deref().unwrap_or("unknown")
    );
    let _root_entered = root_span.enter();
    let workflow_start = Instant::now();
    info!(
        "[PROCESSOR] Processing workflow for flow session: {}",
        processor_message.flow_session_id
    );
    if let Some(task_id) = task_id {
        info!("[PROCESSOR] Processing task_id: {}", task_id);
    }

    // Initialize flow session cache
    let cache_span = tracing::info_span!("init_flow_session_cache");
    let cache_start = Instant::now();
    let flow_session_data = FlowSessionData {
        tasks: HashMap::new(),
    };
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(&processor_message.flow_session_id, flow_session_data);
    }
    let cache_duration = cache_start.elapsed();
    info!(
        "[PROCESSOR] Flow session cache initialized in {:?}",
        cache_duration
    );

    let ctx_span = tracing::info_span!("create_processing_context");
    let ctx_start = Instant::now();
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
    let ctx_duration = ctx_start.elapsed(); 
    info!(
        "[PROCESSOR] ProcessingContext created in {:?}",
        ctx_duration
    );

    if let Some(task) = processor_message.trigger_task {
        let create_task_span = tracing::info_span!("create_first_task");
        let create_task_start = Instant::now();
        if let Err(e) = create_task(&ctx, &task).await {
            error!("[PROCESSOR] Failed to create first task: {}", e);
            return;
        }
        let create_task_duration = create_task_start.elapsed();
        info!(
            "[PROCESSOR] First task created in {:?}",
            create_task_duration
        );

        let process_span = tracing::info_span!("process_task_and_branches");
        let process_start = Instant::now();
        process_task_and_branches(Arc::new(ctx), task).await;
        let process_duration = process_start.elapsed();
        info!(
            "[PROCESSOR] process_task_and_branches completed in {:?}",
            process_duration
        );
    }

    // Update flow session status to completed
    let update_span = tracing::info_span!("update_flow_session_status");
    let update_start = Instant::now();
    let task_message = StatusUpdateMessage {
        operation: Operation::CompleteWorkflow {
            flow_session_id: processor_message.flow_session_id,
            status: FlowSessionStatus::Completed,
            trigger_status: TriggerSessionStatus::Completed,
        },
    };

    // Critical: Properly handle send failure with retries
    let mut retry_count = 0;
    const MAX_SEND_RETRIES: usize = 3;
    const RETRY_DELAY_MS: u64 = 100;

    while retry_count < MAX_SEND_RETRIES {
        match state.task_updater_sender.send(task_message.clone()).await {
            Ok(_) => {
                info!("[PROCESSOR] Successfully sent flow session completion status");
                break;
            }
            Err(e) => {
                retry_count += 1;
                error!(
                    "[PROCESSOR] Failed to send flow session completion status (attempt {}/{}): {}",
                    retry_count, MAX_SEND_RETRIES, e
                );

                if retry_count >= MAX_SEND_RETRIES {
                    // Critical: As a last resort, try direct database update
                    error!("[PROCESSOR] All status update attempts failed, attempting direct database update");
                    match crate::processor::db_calls::update_flow_session_status(
                        &state,
                        &processor_message.flow_session_id,
                        &FlowSessionStatus::Completed,
                        &TriggerSessionStatus::Completed,
                    )
                    .await
                    {
                        Ok(_) => info!("[PROCESSOR] Direct database update succeeded"),
                        Err(db_err) => {
                            error!("[PROCESSOR] Direct database update also failed: {}", db_err)
                        }
                    }
                    break;
                } else {
                    // Wait before retry
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        RETRY_DELAY_MS * retry_count as u64,
                    ))
                    .await;
                }
            }
        }
    }

    let update_duration = update_start.elapsed();
    info!(
        "[PROCESSOR] Flow session status update completed in {:?}",
        update_duration
    );

    let workflow_duration = workflow_start.elapsed();
    info!(
        "[PROCESSOR] process_workflow total duration: {:?}",
        workflow_duration
    );
}

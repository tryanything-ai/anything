use crate::metrics::METRICS;
use crate::processor::parallelizer::process_workflow;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

use tracing::{error, info, warn};
use uuid::Uuid;

// Placeholder function - replace with actual logic to get total permits from AppState
// fn get_total_permits_from_state(state: &Arc<impl HasSemaphore>) -> u64 {
//     // You MUST replace this with the correct way to get your configured max_concurrent_workflows
//     // e.g., state.config.max_concurrent_workflows as u64 or similar.
//     10 // Defaulting to 10 as a placeholder - PLEASE UPDATE
// }

// Implement the HasSemaphore trait for AppState
// impl HasSemaphore for AppState {
//     fn get_semaphore(&self) -> &tokio::sync::Semaphore {
//         &self.workflow_processor_semaphore
//     }
// }

#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub workflow_version: DatabaseFlowVersion,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
}

// #[instrument(skip(state, processor_receiver))]
pub async fn processor(
    state: Arc<AppState>,
    mut processor_receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("[PROCESSOR] Starting processor");

    // Register semaphore metrics callback using the metrics registry
    // METRICS.register_semaphore_metrics(get_total_permits_from_state, state.clone());

    // Keep running until shutdown signal
    loop {
        // Check shutdown signal first
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            info!("[PROCESSOR] Received shutdown signal, stopping processor");
            break;
        }

        // Try to receive a message
        match processor_receiver.recv().await {
            Some(message) => {
                METRICS.processor_messages_received.add(1, &[]); // Increment messages received

                let flow_session_id = message.flow_session_id;
                let workflow_id = message.workflow_id;
                let workflow_version_id = message.workflow_version.flow_version_id;
                let root_span = tracing::info_span!("workflow_lifecycle", flow_session_id = %flow_session_id, workflow_id = %workflow_id, workflow_version_id = %workflow_version_id);
                let _root_entered = root_span.enter();
                let workflow_start = Instant::now();
                info!("[PROCESSOR] Received a new message for processing");
                info!("[PROCESSOR] Received flow_session_id: {}", flow_session_id);

                // Get permit before spawning task
                // let semaphore_span = tracing::info_span!("acquire_semaphore");
                // let semaphore_start = Instant::now();
                match state
                    .workflow_processor_semaphore
                    .clone()
                    .acquire_owned()
                    // .instrument(semaphore_span.clone())
                    .await
                {
                    Ok(permit) => {
                        METRICS.processor_active_workflows.add(1, &[]); // Increment active workflows

                        // let semaphore_duration = semaphore_start.elapsed();
                        // info!(
                        //     "[PROCESSOR] Successfully acquired semaphore permit in {:?}",
                        //     semaphore_duration
                        // );
                        let state = Arc::clone(&state);
                        let client = state.anything_client.clone();

                        // let spawn_span = tracing::info_span!("spawn_workflow", flow_session_id = %flow_session_id);
                        // let spawn_start = Instant::now();
                        let _ = tokio::spawn(async move {
                            let _permit_guard = permit;
                            let workflow_span = tracing::info_span!("workflow_execution", flow_session_id = %flow_session_id);
                            let _entered = workflow_span.enter();
                            let exec_start = Instant::now();
                            info!("[PROCESSOR] Starting workflow {}", flow_session_id);

                            if let Err(e) = tokio::task::spawn(async move {
                                process_workflow(state, (*client).clone(), message).await;
                            })
                            .await
                            {
                                error!("[PROCESSOR] Workflow {} failed with error: {}", flow_session_id, e);
                            }

                            let exec_duration = exec_start.elapsed();
                            METRICS.processor_workflow_duration.record(exec_duration.as_secs_f64(), &[]); // Record duration
                            info!("[PROCESSOR] Completed workflow {} and releasing permit (duration: {:?})", flow_session_id, exec_duration);
                            METRICS.processor_active_workflows.add(-1, &[]); // Decrement active workflows
                        }).await;

                        // .instrument(spawn_span)).await;
                        // let spawn_duration = spawn_start.elapsed();
                        // info!(
                        //     "[PROCESSOR] Workflow spawn+execution took {:?}",
                        //     spawn_duration
                        // );
                    }
                    Err(e) => {
                        error!("[PROCESSOR] Failed to acquire semaphore: {}", e);
                        // Note: METRICS.processor_active_workflows is not incremented here as the workflow didn't start
                        warn!("[PROCESSOR] Continuing to process other messages");
                        continue;
                    }
                }
                let workflow_duration = workflow_start.elapsed();
                info!(
                    "[PROCESSOR] Total workflow lifecycle duration: {:?}",
                    workflow_duration
                );
            }
            None => {
                // Channel was closed - this shouldn't happen unless we're shutting down
                warn!("[PROCESSOR] Channel was closed unexpectedly");
                if !state
                    .shutdown_signal
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    error!("[PROCESSOR] ERROR: Channel closed while processor was still running!");
                }
                break;
            }
        }
    }

    info!("[PROCESSOR] Processor shutdown complete");
    Ok(())
}

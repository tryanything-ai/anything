use crate::processor::parallelizer::process_workflow;
use crate::processor::parallelizer::ProcessingContext;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tracing::Instrument;
use tracing::{error, info, instrument, warn, Span};
use uuid::Uuid;

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
                let flow_session_id = message.flow_session_id;
                let workflow_id = message.workflow_id;
                let workflow_version_id = message.workflow_version.flow_version_id;
                let root_span = tracing::info_span!("workflow_lifecycle", flow_session_id = %flow_session_id, workflow_id = %workflow_id, workflow_version_id = %workflow_version_id);
                let _root_entered = root_span.enter();
                let workflow_start = Instant::now();
                info!("[PROCESSOR] Received a new message for processing");
                info!("[PROCESSOR] Received flow_session_id: {}", flow_session_id);

                // Get permit before spawning task
                let semaphore_span = tracing::info_span!("acquire_semaphore");
                let semaphore_start = Instant::now();
                match state
                    .workflow_processor_semaphore
                    .clone()
                    .acquire_owned()
                    .instrument(semaphore_span.clone())
                    .await
                {
                    Ok(permit) => {
                        let semaphore_duration = semaphore_start.elapsed();
                        info!(
                            "[PROCESSOR] Successfully acquired semaphore permit in {:?}",
                            semaphore_duration
                        );
                        let state = Arc::clone(&state);
                        let client = state.anything_client.clone();

                        let spawn_span = tracing::info_span!("spawn_workflow", flow_session_id = %flow_session_id);
                        let spawn_start = Instant::now();
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
                            info!("[PROCESSOR] Completed workflow {} and releasing permit (duration: {:?})", flow_session_id, exec_duration);
                        }.instrument(spawn_span)).await;
                        let spawn_duration = spawn_start.elapsed();
                        info!(
                            "[PROCESSOR] Workflow spawn+execution took {:?}",
                            spawn_duration
                        );
                    }
                    Err(e) => {
                        error!("[PROCESSOR] Failed to acquire semaphore: {}", e);
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

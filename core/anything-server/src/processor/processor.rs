use crate::metrics::METRICS;
use crate::processor::parallelizer::process_workflow;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

use tracing::{error, info, warn, Span};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub workflow_version: DatabaseFlowVersion,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
    pub task_id: Option<Uuid>, // Add task_id for tracing
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
                METRICS.processor_messages_received.add(1, &[]); // Increment messages received

                let flow_session_id = message.flow_session_id;
                let workflow_id = message.workflow_id;
                let workflow_version_id = message.workflow_version.flow_version_id;
                let task_id = message.task_id;

                // Create root span with task_id for tracing
                let root_span = if let Some(task_id) = task_id {
                    tracing::info_span!("workflow_lifecycle",
                        flow_session_id = %flow_session_id,
                        workflow_id = %workflow_id,
                        workflow_version_id = %workflow_version_id,
                        task_id = %task_id
                    )
                } else {
                    tracing::info_span!("workflow_lifecycle",
                        flow_session_id = %flow_session_id,
                        workflow_id = %workflow_id,
                        workflow_version_id = %workflow_version_id
                    )
                };
                let _root_entered = root_span.enter();
                let workflow_start = Instant::now();
                info!("[PROCESSOR] Received a new message for processing");
                info!("[PROCESSOR] Received flow_session_id: {}", flow_session_id);
                if let Some(task_id) = task_id {
                    info!("[PROCESSOR] Processing task_id: {}", task_id);
                }

                // Get permit before spawning task

                match state
                    .workflow_processor_semaphore
                    .clone()
                    .acquire_owned()
                    // .instrument(semaphore_span.clone())
                    .await
                {
                    Ok(permit) => {
                        METRICS.processor_active_workflows.add(1, &[]); // Increment active workflows

                        let state = Arc::clone(&state);
                        let client = state.anything_client.clone();

                        // Simplified spawn pattern to prevent permit leakage
                        let workflow_handle = tokio::spawn(async move {
                            let _permit_guard = permit; // Ensure permit is released when this task completes
                            let workflow_span = if let Some(task_id) = task_id {
                                tracing::info_span!("workflow_execution",
                                    flow_session_id = %flow_session_id,
                                    task_id = %task_id
                                )
                            } else {
                                tracing::info_span!("workflow_execution", flow_session_id = %flow_session_id)
                            };
                            let _entered = workflow_span.enter();
                            let exec_start = Instant::now();
                            info!("[PROCESSOR] Starting workflow {}", flow_session_id);
                            if let Some(task_id) = task_id {
                                info!("[PROCESSOR] Executing task_id: {}", task_id);
                            }

                            // Process workflow directly without nested spawn
                            process_workflow(state, (*client).clone(), message).await;

                            let exec_duration = exec_start.elapsed();
                            METRICS
                                .processor_workflow_duration
                                .record(exec_duration.as_secs_f64(), &[]); // Record duration
                            info!("[PROCESSOR] Completed workflow {} and releasing permit (duration: {:?})", flow_session_id, exec_duration);
                            METRICS.processor_active_workflows.add(-1, &[]); // Decrement active workflows
                        });

                        // Await the workflow task to ensure proper error handling
                        if let Err(e) = workflow_handle.await {
                            error!(
                                "[PROCESSOR] Workflow {} failed with panic or cancellation: {}",
                                flow_session_id, e
                            );
                            // The permit guard and metrics will still be properly handled due to the Drop trait
                        }
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

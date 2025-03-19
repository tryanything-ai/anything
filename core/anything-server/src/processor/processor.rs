use crate::processor::parallelizer::process_workflow;
use crate::processor::parallelizer::ProcessingContext;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub workflow_version: DatabaseFlowVersion,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
}

pub async fn processor(
    state: Arc<AppState>,
    mut processor_receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESSOR] Starting processor");

    // Keep running until shutdown signal
    loop {
        // Check shutdown signal first
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            println!("[PROCESSOR] Received shutdown signal, stopping processor");
            break;
        }

        // Try to receive a message
        match processor_receiver.recv().await {
            Some(message) => {
                println!("[PROCESSOR] Received a new message for processing");
                let flow_session_id = message.flow_session_id;
                println!("[PROCESSOR] Received flow_session_id: {}", flow_session_id);

                // Get permit before spawning task
                match state
                    .workflow_processor_semaphore
                    .clone()
                    .acquire_owned()
                    .await
                {
                    Ok(permit) => {
                        println!("[PROCESSOR] Successfully acquired semaphore permit");
                        let state = Arc::clone(&state);
                        let client = state.anything_client.clone();

                        tokio::spawn(async move {
                            let _permit_guard = permit;
                            println!("[PROCESSOR] Starting workflow {}", flow_session_id);

                            if let Err(e) = tokio::task::spawn(async move {
                                process_workflow(state, (*client).clone(), message).await;
                            })
                            .await
                            {
                                println!(
                                    "[PROCESSOR] Workflow {} failed with error: {}",
                                    flow_session_id, e
                                );
                            }

                            println!(
                                "[PROCESSOR] Completed workflow {} and releasing permit",
                                flow_session_id
                            );
                        });
                    }
                    Err(e) => {
                        println!("[PROCESSOR] Failed to acquire semaphore: {}", e);
                        println!("[PROCESSOR] Continuing to process other messages");
                        continue;
                    }
                }
            }
            None => {
                // Channel was closed - this shouldn't happen unless we're shutting down
                println!("[PROCESSOR] Channel was closed unexpectedly");
                if !state
                    .shutdown_signal
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    // Log error if we weren't shutting down
                    println!(
                        "[PROCESSOR] ERROR: Channel closed while processor was still running!"
                    );
                }
                break;
            }
        }
    }

    println!("[PROCESSOR] Processor shutdown complete");
    Ok(())
}

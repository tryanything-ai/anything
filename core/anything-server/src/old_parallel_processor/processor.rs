use crate::AppState;
use std::sync::Arc;

use uuid::Uuid;

use crate::processor::parallelizer::start_parallel_workflow_processing;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;

use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub version_id: Option<Uuid>,
    pub workflow_version: DatabaseFlowVersion,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
}

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[OLD PARALLEL PROCESSOR] Starting processor");

    let mut rx = state.processor_receiver.lock().await;
    println!("[PROCESSOR] Successfully acquired receiver lock");

    // Keep track of spawned workflow tasks
    let mut workflow_handles = Vec::new();

    while let Some(message) = rx.recv().await {
        // Check if we received shutdown signal
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            println!(
                "[PROCESSOR] Received shutdown signal, waiting for active workflows to complete"
            );
            break;
        }

        println!(
            "[PROCESSOR] Received flow_session_id: {}",
            message.flow_session_id
        );

        // Clone what we need for the new task
        let state = Arc::clone(&state);

        let number_of_workflow_processors_permit = state
            .workflow_processor_semaphore
            .clone()
            .acquire_owned()
            .await
            .unwrap();

        let client = state.anything_client.clone();

        // Spawn a new task for this workflow
        let handle = tokio::spawn(async move {
            println!("[PROCESSOR] Starting workflow execution");

            // Start parallel workflow processing
            start_parallel_workflow_processing(state.clone(), (*client).clone(), message).await;

            drop(number_of_workflow_processors_permit);
        });

        workflow_handles.push(handle);
    }

    // Wait for all active workflows to complete
    println!(
        "[PROCESSOR] Waiting for {} active workflows to complete",
        workflow_handles.len()
    );
    for handle in workflow_handles {
        if let Err(e) = handle.await {
            println!("[PROCESSOR] Error waiting for workflow to complete: {}", e);
        }
    }
    println!("[PROCESSOR] All workflows completed, shutting down");

    Ok(())
}

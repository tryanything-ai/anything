use crate::processor::utils::{get_workflow_and_tasks_from_cache, is_already_processing};

use crate::AppState;

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;

use crate::types::task_types::Task;

use crate::processor::parallelizer::start_parallel_workflow_processing;

// Add this near your other type definitions
//TODO: probably better just to send workflow in message vs having a cache for it.
//Locks are slow? makes code more complicated
#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub version_id: Option<Uuid>,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
}

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESSOR] Starting processor");

    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));
    let mut rx = state.processor_receiver.lock().await;
    println!("[PROCESSOR] Successfully acquired receiver lock");

    let number_of_processors_semaphore = state.workflow_processor_semaphore.clone();

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

        let workflow_id = message.workflow_id;
        let version_id = message.version_id;
        let flow_session_id = message.flow_session_id;
        let trigger_task = message.trigger_task.clone();
        let trigger_task_id = trigger_task.clone().unwrap().trigger_id;
        let trigger_session_id = message.trigger_session_id;

        println!("[PROCESSOR] Received flow_session_id: {}", flow_session_id);

        // Check if this flow session is already being processed
        if is_already_processing(&active_flow_sessions, flow_session_id).await {
            continue; //SKIP. We are already processing
        }

        // Clone what we need for the new task
        let state = Arc::clone(&state);
        let permit = number_of_processors_semaphore
            .clone()
            .acquire_owned()
            .await
            .unwrap();
        let client = state.anything_client.clone();
        let active_flow_sessions = Arc::clone(&active_flow_sessions);

        // Spawn a new task for this workflow
        let handle = tokio::spawn(async move {
            println!(
                "[PROCESSOR] Starting workflow processing for {}",
                flow_session_id
            );

            // Get workflow definition and cached tasks
            //TOOD: replace with just part of message from triggers?
            let (workflow, cached_tasks) = match get_workflow_and_tasks_from_cache(
                &state,
                flow_session_id,
                &workflow_id,
                &version_id,
            )
            .await
            {
                Ok((workflow, cached_tasks)) => (workflow, cached_tasks),
                Err(e) => {
                    println!("[PROCESSOR] Cannot process workflow: {}", e);
                    // Clean up active sessions before returning
                    active_flow_sessions.lock().await.remove(&flow_session_id);
                    drop(permit);
                    return;
                }
            };

            println!("[PROCESSOR] Starting workflow execution");

            // Start parallel workflow processing
            start_parallel_workflow_processing(
                state.clone(),
                (*client).clone(),
                flow_session_id,
                workflow_id,
                trigger_task_id,
                trigger_session_id,
                workflow,
                message,
                cached_tasks,
            )
            .await;

            println!(
                "[PROCESSOR] Completed workflow processing for {}",
                flow_session_id
            );

            // Invalidate cache for completed flow session
            {
                let mut cache = state.flow_session_cache.write().await;
                cache.invalidate(&flow_session_id);
                println!(
                    "[PROCESSOR] Removed flow session {} from cache",
                    flow_session_id
                );
            }

            // Remove the flow session from active sessions when done
            active_flow_sessions.lock().await.remove(&flow_session_id);
            drop(permit);
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

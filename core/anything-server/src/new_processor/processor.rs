use crate::AppState;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("[PROCESSOR] Starting processor");

    // Create a shared set to track active flow sessions
    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));

    // Get the receiver from the state
    let mut rx = state.processor_receiver.lock().await;
    // Guard againts too many workflows running at once
    let number_of_processors_semaphore = state.workflow_processor_semaphore.clone();

    while let Some(flow_session_id) = rx.recv().await {
        debug!("[PROCESSOR] Received workflow_id: {}", flow_session_id);

        if !flow_session_id.is_empty() {
            // Check if this flow session is already being processed
            let mut active_sessions = active_flow_sessions.lock().await;
            if !active_sessions.insert(flow_session_id.clone()) {
                debug!(
                    "[PROCESSOR] Flow session {} is already being processed, skipping",
                    flow_session_id
                );
                continue;
            }
            drop(active_sessions);

            // Clone what we need for the new task
            let state = Arc::clone(&state);
            let permit = number_of_processors_semaphore.clone().acquire_owned().await.unwrap();
            let client = state.anything_client.clone();
            let active_flow_sessions = Arc::clone(&active_flow_sessions);

            // Spawn a new task for this workflow
            tokio::spawn(async move {
                debug!(
                    "[PROCESSOR] Starting workflow processing for {}",
                    flow_session_id
                );

                // Your workflow processing logic here
                // ...

                debug!(
                    "[PROCESSOR] Completed workflow processing for {}",
                    flow_session_id
                );

                // Remove the flow session from active sessions when done
                active_flow_sessions.lock().await.remove(&flow_session_id);
                drop(permit);
            });
        }
    }

    Ok(())
}
// Process the workflow using your existing task engine logic
// process_flow_tasks(state, &client, &flow_session_id).await;
// Process the workflow...
// TODO: Implementation remains the same
// Process the workflow...
// Process the workflow
//TODO:
//Traverse the worfklow definition to get next task
//Update task status in cache and db
//Bundle the task
//Run task
//Update status and result in cache and db
//Determine if workflow is complete
//If complete, update flow session status in cache and db
//If not complete, update flow session with next task in line
//Send signal to webhook engine if response is needed

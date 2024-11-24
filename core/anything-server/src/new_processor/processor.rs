use crate::AppState;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::debug;

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("[PROCESSOR] Starting processor");

    // Get the receiver from the state
    let mut rx = state.processor_receiver.lock().await; // Lock the mutex to get the receiver
    let semaphore = state.workflow_processor_semaphore.clone();

    while let Some(flow_session_id) = rx.recv().await {
        debug!("[PROCESSOR] Received workflow_id: {}", flow_session_id);

        if !flow_session_id.is_empty() {
            // Clone what we need for the new task
            let state = Arc::clone(&state);
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let client = state.anything_client.clone();

            // Spawn a new task for this workflow
            tokio::spawn(async move {
                debug!(
                    "[PROCESSOR] Starting workflow processing for {}",
                    flow_session_id
                );

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

                debug!(
                    "[PROCESSOR] Completed workflow processing for {}",
                    flow_session_id
                );
                drop(permit); // Release the semaphore when done
            });
        }
    }
    // Process messages in a loop
    while let Some(flow_session_id) = rx.recv().await {
        debug!("[PROCESSOR] Received workflow_id: {}", flow_session_id);

        if !flow_session_id.is_empty() {}
    }

    Ok(())
}

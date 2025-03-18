use crate::AppState;
use std::sync::Arc;

use uuid::Uuid;

use crate::processor::parallelizer::ProcessingContext;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;


use crate::processor::parallelizer::process_workflow;

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
    println!("[PROCESSOR] Starting processor");
    let mut rx = state.processor_receiver.lock().await;

    while let Some(message) = rx.recv().await {
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            println!("[PROCESSOR] Received shutdown signal, stopping processor");
            break;
        }

        println!(
            "[PROCESSOR] Received flow_session_id: {}",
            message.flow_session_id
        );

        let state = Arc::clone(&state);
        let client = state.anything_client.clone();

        // Process workflow sequentially
        process_workflow(state, (*client).clone(), message).await;
    }

    println!("[PROCESSOR] Processor shutdown complete");
    Ok(())
}

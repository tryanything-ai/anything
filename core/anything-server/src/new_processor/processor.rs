use crate::AppState;
use std::sync::Arc;
use tracing::debug;

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("[PROCESSOR] Starting processor");

    // Get signal receiver from state
    let mut processor_signal_rx = state.processor_signal.subscribe();

    loop {
        tokio::select! {
            Ok(_) = processor_signal_rx.changed() => {
                let workflow_id = processor_signal_rx.borrow().clone();
                debug!("[PROCESSOR] Received workflow_id: {}", workflow_id);
                
                // Here you can add your workflow processing logic
                if !workflow_id.is_empty() {
                    // Process the workflow
                }
            }
        }
    }
}
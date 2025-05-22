use crate::processor::parallelizer::process_workflow;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

use tracing::{error, info, instrument, warn, Span};
use uuid::Uuid;

// Added for metrics
use once_cell::sync::Lazy;
use opentelemetry::{
    global as otel_global,
    metrics::{Counter, Histogram, ObservableGauge, Observer, UpDownCounter},
    KeyValue,
};

// Placeholder function - replace with actual logic to get total permits from AppState
// For example, if AppState has a direct field or a config struct.
fn get_total_permits_from_state(state: &Arc<AppState>) -> u64 {
    // You MUST replace this with the correct way to get your configured max_concurrent_workflows
    // e.g., state.config.max_concurrent_workflows as u64 or similar.
    10 // Defaulting to 10 as a placeholder - PLEASE UPDATE
}

#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub workflow_version: DatabaseFlowVersion,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
}

// Metrics Definitions
static METER: Lazy<opentelemetry::metrics::Meter> =
    Lazy::new(|| otel_global::meter("anything_server.processor"));

static MESSAGES_RECEIVED_COUNTER: Lazy<Counter<u64>> = Lazy::new(|| {
    METER
        .u64_counter("processor_messages_received_total")
        .with_description("Total number of messages received by the processor.")
        .init()
});

static ACTIVE_WORKFLOWS_COUNTER: Lazy<UpDownCounter<i64>> = Lazy::new(|| {
    METER
        .i64_up_down_counter("processor_active_workflows")
        .with_description("Number of workflows currently being processed.")
        .init()
});

static WORKFLOW_DURATION_HISTOGRAM: Lazy<Histogram<f64>> = Lazy::new(|| {
    METER
        .f64_histogram("workflow_processing_duration_seconds")
        .with_description("Duration of workflow processing in seconds.")
        .init()
});

static SEMAPHORE_PERMITS_AVAILABLE: Lazy<ObservableGauge<u64>> = Lazy::new(|| {
    METER
        .u64_observable_gauge("semaphore_permits_available")
        .with_description("Number of available semaphore permits for workflow processing.")
        .init()
});

static SEMAPHORE_PERMITS_USED: Lazy<ObservableGauge<u64>> = Lazy::new(|| {
    METER
        .u64_observable_gauge("semaphore_permits_used")
        .with_description("Number of used semaphore permits for workflow processing.")
        .init()
});

// #[instrument(skip(state, processor_receiver))]
pub async fn processor(
    state: Arc<AppState>,
    mut processor_receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("[PROCESSOR] Starting processor");

    // Register observable semaphore metrics callback
    let state_clone_for_callback = state.clone();
    // Register metrics callback and handle errors with if let
    if let Err(err) = METER.register_callback(
        &[
            SEMAPHORE_PERMITS_AVAILABLE.as_any(),
            SEMAPHORE_PERMITS_USED.as_any(),
        ],
        move |observer: &dyn Observer| {
            let permits_available = state_clone_for_callback
                .workflow_processor_semaphore
                .available_permits() as u64;
            observer.observe_u64(&*SEMAPHORE_PERMITS_AVAILABLE, permits_available, &[]);

            let total_permits = get_total_permits_from_state(&state_clone_for_callback); // Use the placeholder
            let permits_used = total_permits.saturating_sub(permits_available);
            observer.observe_u64(&*SEMAPHORE_PERMITS_USED, permits_used, &[]);
        },
    ) {
        error!("Failed to register metrics callback: {}", err);
    }

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
                MESSAGES_RECEIVED_COUNTER.add(1, &[]); // Increment messages received

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
                        ACTIVE_WORKFLOWS_COUNTER.add(1, &[]); // Increment active workflows

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
                            WORKFLOW_DURATION_HISTOGRAM.record(exec_duration.as_secs_f64(), &[]); // Record duration
                            info!("[PROCESSOR] Completed workflow {} and releasing permit (duration: {:?})", flow_session_id, exec_duration);
                            ACTIVE_WORKFLOWS_COUNTER.add(-1, &[]); // Decrement active workflows
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
                        // Note: ACTIVE_WORKFLOWS_COUNTER is not incremented here as the workflow didn't start
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

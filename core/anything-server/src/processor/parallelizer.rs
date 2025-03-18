use crate::AppState;

use std::sync::Arc;
use tokio::sync::Mutex;

use std::collections::HashMap;
use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::processor::flow_session_cache::FlowSessionData;
use crate::processor::processor::ProcessorMessage;
use crate::processor::processor_utils::create_task;
use crate::status_updater::{Operation, StatusUpdateMessage};

use crate::types::{
    task_types::{FlowSessionStatus, TriggerSessionStatus},
    workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
};

use crate::processor::path_processor::spawn_path_processor;

/// Represents the state needed for processing a workflow path
#[derive(Clone)]
pub struct PathProcessingContext {
    pub state: Arc<AppState>,
    pub client: postgrest::Postgrest,
    pub flow_session_id: Uuid,
    pub workflow_id: Uuid,
    pub trigger_task_id: String,
    pub trigger_session_id: Uuid,
    pub workflow: Arc<DatabaseFlowVersion>,
    pub workflow_def: Arc<WorkflowVersionDefinition>,
    pub active_paths: Arc<Mutex<usize>>,
    pub path_semaphore: Arc<Semaphore>,
}

// Constants
const MAX_CONCURRENT_PATHS: usize = 5;

/// Starts processing a workflow with parallel paths
pub async fn start_parallel_workflow_processing(
    state: Arc<AppState>,
    client: postgrest::Postgrest,
    processor_message: ProcessorMessage,
) {
    println!(
        "[PROCESSOR] Starting parallel workflow processing for flow session: {}",
        processor_message.flow_session_id
    );

    // Create a semaphore to limit concurrent paths
    let number_of_parallel_paths_semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_PATHS));
    println!(
        "[PROCESSOR] Created semaphore with {} max concurrent paths",
        MAX_CONCURRENT_PATHS
    );

    // Create a counter to track active path processors
    let active_paths = Arc::new(Mutex::new(0));
    println!("[PROCESSOR] Initialized active paths counter");

    // Clone client before using it in the context
    let client_clone = client.clone();

    // Add session to flow_session_cache
    let flow_session_data = FlowSessionData {
        tasks: HashMap::new(),
    };

    // Set the flow session data in cache
    {
        let mut cache = state.flow_session_cache.write().await;
        cache.set(&processor_message.flow_session_id, flow_session_data);
    }

    // Create the shared context
    let ctx = PathProcessingContext {
        state: state.clone(),
        client: client_clone,
        flow_session_id: processor_message.flow_session_id,
        workflow_id: processor_message.workflow_id,
        trigger_task_id: processor_message
            .trigger_task
            .clone()
            .unwrap()
            .trigger_id
            .clone(),
        trigger_session_id: processor_message.trigger_session_id.clone(),
        workflow: Arc::new(processor_message.workflow_version.clone()),
        workflow_def: Arc::new(processor_message.workflow_version.flow_definition.clone()),
        active_paths: active_paths.clone(),
        path_semaphore: number_of_parallel_paths_semaphore,
    };

    // Check for shutdown signal
    if state
        .shutdown_signal
        .load(std::sync::atomic::Ordering::SeqCst)
    {
        println!("[PROCESSOR] Received shutdown signal, stopping task processing");
        return;
    }

    // If we have an initial task, start processing it in parallel
    if let Some(task) = processor_message.trigger_task {
        println!(
            "[PROCESSOR] Starting initial task processing: {}",
            task.task_id
        );

        // Increment active paths counter
        {
            let mut paths = active_paths.lock().await;
            *paths += 1;
            println!("[PROCESSOR] Incremented active paths to: {}", *paths);
        }

        //Create First Action In Db
        if let Err(e) = create_task(&ctx, &task).await {
            println!("[PROCESSOR] Failed to create first action in db: {}", e);
            return;
        }

        // Spawn the initial task processing
        spawn_path_processor(ctx, task);

        let mut loop_count = 0;
        // Wait for all paths to complete
        loop {
            let paths_count = {
                let paths = active_paths.lock().await;
                *paths
            };

            loop_count += 1;

            println!(
                "[PROCESSOR] Waiting for {} active paths to complete... Loop count: {}",
                paths_count, loop_count
            );

            if paths_count == 0 {
                println!("[PROCESSOR] All paths have completed, workflow is done");
                break;
            }

            // Sleep briefly to avoid busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // If shutdown signal received, log but continue waiting
            if state
                .shutdown_signal
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                println!(
                    "[PROCESSOR] Shutdown signal received, waiting for {} active paths to complete",
                    paths_count
                );
            }
        }
    } else {
        println!("[PROCESSOR] No trigger task to process");
    }

    // This code runs after the loop is broken
    println!(
        "[PROCESSOR] Workflow processing complete: {}",
        processor_message.flow_session_id
    );

    // Update flow session status to completed
    let task_message = StatusUpdateMessage {
        operation: Operation::CompleteWorkflow {
            flow_session_id: processor_message.flow_session_id,
            status: FlowSessionStatus::Completed,
            trigger_status: TriggerSessionStatus::Completed,
        },
    };
    state.task_updater_sender.send(task_message).await.unwrap();
}

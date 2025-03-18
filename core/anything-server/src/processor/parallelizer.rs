use crate::processor::utils::{create_workflow_graph, get_trigger_node};

use crate::AppState;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::processor::processor_utils::{create_task_for_action, drop_path_counter, process_task};

use crate::processor::processor::ProcessorMessage;
use crate::status_updater::{Operation, StatusUpdateMessage};

use crate::types::{
    action_types::ActionType,
    task_types::{FlowSessionStatus, Stage, Task, TaskConfig, TaskStatus, TriggerSessionStatus},
    workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
};

use std::time::Instant;

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
    flow_session_id: Uuid,
    workflow_id: Uuid,
    trigger_task_id: String,
    trigger_session_id: Uuid,
    workflow: DatabaseFlowVersion,
    processor_message: ProcessorMessage,
    cached_tasks: Option<HashMap<Uuid, Task>>,
) {
    let start = Instant::now();
    println!(
        "[PROCESSOR] Starting parallel workflow processing for flow session: {}",
        flow_session_id
    );

    // Create a semaphore to limit concurrent paths
    let path_semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_PATHS));
    println!(
        "[PROCESSOR] Created semaphore with {} max concurrent paths",
        MAX_CONCURRENT_PATHS
    );

    // Create a counter to track active path processors
    let active_paths = Arc::new(Mutex::new(0));
    println!("[PROCESSOR] Initialized active paths counter");

    // Create the workflow graph
    let workflow_def = Arc::new(workflow.flow_definition.clone());

    // Clone workflow before using it in the Arc
    let workflow_clone = workflow.clone();

    // Clone client before using it in the context
    let client_clone = client.clone();

    // Create the shared context
    let ctx = PathProcessingContext {
        state: state.clone(),
        client: client_clone,
        flow_session_id,
        workflow_id,
        trigger_task_id: trigger_task_id.clone(),
        trigger_session_id: trigger_session_id.clone(),
        workflow: Arc::new(workflow_clone),
        workflow_def,
        active_paths: active_paths.clone(),
        path_semaphore,
    };
    println!(
        "[SPEED] Parallelizer::context_setup - {:?}",
        start.elapsed()
    );

    let trigger_task = processor_message.trigger_task.clone();

    // Create initial trigger task
    let trigger_setup_start = Instant::now();
    let trigger_node = get_trigger_node(&workflow.flow_definition).unwrap();

    // If we have a trigger task in the message, use that, otherwise check cache
    let initial_task = if let Some(trigger_task) = trigger_task {
        // Create task from the provided trigger task input
        let task_message = StatusUpdateMessage {
            task_id: trigger_task.task_id,
            operation: Operation::CreateTask {
                input: trigger_task.clone(),
            },
        };

        state.task_updater_sender.send(task_message).await.unwrap();

        // Update cache with new task
        let mut cache = state.flow_session_cache.write().await;
        if cache.add_task(&flow_session_id, trigger_task.clone()) {
            Some(trigger_task)
        } else {
            println!(
                "[PROCESSOR] Failed to add task to cache for flow_session_id: {}",
                flow_session_id
            );
            Some(trigger_task)
        }
    } else if cached_tasks.is_none() || cached_tasks.as_ref().unwrap().is_empty() {
        // Only create trigger task if there are no existing tasks in cache
        let new_task: Option<Task> = match Task::builder()
            .account_id(workflow.account_id.clone())
            .flow_id(workflow_id)
            .flow_version_id(workflow.flow_version_id)
            .action_label(trigger_node.label.clone())
            .trigger_id(trigger_task_id.clone())
            .trigger_session_id(trigger_session_id)
            .flow_session_id(flow_session_id)
            .action_id(trigger_node.action_id.clone())
            .r#type(ActionType::Trigger)
            .plugin_name(trigger_node.plugin_name.clone())
            .plugin_version(trigger_node.plugin_version.clone())
            .stage(if workflow.published {
                Stage::Production
            } else {
                Stage::Testing
            })
            .config(TaskConfig {
                inputs: Some(trigger_node.inputs.clone().unwrap()),
                inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
                plugin_config: Some(trigger_node.plugin_config.clone()),
                plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
            })
            .build()
        {
            Ok(task) => Some(task),
            Err(_e) => None,
        };

        if let Some(new_task) = new_task {
            // Create task from the provided workflow
            let task_message = StatusUpdateMessage {
                task_id: new_task.task_id,
                operation: Operation::CreateTask {
                    input: new_task.clone(),
                },
            };

            state.task_updater_sender.send(task_message).await.unwrap();

            // Update cache with new task
            let mut cache = state.flow_session_cache.write().await;
            if cache.add_task(&flow_session_id, new_task.clone()) {
                Some(new_task)
            } else {
                println!(
                    "[PROCESSOR] Failed to add task to cache for flow_session_id: {}",
                    flow_session_id
                );
                Some(new_task)
            }
        } else {
            println!("[PROCESSOR] Failed to create new task");
            None
        }
    } else {
        // We have existing tasks - find the last incomplete task or the highest processing order
        let existing_tasks = cached_tasks.as_ref().unwrap();

        // First try to find an incomplete task
        let incomplete_tasks = existing_tasks
            .values()
            .filter(|task| {
                task.task_status == TaskStatus::Running
                    || task.task_status == TaskStatus::Pending
                    || task.task_status == TaskStatus::Failed
            })
            .collect::<Vec<_>>();

        // If there are incomplete tasks, resume from them
        if !incomplete_tasks.is_empty() {
            println!(
                "[PROCESSOR] Found {} incomplete tasks to resume",
                incomplete_tasks.len()
            );

            // For each incomplete task, spawn a processing path
            for task in incomplete_tasks {
                println!(
                    "[PROCESSOR] Resuming from incomplete task: {}",
                    task.task_id
                );

                // Increment active paths counter
                {
                    let mut paths = active_paths.lock().await;
                    *paths += 1;
                    println!("[PROCESSOR] Incremented active paths to: {}", *paths);
                }

                // Clone the context for this path
                let path_ctx = PathProcessingContext {
                    state: ctx.state.clone(),
                    client: ctx.client.clone(),
                    flow_session_id,
                    workflow_id,
                    trigger_task_id: ctx.trigger_task_id.clone(),
                    trigger_session_id: ctx.trigger_session_id,
                    workflow: ctx.workflow.clone(),
                    workflow_def: ctx.workflow_def.clone(),
                    active_paths: ctx.active_paths.clone(),
                    path_semaphore: ctx.path_semaphore.clone(),
                };

                // Spawn a processing path for this task
                spawn_path_processor(path_ctx, task.clone());
            }

            // Return None since we've already spawned paths for all incomplete tasks
            None
        } else {
            // All tasks are either completed or there are no tasks
            println!("[PROCESSOR] No incomplete tasks found to resume");
            None
        }
    };
    println!(
        "[SPEED] Parallelizer::trigger_setup - {:?}",
        trigger_setup_start.elapsed()
    );

    // Check for shutdown signal
    if state
        .shutdown_signal
        .load(std::sync::atomic::Ordering::SeqCst)
    {
        println!("[PROCESSOR] Received shutdown signal, stopping task processing");
        return;
    }

    // If we have an initial task, start processing it in parallel
    if let Some(task) = initial_task {
        let process_start = Instant::now();
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
        println!(
            "[SPEED] Parallelizer::task_processing - {:?}",
            process_start.elapsed()
        );
    } else {
        println!("[PROCESSOR] No initial task to process, workflow is complete");
    }

    // This code runs after the loop is broken
    println!(
        "[PROCESSOR] Workflow processing complete: {}",
        flow_session_id
    );

    // Update flow session status to completed
    let task_message = StatusUpdateMessage {
        task_id: flow_session_id,
        operation: Operation::CompleteWorkflow {
            flow_session_id,
            status: FlowSessionStatus::Completed,
            trigger_status: TriggerSessionStatus::Completed,
        },
    };
    state.task_updater_sender.send(task_message).await.unwrap();

    println!(
        "[SPEED] Parallelizer::total_workflow_processing - {:?}",
        start.elapsed()
    );
}

fn spawn_path_processor(ctx: PathProcessingContext, task: Task) {
    let start = Instant::now();
    println!(
        "[PROCESSOR] Entering spawn_path_processor for task: {}",
        task.task_id
    );
    tokio::spawn(async move {
        println!(
            "[PROCESSOR] Starting parallel path for action: {} (task: {})",
            task.action_label, task.task_id
        );

        // Acquire a permit from the semaphore
        let permit_start = Instant::now();
        println!(
            "[PROCESSOR] Attempting to acquire semaphore permit for task: {}",
            task.task_id
        );
        let permit = match ctx.path_semaphore.acquire().await {
            Ok(permit) => {
                println!(
                    "[SPEED] Parallelizer::acquire_permit - {:?}",
                    permit_start.elapsed()
                );
                permit
            }
            Err(e) => {
                println!(
                    "[PROCESSOR] Failed to acquire path permit for task {}: {}",
                    task.task_id, e
                );
                println!(
                    "[SPEED] Parallelizer::acquire_permit_failed - {:?}",
                    permit_start.elapsed()
                );
                // Decrement active paths counter
                {
                    let mut paths = ctx.active_paths.lock().await;
                    *paths -= 1;
                    println!(
                        "[PROCESSOR] Decremented active paths to {} after permit failure",
                        *paths
                    );
                }
                return;
            }
        };

        // Process the path (inline the process_path logic)
        let graph_start = Instant::now();
        println!(
            "[PROCESSOR] Creating workflow graph for task: {}",
            task.task_id
        );
        let graph = create_workflow_graph(&ctx.workflow_def);
        println!(
            "[SPEED] Parallelizer::create_workflow_graph - {:?}",
            graph_start.elapsed()
        );
        let mut current_task = task;

        // Process tasks in this path until completion
        let process_start = Instant::now();
        println!("[PROCESSOR] Starting task processing loop for path");
        loop {
            let task_start = Instant::now();
            println!(
                "[PROCESSOR] Processing task: {} in loop",
                current_task.task_id
            );
            // Process the current task
            let next_actions = match process_task(&ctx, &current_task, &graph).await {
                Ok(actions) => {
                    println!(
                        "[SPEED] Parallelizer::process_task - {:?}",
                        task_start.elapsed()
                    );
                    println!(
                        "[PROCESSOR] Found {} next actions for task {}",
                        actions.len(),
                        current_task.task_id
                    );
                    actions
                }
                Err(_e) => {
                    println!(
                        "[SPEED] Parallelizer::process_task_error - {:?}",
                        task_start.elapsed()
                    );
                    println!(
                        "[PROCESSOR] Task {} failed, marking path as failed",
                        current_task.task_id
                    );
                    // Task failed, mark path as failed and exit
                    // check_and_update_workflow_completion(&ctx, false).await;
                    drop_path_counter(&ctx).await;
                    drop(permit);
                    return;
                }
            };
            println!(
                "[SPEED] Parallelizer::process_task - {:?}",
                process_start.elapsed()
            );

            // If we have multiple next actions, spawn new paths for all but the first
            if next_actions.len() > 1 {
                println!(
                    "[PROCESSOR] Multiple next actions found ({}), spawning new paths",
                    next_actions.len()
                );
                // Increment active paths counter for the additional paths
                {
                    let mut paths = ctx.active_paths.lock().await;
                    *paths += next_actions.len() - 1; // -1 because we'll process one in this path
                    println!(
                        "[PROCESSOR] Incremented active paths to {} for parallel processing",
                        *paths
                    );
                }

                // Process all but the first action in new paths
                for (idx, next_action) in next_actions.iter().skip(1).enumerate() {
                    println!(
                        "[PROCESSOR] Creating task for parallel path {} of {}",
                        idx + 1,
                        next_actions.len() - 1
                    );
                    // Create a new task for this action
                    match create_task_for_action(
                        &ctx,
                        next_action,
                        current_task.processing_order + 1,
                    )
                    .await
                    {
                        Ok(new_task) => {
                            println!(
                                "[PROCESSOR] Successfully created task {} for parallel path",
                                new_task.task_id
                            );
                            // Clone the context for the new path
                            let new_ctx = PathProcessingContext {
                                state: ctx.state.clone(),
                                client: ctx.client.clone(),
                                flow_session_id: ctx.flow_session_id,
                                workflow_id: ctx.workflow_id,
                                trigger_task_id: ctx.trigger_task_id.clone(),
                                trigger_session_id: ctx.trigger_session_id,
                                workflow: ctx.workflow.clone(),
                                workflow_def: ctx.workflow_def.clone(),
                                active_paths: ctx.active_paths.clone(),
                                path_semaphore: ctx.path_semaphore.clone(),
                            };

                            println!(
                                "[PROCESSOR] Spawning new process path for task: {}",
                                new_task.task_id
                            );
                            spawn_path_processor(new_ctx, new_task);
                        }
                        Err(e) => {
                            println!("[PROCESSOR] Error creating task for parallel path: {}", e);

                            // Decrement active paths counter for this failed path
                            {
                                let mut paths = ctx.active_paths.lock().await;
                                *paths -= 1;
                                println!("[PROCESSOR] Decremented active paths to {} after task creation failure", *paths);
                            }
                        }
                    }
                }

                // Continue with the first next action in this path
                println!("[PROCESSOR] Continuing with first action in current path");
                if let Some(first_action) = next_actions.first() {
                    match create_task_for_action(
                        &ctx,
                        first_action,
                        current_task.processing_order + 1,
                    )
                    .await
                    {
                        Ok(new_task) => {
                            println!(
                                "[PROCESSOR] Created next task {} in current path",
                                new_task.task_id
                            );
                            current_task = new_task;
                        }
                        Err(e) => {
                            println!(
                                "[PROCESSOR] Error creating next task in current path: {}",
                                e
                            );

                            // Path is complete with error
                            //TODO: figure how to handle this better.
                            // check_and_update_workflow_completion(&ctx, false).await;
                            drop_path_counter(&ctx).await;
                            drop(permit);
                            return;
                        }
                    }
                } else {
                    println!("[PROCESSOR] No first action found (unexpected), breaking loop");
                    // No next action (shouldn't happen, but handle it)
                    break;
                }
            } else if next_actions.len() == 1 {
                println!("[PROCESSOR] Single next action found, continuing in current path");
                // Just one next action, continue in this path
                match create_task_for_action(
                    &ctx,
                    &next_actions[0],
                    current_task.processing_order + 1,
                )
                .await
                {
                    Ok(new_task) => {
                        println!(
                            "[PROCESSOR] Created next task {} in current path",
                            new_task.task_id
                        );
                        current_task = new_task;
                    }
                    Err(e) => {
                        println!(
                            "[PROCESSOR] Error creating next task in current path: {}",
                            e
                        );

                        // Path is complete with error
                        // check_and_update_workflow_completion(&ctx, false).await;
                        drop_path_counter(&ctx).await;
                        drop(permit);
                        return;
                    }
                }
            } else {
                // No more actions in this path
                println!("[PROCESSOR] No more actions in path, completing successfully");
                break;
            }
        }

        // Path completed successfully
        println!("[PROCESSOR] Path completed successfully, updating workflow completion status");
        // check_and_update_workflow_completion(&ctx, true).await;
        drop_path_counter(&ctx).await;

        // Release the semaphore permit
        println!("[PROCESSOR] Releasing semaphore permit");
        drop(permit);
        println!(
            "[SPEED] Parallelizer::release_permit - {:?}",
            start.elapsed()
        );
    });
    println!(
        "[SPEED] Parallelizer::path_processing - {:?}",
        start.elapsed()
    );
}

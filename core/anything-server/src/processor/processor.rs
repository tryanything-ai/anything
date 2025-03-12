use crate::processor::utils::{
    create_workflow_graph, get_trigger_node, get_workflow_and_tasks_from_cache,
    is_already_processing,
};
use crate::AppState;
use chrono::Utc;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::processor::processor_utils::{
    check_and_update_workflow_completion, create_task_for_action, process_task, update_flow_status,
};

use crate::processor::db_calls::{create_task, update_flow_session_status, update_task_status};

use crate::types::{
    action_types::ActionType,
    task_types::{
        CreateTaskInput, FlowSessionStatus, Stage, Task, TaskConfig, TaskStatus,
        TriggerSessionStatus,
    },
    workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
};

// Add this near your other type definitions
#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub version_id: Option<Uuid>,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<CreateTaskInput>,
    pub subflow_depth: usize,
}

// Constants
const MAX_CONCURRENT_PATHS: usize = 5;

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
    pub subflow_depth: usize,
}

/// Processes a single path in the workflow
async fn process_path(ctx: PathProcessingContext, initial_task: Task) {
    println!(
        "[PROCESSOR] Starting process_path for task: {} (action: {})",
        initial_task.task_id, initial_task.action_label
    );

    // Acquire a permit from the semaphore before processing this path
    let permit = match ctx.path_semaphore.acquire().await {
        Ok(permit) => {
            println!("[PROCESSOR] Successfully acquired semaphore permit for path");
            permit
        }
        Err(e) => {
            println!("[PROCESSOR] Failed to acquire path permit: {}", e);

            // Decrement active paths counter since we won't process this path
            check_and_update_workflow_completion(&ctx, true).await;
            return;
        }
    };

    println!(
        "[PROCESSOR] Starting parallel path for action: {}",
        initial_task.action_label
    );

    // Create the workflow graph for this path
    println!("[PROCESSOR] Creating workflow graph");
    let graph = create_workflow_graph(&ctx.workflow_def);

    // Start with the initial task
    let mut current_task = initial_task;

    // Process tasks in this path until completion
    println!("[PROCESSOR] Beginning task processing loop");
    loop {
        // Process the current task
        let next_actions = match process_task(&ctx, &current_task, &graph).await {
            Ok(actions) => {
                println!(
                    "[PROCESSOR] Found {} next actions for task {}",
                    actions.len(),
                    current_task.task_id
                );
                if actions.is_empty() {
                    println!("[PROCESSOR] No next actions returned (filter stopped), ending path");
                    break;
                }
                actions
            }
            Err(e) => {
                println!(
                    "[PROCESSOR] Task {} failed, marking path as failed",
                    current_task.task_id
                );
                // Task failed, mark path as failed and exit
                check_and_update_workflow_completion(&ctx, false).await;
                drop(permit);
                return;
            }
        };

        // If we have multiple next actions, spawn new paths for all but the first
        if next_actions.len() > 1 {
            println!(
                "[PROCESSOR] Found multiple next actions ({}), spawning new paths",
                next_actions.len()
            );
            // Increment active paths counter for the additional paths
            {
                let mut paths = ctx.active_paths.lock().await;
                *paths += next_actions.len() - 1; // -1 because we'll process one in this path
                println!("[PROCESSOR] Incremented active paths to: {}", *paths);
            }

            // Process all but the first action in new paths
            for next_action in next_actions.iter().skip(1) {
                println!(
                    "[PROCESSOR] Creating task for parallel action: {}",
                    next_action.label
                );
                // Create a new task for this action
                match create_task_for_action(&ctx, next_action, current_task.processing_order + 1)
                    .await
                {
                    Ok(new_task) => {
                        println!(
                            "[PROCESSOR] Successfully created task {} for parallel action",
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
                            subflow_depth: ctx.subflow_depth,
                        };

                        // Spawn a new task for this path
                        println!(
                            "[PROCESSOR] Spawning new process path for task: {}",
                            new_task.task_id
                        );
                        spawn_process_path(new_ctx, new_task);
                    }
                    Err(e) => {
                        println!("[PROCESSOR] Error creating task for parallel path: {}", e);

                        // Decrement active paths counter for this failed path
                        {
                            let mut paths = ctx.active_paths.lock().await;
                            *paths -= 1;
                            println!("[PROCESSOR] Decremented active paths to: {}", *paths);
                        }
                    }
                }
            }

            // Continue with the first next action in this path
            if let Some(first_action) = next_actions.first() {
                println!(
                    "[PROCESSOR] Creating task for first action in current path: {}",
                    first_action.label
                );
                match create_task_for_action(&ctx, first_action, current_task.processing_order + 1)
                    .await
                {
                    Ok(new_task) => {
                        println!(
                            "[PROCESSOR] Successfully created task {} for first action",
                            new_task.task_id
                        );
                        current_task = new_task;
                    }
                    Err(e) => {
                        println!("[PROCESSOR] Error creating next task: {}", e);

                        // Path is complete with error
                        check_and_update_workflow_completion(&ctx, false).await;
                        drop(permit);
                        return;
                    }
                }
            } else {
                println!("[PROCESSOR] No first action found (shouldn't happen), breaking");
                // No next action (shouldn't happen, but handle it)
                break;
            }
        } else if next_actions.len() == 1 {
            println!("[PROCESSOR] Found single next action, continuing in current path");
            // Just one next action, continue in this path
            match create_task_for_action(&ctx, &next_actions[0], current_task.processing_order + 1)
                .await
            {
                Ok(new_task) => {
                    println!(
                        "[PROCESSOR] Successfully created next task: {}",
                        new_task.task_id
                    );
                    current_task = new_task;
                }
                Err(e) => {
                    println!("[PROCESSOR] Error creating next task: {}", e);

                    // Path is complete with error
                    check_and_update_workflow_completion(&ctx, false).await;
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
    println!("[PROCESSOR] Path completed successfully, updating workflow completion");
    check_and_update_workflow_completion(&ctx, true).await;

    // Release the semaphore permit
    println!("[PROCESSOR] Releasing semaphore permit");
    drop(permit);
}

/// Starts processing a workflow with parallel paths
async fn start_parallel_workflow_processing(
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
    let graph = create_workflow_graph(&workflow_def);
    println!("[PROCESSOR] Created workflow graph");

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
        subflow_depth: processor_message.subflow_depth,
    };
    println!("[PROCESSOR] Created shared processing context");

    // let workflow_id = processor_message.workflow_id;
    // let version_id = processor_message.version_id;
    // let flow_session_id = processor_message.flow_session_id;
    let trigger_task = processor_message.trigger_task.clone();
    // let trigger_task_id = trigger_task.clone().unwrap().trigger_id;
    // let trigger_session_id = processor_message.trigger_session_id;

    // Create initial trigger task
    let trigger_node = get_trigger_node(&workflow.flow_definition).unwrap();

    // If there are no tasks in cache, we need to create the trigger task
    let initial_task = if cached_tasks.is_none() || cached_tasks.as_ref().unwrap().is_empty() {
        // Only create trigger task if there are no existing tasks in cache
        let initial_task = if let Some(trigger_task) = trigger_task {
            trigger_task
        } else {
            CreateTaskInput {
                account_id: workflow.account_id.to_string(),
                processing_order: 0,
                task_status: TaskStatus::Running.as_str().to_string(),
                flow_id: workflow_id.to_string(),
                flow_version_id: workflow.flow_version_id.to_string(),
                action_label: trigger_node.label.clone(),
                trigger_id: trigger_task_id.clone(),
                trigger_session_id: trigger_session_id.to_string(),
                trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
                flow_session_id: flow_session_id.to_string(),
                flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
                action_id: trigger_node.action_id.clone(),
                r#type: ActionType::Trigger,
                plugin_name: trigger_node.plugin_name.clone(),
                plugin_version: trigger_node.plugin_version.clone(),
                stage: if workflow.published {
                    Stage::Production.as_str().to_string()
                } else {
                    Stage::Testing.as_str().to_string()
                },
                config: TaskConfig {
                    inputs: Some(trigger_node.inputs.clone().unwrap()),
                    inputs_schema: Some(trigger_node.inputs_schema.clone().unwrap()),
                    plugin_config: Some(trigger_node.plugin_config.clone()),
                    plugin_config_schema: Some(trigger_node.plugin_config_schema.clone()),
                },
                result: None,
                error: None,
                started_at: Some(Utc::now()),
                test_config: None,
            }
        };

        // Start with trigger task
        match create_task(state.clone(), &initial_task).await {
            Ok(task) => {
                // Update cache with new task
                let mut cache = state.flow_session_cache.write().await;
                if cache.add_task(&flow_session_id, task.clone()) {
                    Some(task)
                } else {
                    println!(
                        "[PROCESSOR] Failed to add task to cache for flow_session_id: {}",
                        flow_session_id
                    );
                    Some(task)
                }
            }
            Err(e) => {
                println!("[PROCESSOR] Error creating initial task: {}", e);
                None
            }
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
                    subflow_depth: ctx.subflow_depth,
                };

                // Spawn a processing path for this task
                spawn_process_path(path_ctx, task.clone());
            }

            // Return None since we've already spawned paths for all incomplete tasks
            None
        } else {
            // All tasks are either completed or there are no tasks
            println!("[PROCESSOR] No incomplete tasks found to resume");
            None
        }
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
    if let Some(task) = initial_task {
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
        spawn_process_path(ctx, task);

        // Wait for all paths to complete
        loop {
            // Check if all paths have completed
            let paths_count = {
                let paths = active_paths.lock().await;
                *paths
            };

            if paths_count == 0 {
                println!("[PROCESSOR] All paths have completed, workflow is done");
                break;
            }

            // Sleep briefly to avoid busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Check for shutdown signal
            if state
                .shutdown_signal
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                println!("[PROCESSOR] Received shutdown signal during path completion wait");
                break;
            }
        }
    } else {
        println!("[PROCESSOR] No initial task to process, workflow is complete");

        // Update flow session status to completed
        update_flow_status(
            &ctx,
            &FlowSessionStatus::Completed,
            &TriggerSessionStatus::Completed,
        )
        .await;
    }

    println!(
        "[PROCESSOR] Workflow processing complete: {}",
        flow_session_id
    );
}

fn spawn_process_path(ctx: PathProcessingContext, task: Task) {
    println!(
        "[PROCESSOR] Entering spawn_process_path for task: {}",
        task.task_id
    );
    tokio::spawn(async move {
        println!(
            "[PROCESSOR] Starting parallel path for action: {} (task: {})",
            task.action_label, task.task_id
        );

        // Acquire a permit from the semaphore
        println!(
            "[PROCESSOR] Attempting to acquire semaphore permit for task: {}",
            task.task_id
        );
        let permit = match ctx.path_semaphore.acquire().await {
            Ok(permit) => {
                println!(
                    "[PROCESSOR] Successfully acquired permit for task: {}",
                    task.task_id
                );
                permit
            }
            Err(e) => {
                println!(
                    "[PROCESSOR] Failed to acquire path permit for task {}: {}",
                    task.task_id, e
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
        println!(
            "[PROCESSOR] Creating workflow graph for task: {}",
            task.task_id
        );
        let graph = create_workflow_graph(&ctx.workflow_def);
        let mut current_task = task;

        // Process tasks in this path until completion
        println!("[PROCESSOR] Starting task processing loop for path");
        loop {
            println!(
                "[PROCESSOR] Processing task: {} in loop",
                current_task.task_id
            );
            // Process the current task
            let next_actions = match process_task(&ctx, &current_task, &graph).await {
                Ok(actions) => {
                    println!(
                        "[PROCESSOR] Found {} next actions for task {}",
                        actions.len(),
                        current_task.task_id
                    );
                    if actions.is_empty() {
                        println!(
                            "[PROCESSOR] No next actions returned (filter stopped), ending path"
                        );
                        break;
                    }
                    actions
                }
                Err(e) => {
                    println!(
                        "[PROCESSOR] Task {} failed, marking path as failed",
                        current_task.task_id
                    );
                    // Task failed, mark path as failed and exit
                    check_and_update_workflow_completion(&ctx, false).await;
                    drop(permit);
                    return;
                }
            };

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
                                subflow_depth: ctx.subflow_depth,
                            };

                            println!(
                                "[PROCESSOR] Spawning new process path for task: {}",
                                new_task.task_id
                            );
                            spawn_process_path(new_ctx, new_task);
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
                            check_and_update_workflow_completion(&ctx, false).await;
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
                        check_and_update_workflow_completion(&ctx, false).await;
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
        check_and_update_workflow_completion(&ctx, true).await;

        // Release the semaphore permit
        println!("[PROCESSOR] Releasing semaphore permit");
        drop(permit);
    });
}

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESSOR] Starting processor");

    // Create a shared set to track active flow sessions
    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));
    // Get the receiver from the state
    let mut rx = state.processor_receiver.lock().await;
    // Guard against too many workflows running at once
    let number_of_processors_semaphore = state.workflow_processor_semaphore.clone();

    while let Some(message) = rx.recv().await {
        // Check if we received shutdown signal
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            println!("[PROCESSOR] Received shutdown signal, stopping processor");
            break;
        }

        let workflow_id = message.workflow_id;
        let version_id = message.version_id;
        let flow_session_id = message.flow_session_id;
        let trigger_task = message.trigger_task.clone();
        let trigger_task_id = trigger_task.clone().unwrap().trigger_id;
        let trigger_session_id = message.trigger_session_id;

        println!("[PROCESSOR] Received workflow_id: {}", flow_session_id);

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
        tokio::spawn(async move {
            println!(
                "[PROCESSOR] Starting workflow processing for {}",
                flow_session_id
            );

            // Get workflow definition and cached tasks
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
    }

    Ok(())
}

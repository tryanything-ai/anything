use crate::processor::execute_task::execute_task;
use crate::processor::flow_session_cache::FlowSessionData;
use crate::processor::parsing_utils::get_trigger_node;
use crate::AppState;
use chrono::Utc;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;

use crate::processor::db_calls::{
    create_task, get_workflow_definition, update_flow_session_status, update_task_status,
};
use crate::types::{
    action_types::ActionType,
    task_types::{
        CreateTaskInput, FlowSessionStatus, Stage, TaskConfig, TaskStatus, TriggerSessionStatus,
    },
    workflow_types::WorkflowVersionDefinition,
};

// Add this near your other type definitions
#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub version_id: Option<Uuid>,
    pub flow_session_id: Uuid,
    pub trigger_task: Option<CreateTaskInput>,
}

pub async fn processor(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESSOR] Starting processor");

    // Create a shared set to track active flow sessions
    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));
    // Get the receiver from the state
    let mut rx = state.processor_receiver.lock().await;
    // Guard againts too many workflows running at once
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
        let trigger_task = message.trigger_task;

        println!("[PROCESSOR] Received workflow_id: {}", flow_session_id);

        // Check if this flow session is already being processed
        {
            // Use a scope block to automatically drop the lock when done
            let mut active_sessions = active_flow_sessions.lock().await;
            if !active_sessions.insert(flow_session_id) {
                println!(
                    "[PROCESSOR] Flow session {} is already being processed, skipping",
                    flow_session_id
                );
                continue;
            }
            println!(
                "[PROCESSOR] Added flow session {} to active sessions",
                flow_session_id
            );
            // Lock is automatically dropped here at end of scope
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
        //SPAWN NEW PROCESSOR FOR EACH WORKFLOW
        tokio::spawn(async move {
            println!(
                "[PROCESSOR] Starting workflow processing for {}",
                flow_session_id
            );

            let mut workflow_definition = None;
            let mut cached_tasks = None;

            // Try to get from cache first using a read lock
            {
                let cache = state.flow_session_cache.read().await;
                println!(
                    "[PROCESSOR] Checking cache for flow_session_id: {}",
                    flow_session_id
                );
                if let Some(session_data) = cache.get(&flow_session_id) {
                    if let Some(workflow) = &session_data.workflow {
                        println!(
                            "[PROCESSOR] Found workflow in cache for flow_session_id: {}",
                            flow_session_id
                        );
                        workflow_definition = Some(workflow.clone());
                    }
                    //When we hydrate old tasks this will have items init from hydrate_processor
                    cached_tasks = Some(session_data.tasks);
                }
            }

            // Only fetch flow definition from DB if we didn't find it in cache
            if workflow_definition.is_none() {
                println!(
                "[PROCESSOR] No workflow found in cache, fetching from DB for flow_session_id: {}",
                flow_session_id
            );

                let workflow =
                    match get_workflow_definition(state.clone(), &workflow_id, version_id.as_ref())
                        .await
                    {
                        Ok(w) => {
                            println!("[PROCESSOR] Successfully fetched workflow from DB");
                            w
                        }
                        Err(e) => {
                            println!("[PROCESSOR] Error getting workflow definition: {}", e);
                            return;
                        }
                    };

                // Only update cache if there isn't already data there
                //TODO: this feels like it could be wrong. In what situation is do we need to fetch worfklow but also no session in cache yet?
                {
                    let mut cache = state.flow_session_cache.write().await;
                    if cache.get(&flow_session_id).is_none() {
                        println!("[PROCESSOR] Creating new session data in cache");
                        let session_data = FlowSessionData {
                            workflow: Some(workflow.clone()),
                            tasks: HashMap::new(),
                            flow_session_id,
                            workflow_id,
                            workflow_version_id: version_id,
                        };
                        cache.set(&flow_session_id, session_data);
                    }
                }

                workflow_definition = Some(workflow);
            }

            println!(
                "[PROCESSOR] Workflow definition status: {:?}",
                workflow_definition.is_some()
            );

            let workflow = match &workflow_definition {
                Some(w) => w,
                None => {
                    println!("[PROCESSOR] No workflow definition found");
                    //This should never happen
                    return;
                }
            };

            println!("[PROCESSOR] Starting workflow execution");

            // Create initial trigger task
            let trigger_node = get_trigger_node(&workflow.flow_definition).unwrap();

            //If there are no tasks in cache, we need to create the trigger task
            let mut current_task = if cached_tasks.is_none()
                || cached_tasks.as_ref().unwrap().is_empty()
            {
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
                        trigger_id: trigger_node.action_id.clone(),
                        trigger_session_id: Uuid::new_v4().to_string(),
                        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
                        flow_session_id: flow_session_id.to_string(),
                        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
                        action_id: trigger_node.action_id.clone(),
                        r#type: ActionType::Trigger,
                        plugin_id: trigger_node.plugin_id.clone(),
                        stage: if workflow.published {
                            Stage::Production.as_str().to_string()
                        } else {
                            Stage::Testing.as_str().to_string()
                        },
                        config: TaskConfig {
                            variables: Some(trigger_node.variables.clone().unwrap()),
                            variables_schema: Some(trigger_node.variables_schema.clone().unwrap()),
                            input: Some(trigger_node.input.clone()),
                            input_schema: Some(trigger_node.input_schema.clone()),
                        },
                        result: None,
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
                let incomplete_task = existing_tasks.values().find(|task| {
                    task.task_status == TaskStatus::Running
                        || task.task_status == TaskStatus::Pending
                });

                if let Some(task) = incomplete_task {
                    println!(
                        "[PROCESSOR] Resuming from incomplete task: {}",
                        task.task_id
                    );

                    //WE have an incomplete task - process it
                    Some(task.clone())
                } else {
                    // If no incomplete task, get the task with highest processing order
                    let last_completed_task = existing_tasks
                        .values()
                        .max_by_key(|task| task.processing_order);

                    if let Some(task) = last_completed_task {
                        println!(
                            "[PROCESSOR] All existing tasks completed, finding next task after: {}",
                            task.task_id
                        );
                        // Find the next action after this completed task using the graph
                        let graph = create_workflow_graph(&workflow.flow_definition);
                        if let Some(neighbors) = graph.get(&task.action_id) {
                            for neighbor_id in neighbors {
                                let next_action = workflow
                                    .flow_definition
                                    .actions
                                    .iter()
                                    .find(|action| &action.action_id == neighbor_id);

                                //We found the next action to run in graph. lets make a task for it
                                if let Some(action) = next_action {
                                    // Create the next task
                                    let next_task_input = CreateTaskInput {
                                        account_id: workflow.account_id.to_string(),
                                        processing_order: task.processing_order + 1,
                                        task_status: TaskStatus::Running.as_str().to_string(),
                                        flow_id: workflow_id.to_string(),
                                        flow_version_id: workflow.flow_version_id.to_string(),
                                        action_label: action.label.clone(),
                                        trigger_id: action.action_id.clone(),
                                        trigger_session_id: Uuid::new_v4().to_string(),
                                        trigger_session_status: TriggerSessionStatus::Running
                                            .as_str()
                                            .to_string(),
                                        flow_session_id: flow_session_id.to_string(),
                                        flow_session_status: FlowSessionStatus::Running
                                            .as_str()
                                            .to_string(),
                                        action_id: action.action_id.clone(),
                                        r#type: action.r#type.clone(),
                                        plugin_id: action.plugin_id.clone(),
                                        stage: if workflow.published {
                                            Stage::Production.as_str().to_string()
                                        } else {
                                            Stage::Testing.as_str().to_string()
                                        },
                                        config: TaskConfig {
                                            variables: Some(action.variables.clone().unwrap()),
                                            variables_schema: Some(action.variables_schema.clone().unwrap()),
                                            input: Some(action.input.clone()),
                                            input_schema: Some(action.input_schema.clone()),
                                        },
                                        result: None,
                                        started_at: Some(Utc::now()),
                                        test_config: None,
                                    };

                                    match create_task(state.clone(), &next_task_input).await {
                                        Ok(new_task) => {
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
                                        }
                                        Err(e) => {
                                            println!("[PROCESSOR] Error creating next task: {}", e);
                                            None
                                        }
                                    };
                                }
                            }
                        }
                        None // No next task found
                    } else {
                        println!("[PROCESSOR] No existing tasks found in cache");
                        None
                    }
                }
            };

            // Create graph for BFS traversal
            let workflow_def: WorkflowVersionDefinition = workflow.flow_definition.clone();

            let graph = create_workflow_graph(&workflow_def);

            // Process tasks until workflow completion or shutdown
            while let Some(task) = current_task {
                // Check for shutdown signal after creating new task
                if state
                    .shutdown_signal
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    println!("[PROCESSOR] Received shutdown signal, stopping task processing");
                    break;
                }

                // Execute the current task and handle its result
                println!("[PROCESSOR] Executing task: {}", task.task_id);

                let processing_order = task.processing_order;

                let (task_result, bundled_context) =
                    match execute_task(state.clone(), &client, &task).await {
                        Ok(success_value) => {
                            println!("[PROCESSOR] Task {} completed successfully", task.task_id);
                            success_value
                        }
                        Err(error) => {
                            println!("[PROCESSOR] Task {} failed: {:?}", task.task_id, error);

                            // Update task status to failed
                            let state_clone = state.clone();
                            let task_id = task.task_id.clone();
                            let error_clone = error.clone();
                            tokio::spawn(async move {
                                if let Err(e) = update_task_status(
                                    state_clone,
                                    &task_id,
                                    &TaskStatus::Failed,
                                    Some(error_clone.context),
                                    None,
                                    Some(error_clone.error),
                                )
                                .await
                                {
                                    println!("[PROCESSOR] Failed to update task status: {}", e);
                                }
                            });

                            // Update flow session status to failed
                            let state_clone = state.clone();
                            let flow_session_id_clone = flow_session_id.clone();
                            tokio::spawn(async move {
                                if let Err(e) = update_flow_session_status(
                                    &state_clone,
                                    &flow_session_id_clone,
                                    &FlowSessionStatus::Failed,
                                    &TriggerSessionStatus::Failed,
                                )
                                .await
                                {
                                    println!(
                                        "[PROCESSOR] Failed to update flow session status: {}",
                                        e
                                    );
                                }
                            });

                            // Update cache
                            {
                                let mut cache = state.flow_session_cache.write().await;
                                let mut task_copy = task.clone();
                                task_copy.result = Some(error.error.clone());
                                task_copy.context = Some(error.context.clone());
                                task_copy.task_status = TaskStatus::Failed;
                                task_copy.ended_at = Some(Utc::now());
                                let _ = cache.update_task(&flow_session_id, task_copy);
                            }

                            println!("[PROCESSOR] Workflow failed: {}", flow_session_id);

                            // Send error response to webhook if needed
                            let mut completions = state.flow_completions.lock().await;
                            if let Some(completion) =
                                completions.remove(&flow_session_id.to_string())
                            {
                                if completion.needs_response {
                                    println!(
                                    "[PROCESSOR] Sending error response through completion channel"
                                );
                                    let _ = completion.sender.send(error.error.clone());
                                }
                            }
                            break; // Exit the while loop
                        }
                    };

                // Spawn task status update to DB asynchronously
                let state_clone = state.clone();
                let task_id = task.task_id.clone();
                let task_result_clone = task_result.clone();
                let bundled_context_clone = bundled_context.clone();
                tokio::spawn(async move {
                    if let Err(e) = update_task_status(
                        state_clone,
                        &task_id,
                        &TaskStatus::Completed,
                        Some(bundled_context_clone),
                        task_result_clone.clone(),
                        None,
                    )
                    .await
                    {
                        println!("[PROCESSOR] Failed to update task status: {}", e);
                    }
                });

                //Update cache with result the same we do the db. these need to match!
                {
                    let mut cache = state.flow_session_cache.write().await;
                    let mut task_copy = task.clone();
                    task_copy.result = task_result;
                    task_copy.context = Some(bundled_context);
                    task_copy.task_status = TaskStatus::Completed;
                    task_copy.ended_at = Some(Utc::now());
                    let _ = cache.update_task(&flow_session_id, task_copy);
                }

                let next_action = if let Some(neighbors) = graph.get(&task.action_id) {
                    let mut next_action = None;
                    // Get the first unprocessed neighbor
                    //TODO: this is where we would handle if we have multiple paths to take and can parallelize
                    for neighbor_id in neighbors {
                        let neighbor = workflow_def
                            .actions
                            .iter()
                            .find(|action| &action.action_id == neighbor_id);

                        if let Some(action) = neighbor {
                            // Check if this task has already been processed
                            let cache = state.flow_session_cache.read().await;
                            if let Some(session_data) = cache.get(&flow_session_id) {
                                if !session_data
                                    .tasks
                                    .iter()
                                    .any(|(_, t)| t.action_id == action.action_id)
                                {
                                    next_action = Some(action.clone());
                                    break;
                                }
                            }
                        }
                    }
                    next_action
                } else {
                    None
                };

                // Create next task if available
                current_task = if let Some(next_action) = next_action {
                    let next_task_input = CreateTaskInput {
                        account_id: workflow.account_id.to_string(),
                        processing_order: processing_order + 1,
                        task_status: TaskStatus::Running.as_str().to_string(), //we create tasks when we start them
                        flow_id: workflow_id.to_string(),
                        flow_version_id: workflow.flow_version_id.to_string(),
                        action_label: next_action.label.clone(),
                        trigger_id: next_action.action_id.clone(),
                        trigger_session_id: Uuid::new_v4().to_string(),
                        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
                        flow_session_id: flow_session_id.to_string(),
                        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
                        action_id: next_action.action_id,
                        r#type: next_action.r#type,
                        plugin_id: next_action.plugin_id.clone(),
                        stage: if workflow.published {
                            Stage::Production.as_str().to_string()
                        } else {
                            Stage::Testing.as_str().to_string()
                        },
                        config: TaskConfig {
                            variables: Some(next_action.variables.clone().unwrap()),
                            variables_schema: Some(next_action.variables_schema.clone().unwrap()),
                            input: Some(next_action.input.clone()),
                            input_schema: Some(next_action.input_schema.clone()),
                        },
                        result: None,
                        test_config: None,
                        started_at: Some(Utc::now()),
                    };

                    match create_task(state.clone(), &next_task_input).await {
                        Ok(new_task) => {
                            // Update cache
                            {
                                let mut cache = state.flow_session_cache.write().await;
                                if let Some(mut session_data) = cache.get(&flow_session_id) {
                                    session_data
                                        .tasks
                                        .insert(new_task.task_id.clone(), new_task.clone());
                                    cache.set(&flow_session_id, session_data);
                                }
                            } // Lock is dropped here
                              // processing_order += 1;
                            Some(new_task)
                        }
                        Err(e) => {
                            println!("[PROCESSOR] Error creating next task: {}", e);
                            None
                        }
                    }
                } else {
                    // No more tasks - workflow is complete
                    let state_clone = state.clone();
                    let flow_session_id_clone = flow_session_id.clone();
                    tokio::spawn(async move {
                        if let Err(e) = update_flow_session_status(
                            &state_clone,
                            &flow_session_id_clone,
                            &FlowSessionStatus::Completed,
                            &TriggerSessionStatus::Completed,
                        )
                        .await
                        {
                            println!("[PROCESSOR] Failed to update flow session status: {}", e);
                        }
                    });

                    println!("[PROCESSOR] Workflow completed: {}", flow_session_id);
                    None
                };
            }

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

            // // Remove the flow session from active sessions when done
            active_flow_sessions.lock().await.remove(&flow_session_id);
            drop(permit);
        });
        //END SPAWNED PROCESSOR
    }

    Ok(())
}

/// Creates a graph representation of the workflow
pub fn create_workflow_graph(
    workflow_def: &WorkflowVersionDefinition,
) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &workflow_def.edges {
        graph
            .entry(edge.source.clone())
            .or_insert_with(Vec::new)
            .push(edge.target.clone());
    }
    graph
}

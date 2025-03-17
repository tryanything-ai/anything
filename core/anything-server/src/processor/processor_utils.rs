use crate::processor::execute_task::execute_task;

use chrono::Utc;
use serde_json::Value;

use std::collections::HashMap;

use crate::processor::execute_task::TaskError;

use crate::processor::db_calls::{create_task, update_flow_session_status, update_task_status};

use crate::processor::parallelizer::PathProcessingContext;

use crate::types::{
    action_types::Action,
    task_types::{
        CreateTaskInput, FlowSessionStatus, Stage, Task, TaskConfig, TaskStatus,
        TriggerSessionStatus,
    },
};

/// Creates a task for the given action
pub async fn create_task_for_action(
    ctx: &PathProcessingContext,
    action: &Action,
    processing_order: i32,
) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR] Creating new task for action: {} (order: {})",
        action.label, processing_order
    );

    let next_task_input = CreateTaskInput {
        account_id: ctx.workflow.account_id.to_string(),
        processing_order,
        task_status: TaskStatus::Running.as_str().to_string(),
        flow_id: ctx.workflow_id.to_string(),
        flow_version_id: ctx.workflow.flow_version_id.to_string(),
        action_label: action.label.clone(),
        trigger_id: ctx.trigger_task_id.clone(),
        trigger_session_id: ctx.trigger_session_id.to_string(),
        trigger_session_status: TriggerSessionStatus::Running.as_str().to_string(),
        flow_session_id: ctx.flow_session_id.to_string(),
        flow_session_status: FlowSessionStatus::Running.as_str().to_string(),
        action_id: action.action_id.clone(),
        r#type: action.r#type.clone(),
        plugin_name: action.plugin_name.clone(),
        plugin_version: action.plugin_version.clone(),
        stage: if ctx.workflow.published {
            Stage::Production.as_str().to_string()
        } else {
            Stage::Testing.as_str().to_string()
        },
        config: TaskConfig {
            inputs: Some(action.inputs.clone().unwrap()),
            inputs_schema: Some(action.inputs_schema.clone().unwrap()),
            plugin_config: Some(action.plugin_config.clone()),
            plugin_config_schema: Some(action.plugin_config_schema.clone()),
        },
        result: None,
        error: None,
        test_config: None,
        started_at: Some(Utc::now()),
    };

    println!(
        "[PROCESSOR] Calling create_task for action: {}",
        action.label
    );
    let task = create_task(ctx.state.clone(), &next_task_input).await?;
    println!(
        "[PROCESSOR] Successfully created task {} for action: {}",
        task.task_id, action.label
    );

    // Update cache with new task
    {
        println!("[PROCESSOR] Updating cache with new task: {}", task.task_id);
        let mut cache = ctx.state.flow_session_cache.write().await;
        if let Some(mut session_data) = cache.get(&ctx.flow_session_id) {
            session_data
                .tasks
                .insert(task.task_id.clone(), task.clone());
            cache.set(&ctx.flow_session_id, session_data);
            println!(
                "[PROCESSOR] Successfully updated cache with task: {}",
                task.task_id
            );
        } else {
            println!(
                "[PROCESSOR] Warning: Could not find session data in cache for flow: {}",
                ctx.flow_session_id
            );
        }
    }

    Ok(task)
}

/// Finds all unprocessed next actions for a task
pub async fn find_next_actions(
    ctx: &PathProcessingContext,
    task: &Task,
    graph: &HashMap<String, Vec<String>>,
) -> Vec<Action> {
    println!(
        "[PROCESSOR] Finding next actions for task: {} (action: {})",
        task.task_id, task.action_label
    );

    let mut next_actions = Vec::new();

    if let Some(neighbors) = graph.get(&task.action_id) {
        println!(
            "[PROCESSOR] Found {} potential next actions in graph",
            neighbors.len()
        );

        for neighbor_id in neighbors {
            let neighbor = ctx
                .workflow_def
                .actions
                .iter()
                .find(|action| &action.action_id == neighbor_id);

            if let Some(action) = neighbor {
                // Check if this task has already been processed
                let cache = ctx.state.flow_session_cache.read().await;
                if let Some(session_data) = cache.get(&ctx.flow_session_id) {
                    if !session_data
                        .tasks
                        .iter()
                        .any(|(_, t)| t.action_id == action.action_id)
                    {
                        println!(
                            "[PROCESSOR] Adding unprocessed action to next actions: {}",
                            action.label
                        );
                        next_actions.push(action.clone());
                    } else {
                        println!(
                            "[PROCESSOR] Skipping already processed action: {}",
                            action.label
                        );
                    }
                }
            }
        }
    } else {
        println!(
            "[PROCESSOR] No next actions found in graph for task: {}",
            task.task_id
        );
    }

    println!(
        "[PROCESSOR] Found {} unprocessed next actions",
        next_actions.len()
    );
    next_actions
}

/// Updates the task status in the database and cache
pub async fn update_task_with_result(
    ctx: &PathProcessingContext,
    task: &Task,
    task_result: Option<Value>,
    bundled_context: Value,
    status: TaskStatus,
) {
    // Instead of fire-and-forget, wait for the update
    let state_clone = ctx.state.clone();
    let task_id = task.task_id.clone();
    let task_result_clone = task_result.clone();
    let bundled_context_clone = bundled_context.clone();
    let status_clone = status.clone();

    // Wait for the status update to complete
    if let Err(e) = update_task_status(
        state_clone,
        &task_id,
        &status_clone,
        Some(bundled_context_clone),
        task_result_clone,
        None,
    )
    .await
    {
        println!(
            "[PROCESSOR] Failed to update task status in database: {}",
            e
        );
        // Optionally retry or handle the error
    }

    // Update cache after confirming DB update
    let mut cache = ctx.state.flow_session_cache.write().await;
    let mut task_copy = task.clone();
    task_copy.result = task_result;
    task_copy.context = Some(bundled_context);
    task_copy.task_status = status;
    task_copy.ended_at = Some(Utc::now());
    let _ = cache.update_task(&ctx.flow_session_id, task_copy);
}

/// Updates the task status on error
pub async fn handle_task_error(ctx: &PathProcessingContext, task: &Task, error: TaskError) {
    println!(
        "[PROCESSOR] Handling error for task {}: {:?}",
        task.task_id, error
    );

    // Update task status to failed
    let state_clone = ctx.state.clone();
    let task_id = task.task_id.clone();
    let error_clone = error.clone();

    tokio::spawn(async move {
        println!(
            "[PROCESSOR] Spawned task to update error status for task: {}",
            task_id
        );
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
            println!(
                "[PROCESSOR] Failed to update error status in database: {}",
                e
            );
        } else {
            println!(
                "[PROCESSOR] Successfully updated error status in database: {}",
                task_id
            );
        }
    });

    // Update cache
    {
        println!(
            "[PROCESSOR] Updating error status in cache for task: {}",
            task.task_id
        );
        let mut cache = ctx.state.flow_session_cache.write().await;
        let mut task_copy = task.clone();
        task_copy.result = Some(error.error.clone());
        task_copy.context = Some(error.context.clone());
        task_copy.task_status = TaskStatus::Failed;
        task_copy.ended_at = Some(Utc::now());
        let _ = cache.update_task(&ctx.flow_session_id, task_copy);
        println!(
            "[PROCESSOR] Updated error status in cache: {}",
            task.task_id
        );
    }
}

/// Updates the flow session status
pub async fn update_flow_status(
    ctx: &PathProcessingContext,
    status: &FlowSessionStatus,
    trigger_status: &TriggerSessionStatus,
) {
    println!(
        "[PROCESSOR] Updating flow session {} status to: {:?}",
        ctx.flow_session_id, status
    );
    if let Err(e) =
        update_flow_session_status(&ctx.state, &ctx.flow_session_id, status, trigger_status).await
    {
        println!("[PROCESSOR] Failed to update flow session status: {}", e);
    } else {
        println!(
            "[PROCESSOR] Successfully updated flow session status: {}",
            ctx.flow_session_id
        );
    }
}

/// Checks if this is the last active path and updates workflow status accordingly
pub async fn check_and_update_workflow_completion(ctx: &PathProcessingContext, is_success: bool) {
    let mut paths = ctx.active_paths.lock().await;
    *paths -= 1;
    let remaining_paths = *paths;
    println!(
        "[PROCESSOR] Checking workflow completion. Remaining paths: {}",
        remaining_paths
    );

    // If this was the last path, mark workflow as complete or failed
    if remaining_paths == 0 {
        println!(
            "[PROCESSOR] All parallel paths completed for flow {}, workflow is {}",
            ctx.flow_session_id,
            if is_success { "successful" } else { "failed" }
        );

        // Update workflow status
        let status = if is_success {
            FlowSessionStatus::Completed
        } else {
            FlowSessionStatus::Failed
        };

        let trigger_status = if is_success {
            TriggerSessionStatus::Completed
        } else {
            TriggerSessionStatus::Failed
        };

        drop(paths); // Release the lock before the async call

        update_flow_status(ctx, &status, &trigger_status).await;
    } else {
        println!(
            "[PROCESSOR] Still waiting on {} paths to complete",
            remaining_paths
        );
    }
}

/// Processes a single task in a path
pub async fn process_task(
    ctx: &PathProcessingContext,
    task: &Task,
    graph: &HashMap<String, Vec<String>>,
) -> Result<Vec<Action>, TaskError> {
    println!(
        "[PROCESSOR] Starting execution of task {} (action: {})",
        task.task_id, task.action_label
    );

    // Execute the task
    let (task_result, bundled_context) =
        match execute_task(ctx.state.clone(), &ctx.client, task).await {
            Ok(success_value) => {
                println!("[PROCESSOR] Task {} completed successfully", task.task_id);
                success_value
            }
            Err(error) => {
                println!(
                    "[PROCESSOR] Task {} failed with error: {:?}",
                    task.task_id,
                    error.clone()
                );

                // Handle the error
                handle_task_error(ctx, task, error.clone()).await;

                return Err(error);
            }
        };

    print!("[PROCESSOR] Task Result: {:?}", task_result);

    // Update task status to completed
    println!(
        "[PROCESSOR] Updating task {} status to completed",
        task.task_id
    );
    update_task_with_result(
        ctx,
        task,
        task_result.clone(),
        bundled_context,
        TaskStatus::Completed,
    )
    .await;

    // Check if this is a filter task that returned false
    if let Some(plugin_name) = &task.plugin_name {
        println!("[PROCESSOR - FILTER] Checking plugin name: {}", plugin_name);
        if plugin_name.as_str() == "@anything/filter" {
            println!("[PROCESSOR - FILTER] Found filter task: {}", task.task_id);
            if let Some(result_value) = &task_result {
                println!(
                    "[PROCESSOR - FILTER] Filter result value: {:?}",
                    result_value
                );
                // Check if the filter returned false
                if let Some(should_continue) = result_value.get("should_continue") {
                    println!(
                        "[PROCESSOR - FILTER] Found should_continue value: {:?}",
                        should_continue
                    );
                    if let Some(continue_value) = should_continue.as_bool() {
                        println!(
                            "[PROCESSOR - FILTER] Parsed boolean value: {}",
                            continue_value
                        );
                        if !continue_value {
                            println!(
                                "[PROCESSOR - FILTER] Task {} returned false, stopping branch execution",
                                task.task_id
                            );
                            // Return empty vector to indicate no next actions
                            return Ok(Vec::new());
                        }
                        println!(
                            "[PROCESSOR - FILTER] Task {} returned true, continuing execution",
                            task.task_id
                        );
                    } else {
                        println!("[PROCESSOR - FILTER] should_continue is not a boolean value");
                    }
                } else {
                    println!("[PROCESSOR - FILTER] No should_continue field found in result");
                }
            } else {
                println!("[PROCESSOR - FILTER] No result value found for filter task");
            }
        }
    }

    // Find next actions
    println!(
        "[PROCESSOR] Finding next actions for completed task: {}",
        task.task_id
    );
    let next_actions = find_next_actions(ctx, task, graph).await;
    println!(
        "[PROCESSOR] Found {} next actions for task {}",
        next_actions.len(),
        task.task_id
    );

    Ok(next_actions)
}

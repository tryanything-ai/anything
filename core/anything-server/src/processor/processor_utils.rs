use crate::processor::execute_task::execute_task;
use crate::status_updater::{Operation, StatusUpdateMessage};

use serde_json::Value;

use crate::processor::execute_task::TaskError;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::processor::parallelizer::ProcessingContext;

use crate::types::{
    action_types::Action,
    task_types::{Stage, Task, TaskConfig, TaskStatus},
};

use std::time::Instant;
use tracing::{error, info, instrument, warn};

/// Creates a task for the given action
#[instrument(skip(ctx, task), fields(
    task_id = %task.task_id,
    flow_session_id = %ctx.flow_session_id,
))]
pub async fn create_task(
    ctx: &ProcessingContext,
    task: &Task,
) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
    let create_start = Instant::now();
    info!("[PROCESSOR_UTILS] Creating new task: {}", task.task_id);

    let create_task_message = StatusUpdateMessage {
        operation: Operation::CreateTask {
            task_id: task.task_id.clone(),
            input: task.clone(),
        },
    };

    if let Err(e) = ctx
        .state
        .task_updater_sender
        .send(create_task_message)
        .await
    {
        error!(
            "[PROCESSOR_UTILS] Failed to send create task message: {}",
            e
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send task creation message: {}", e),
        )));
    }

    // Update in-memory task tracking
    let cache_start = Instant::now();
    {
        let mut processed_tasks = ctx.processed_tasks.lock().await;
        processed_tasks.insert(task.task_id, task.clone());
        info!(
            "[PROCESSOR_UTILS] Successfully updated in-memory task tracking: {} in {:?}",
            task.task_id,
            cache_start.elapsed()
        );
    }

    let total_duration = create_start.elapsed();
    info!(
        "[PROCESSOR_UTILS] Task creation completed in {:?}",
        total_duration
    );

    Ok(task.clone())
}

/// Creates a task for the given action
#[instrument(skip(ctx, action), fields(
    action_id = %action.action_id,
    action_label = %action.label,
    processing_order = %processing_order,
    flow_session_id = %ctx.flow_session_id,
))]
pub async fn create_task_for_action(
    ctx: &ProcessingContext,
    action: &Action,
    processing_order: i32,
) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
    let create_start = Instant::now();
    info!(
        "[PROCESSOR_UTILS] Creating new task for action: {} (order: {})",
        action.label, processing_order
    );

    let task = Task::builder()
        .account_id(ctx.workflow.account_id.clone())
        .flow_id(ctx.workflow_id.clone())
        .flow_version_id(ctx.workflow.flow_version_id.clone())
        .action_label(action.label.clone())
        .trigger_id(ctx.trigger_task_id.clone())
        .flow_session_id(ctx.flow_session_id.clone())
        .trigger_session_id(ctx.trigger_session_id.clone())
        .action_id(action.action_id.clone())
        .r#type(action.r#type.clone())
        .plugin_name(action.plugin_name.clone())
        .plugin_version(action.plugin_version.clone())
        .stage(if ctx.workflow.published {
            Stage::Production
        } else {
            Stage::Testing
        })
        .processing_order(processing_order)
        .config(TaskConfig {
            inputs: Some(action.inputs.clone().unwrap_or_default()),
            inputs_schema: Some(action.inputs_schema.clone().unwrap()),
            plugin_config: Some(action.plugin_config.clone()),
            plugin_config_schema: Some(action.plugin_config_schema.clone()),
        })
        .build()
        .map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )) as Box<dyn std::error::Error + Send + Sync>
        })?;

    let message_start = Instant::now();
    info!(
        "[PROCESSOR_UTILS] Sending create task message for action: {}",
        action.label
    );

    let create_task_message = StatusUpdateMessage {
        operation: Operation::CreateTask {
            task_id: task.task_id.clone(),
            input: task.clone(),
        },
    };

    if let Err(e) = ctx
        .state
        .task_updater_sender
        .send(create_task_message)
        .await
    {
        error!(
            "[PROCESSOR_UTILS] Failed to send create task message: {}",
            e
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send task creation message: {}", e),
        )));
    }

    info!(
        "[PROCESSOR_UTILS] Task message sent in {:?}",
        message_start.elapsed()
    );

    // Update in-memory task tracking
    let cache_start = Instant::now();
    {
        let mut processed_tasks = ctx.processed_tasks.lock().await;
        processed_tasks.insert(task.task_id, task.clone());
        info!(
            "[PROCESSOR_UTILS] Successfully updated in-memory task tracking: {} in {:?}",
            task.task_id,
            cache_start.elapsed()
        );
    }

    let total_duration = create_start.elapsed();
    info!(
        "[PROCESSOR_UTILS] Task for action creation completed in {:?}",
        total_duration
    );

    Ok(task)
}

/// Finds all unprocessed next actions for a task
#[instrument(skip(ctx, task, graph), fields(
    task_id = %task.task_id,
    action_label = %task.action_label,
    flow_session_id = %ctx.flow_session_id,
))]
pub async fn find_next_actions(
    ctx: &ProcessingContext,
    task: &Task,
    graph: &HashMap<String, Vec<String>>,
) -> Vec<Action> {
    info!(
        "[PROCESSOR_UTILS] Finding next actions for task: {} (action: {})",
        task.task_id, task.action_label
    );

    let mut next_actions = Vec::new();

    if let Some(neighbors) = graph.get(&task.action_id) {
        info!(
            "[PROCESSOR_UTILS] Found {} potential next actions in graph: {:?}",
            neighbors.len(),
            neighbors
        );

        for neighbor_id in neighbors {
            info!(
                "[PROCESSOR_UTILS] Evaluating neighbor with ID: {} for task: {}",
                neighbor_id, task.task_id
            );

            let neighbor = ctx
                .workflow_def
                .actions
                .iter()
                .find(|action| &action.action_id == neighbor_id);

            if let Some(action) = neighbor {
                info!(
                    "[PROCESSOR_UTILS] Found action in workflow definition: {} (ID: {})",
                    action.label, action.action_id
                );

                // Check if this action has already been processed using in-memory tracking
                let processed_tasks = ctx.processed_tasks.lock().await;
                let already_processed = processed_tasks
                    .values()
                    .any(|t| t.action_id == action.action_id);

                if !already_processed {
                    info!(
                        "[PROCESSOR_UTILS] Adding unprocessed action to next actions: {}",
                        action.label
                    );
                    next_actions.push(action.clone());
                } else {
                    info!(
                        "[PROCESSOR_UTILS] Skipping already processed action: {}",
                        action.label
                    );
                }
            } else {
                warn!(
                    "[PROCESSOR_UTILS] No action found in workflow definition for neighbor ID: {}",
                    neighbor_id
                );
            }
        }
    } else {
        info!(
            "[PROCESSOR_UTILS] No next actions found in graph for task: {}",
            task.task_id
        );
    }

    info!(
        "[PROCESSOR_UTILS] Found {} unprocessed next actions",
        next_actions.len()
    );
    next_actions
}

/// Updates the task status in the database and in-memory tracking
pub async fn update_completed_task_with_result(
    ctx: &ProcessingContext,
    task: &Task,
    task_result: Option<Value>,
    bundled_context: Value,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
) {
    // Update in-memory task tracking immediately
    let mut processed_tasks = ctx.processed_tasks.lock().await;
    let mut task_copy = task.clone();
    task_copy.result = task_result.clone();
    task_copy.context = Some(bundled_context.clone());
    task_copy.task_status = TaskStatus::Completed;
    task_copy.ended_at = Some(Utc::now());
    processed_tasks.insert(task.task_id, task_copy);
    drop(processed_tasks);

    let task_message = StatusUpdateMessage {
        operation: Operation::UpdateTask {
            task_id: task.task_id.clone(),
            status: TaskStatus::Completed,
            result: task_result.clone(),
            error: None,
            context: Some(bundled_context.clone()),
            started_at: Some(started_at),
            ended_at: Some(ended_at),
        },
    };

    if let Err(e) = ctx.state.task_updater_sender.send(task_message).await {
        println!("[PROCESSOR] Failed to send completed task update: {}", e);
    }
}

/// Updates the task status on error
pub async fn handle_task_error(
    ctx: &ProcessingContext,
    task: &Task,
    error: TaskError,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
) {
    // Update in-memory task tracking immediately
    let mut processed_tasks = ctx.processed_tasks.lock().await;
    let mut task_copy = task.clone();
    task_copy.result = Some(error.error.clone());
    task_copy.context = Some(error.context.clone());
    task_copy.task_status = TaskStatus::Failed;
    task_copy.ended_at = Some(Utc::now());
    processed_tasks.insert(task.task_id, task_copy);
    drop(processed_tasks);

    let error_message = StatusUpdateMessage {
        operation: Operation::UpdateTask {
            task_id: task.task_id.clone(),
            status: TaskStatus::Failed,
            result: None,
            error: Some(error.error.clone()),
            context: Some(error.context.clone()),
            started_at: Some(started_at),
            ended_at: Some(ended_at),
        },
    };

    if let Err(e) = ctx.state.task_updater_sender.send(error_message).await {
        println!("[PROCESSOR] Failed to send task error update: {}", e);
    }
}

/// Processes a single task in a path
#[instrument(skip(ctx, task, graph), fields(
    task_id = %task.task_id,
    action_label = %task.action_label,
    flow_session_id = %ctx.flow_session_id,
))]
pub async fn process_task(
    ctx: &ProcessingContext,
    task: &Task,
    graph: &HashMap<String, Vec<String>>,
) -> Result<Vec<Action>, TaskError> {
    let process_start = Instant::now();
    info!(
        "[PROCESSOR_UTILS] Processing task {} (action: {})",
        task.task_id, task.action_label
    );

    let started_at = Utc::now();
    let execution_start = Instant::now();

    // Get a clone of in-memory tasks for bundling context
    let in_memory_tasks = {
        let processed_tasks = ctx.processed_tasks.lock().await;
        processed_tasks.clone()
    };

    info!(
        "[PROCESSOR_UTILS] About to call execute_task for {}",
        task.task_id
    );
    let (task_result, bundled_context, _, ended_at) =
        match execute_task(ctx.state.clone(), &ctx.client, task, Some(&in_memory_tasks)).await {
            Ok(success_value) => {
                let execution_duration = execution_start.elapsed();
                info!(
                    "[PROCESSOR_UTILS] Task {} executed successfully in {:?}",
                    task.task_id, execution_duration
                );
                success_value
            }
            Err(error) => {
                let execution_duration = execution_start.elapsed();
                error!(
                    "[PROCESSOR_UTILS] Task {} execution failed after {:?}: {}",
                    task.task_id, execution_duration, error.error
                );
                handle_task_error(ctx, task, error.clone(), started_at, Utc::now()).await;
                return Ok(Vec::new());
            }
        };
    info!(
        "[PROCESSOR_UTILS] execute_task completed for {}",
        task.task_id
    );

    let update_start = Instant::now();
    update_completed_task_with_result(
        ctx,
        task,
        task_result.clone(),
        bundled_context,
        started_at,
        ended_at,
    )
    .await;
    info!(
        "[PROCESSOR_UTILS] Task {} status updated in {:?}",
        task.task_id,
        update_start.elapsed()
    );

    // Handle filter tasks
    if let Some(plugin_name) = &task.plugin_name {
        if plugin_name.as_str() == "@anything/filter" {
            info!("[PROCESSOR_UTILS] Processing filter task: {}", task.task_id);
            if let Some(result_value) = &task_result {
                if let Some(should_continue) = result_value.get("should_continue") {
                    if let Some(false) = should_continue.as_bool() {
                        info!(
                            "[PROCESSOR_UTILS] Filter task {} returned false, stopping branch",
                            task.task_id
                        );
                        return Ok(Vec::new());
                    }
                }
            }
        }
    }

    let next_actions_start = Instant::now();
    let next_actions = find_next_actions(ctx, task, graph).await;
    info!(
        "[PROCESSOR_UTILS] Found {} next actions for task {} in {:?}",
        next_actions.len(),
        task.task_id,
        next_actions_start.elapsed()
    );

    let total_duration = process_start.elapsed();
    info!(
        "[PROCESSOR_UTILS] Task {} processing completed in {:?}",
        task.task_id, total_duration
    );

    Ok(next_actions)
}

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

/// Creates a task for the given action
pub async fn create_task(
    ctx: &ProcessingContext,
    task: &Task,
) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESSOR] Creating new task: {}", task.task_id);

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
        println!("[PROCESSOR] Failed to send create task message: {}", e);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send task creation message: {}", e),
        )));
    }

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

    Ok(task.clone())
}

/// Creates a task for the given action
pub async fn create_task_for_action(
    ctx: &ProcessingContext,
    action: &Action,
    processing_order: i32,
) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR] Creating new task for action: {} (order: {})",
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

    let create_task_start = Instant::now();
    println!(
        "[PROCESSOR] Calling create_task for action: {}",
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
        println!("[PROCESSOR] Failed to send create task message: {}", e);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send task creation message: {}", e),
        )));
    }

    println!(
        "[SPEED] ProcessorUtils::create_task_message - {:?}",
        create_task_start.elapsed()
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
    ctx: &ProcessingContext,
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
            "[PROCESSOR] Found {} potential next actions in graph: {:?}",
            neighbors.len(),
            neighbors
        );

        for neighbor_id in neighbors {
            println!(
                "[PROCESSOR] Evaluating neighbor with ID: {} for task: {}",
                neighbor_id, task.task_id
            );

            // println!("[PROCESSOR] Workflow definition: {:?}", ctx.workflow_def);

            let neighbor = ctx
                .workflow_def
                .actions
                .iter()
                .find(|action| &action.action_id == neighbor_id);

            println!(
                "[PROCESSOR] Found neighbor in workflow definition: {} (ID: {})",
                neighbor.unwrap().label,
                neighbor_id
            );

            if let Some(action) = neighbor {
                println!(
                    "[PROCESSOR] Found action in workflow definition: {} (ID: {})",
                    action.label, action.action_id
                );

                let cache = ctx.state.flow_session_cache.read().await;
                // Check if this task has already been processed
                if let Some(session_data) = cache.get(&ctx.flow_session_id) {
                    println!(
                        "[PROCESSOR] Retrieved session data for flow session ID: {}",
                        ctx.flow_session_id
                    );

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
                } else {
                    println!(
                        "[PROCESSOR] Warning: No session data found for flow session ID: {}",
                        ctx.flow_session_id
                    );
                }
            } else {
                println!(
                    "[PROCESSOR] No action found in workflow definition for neighbor ID: {}",
                    neighbor_id
                );
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
pub async fn update_completed_task_with_result(
    ctx: &ProcessingContext,
    task: &Task,
    task_result: Option<Value>,
    bundled_context: Value,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
) {
    // Update cache immediately
    let mut cache = ctx.state.flow_session_cache.write().await;
    let mut task_copy = task.clone();
    task_copy.result = task_result.clone();
    task_copy.context = Some(bundled_context.clone());
    task_copy.task_status = TaskStatus::Completed;
    task_copy.ended_at = Some(Utc::now());
    let _ = cache.update_task(&ctx.flow_session_id, task_copy);
    drop(cache);

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
    // Update cache immediately
    let mut cache = ctx.state.flow_session_cache.write().await;
    let mut task_copy = task.clone();
    task_copy.result = Some(error.error.clone());
    task_copy.context = Some(error.context.clone());
    task_copy.task_status = TaskStatus::Failed;
    task_copy.ended_at = Some(Utc::now());
    let _ = cache.update_task(&ctx.flow_session_id, task_copy);
    drop(cache);

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
pub async fn process_task(
    ctx: &ProcessingContext,
    task: &Task,
    graph: &HashMap<String, Vec<String>>,
) -> Result<Vec<Action>, TaskError> {
    println!(
        "[PROCESSOR] Processing task {} (action: {})",
        task.task_id, task.action_label
    );

    let started_at = Utc::now();
    let (task_result, bundled_context, _, ended_at) =
        match execute_task(ctx.state.clone(), &ctx.client, task).await {
            Ok(success_value) => success_value,
            Err(error) => {
                handle_task_error(ctx, task, error.clone(), started_at, Utc::now()).await;
                return Ok(Vec::new());
            }
        };

    update_completed_task_with_result(
        ctx,
        task,
        task_result.clone(),
        bundled_context,
        started_at,
        ended_at,
    )
    .await;

    // Handle filter tasks
    if let Some(plugin_name) = &task.plugin_name {
        if plugin_name.as_str() == "@anything/filter" {
            if let Some(result_value) = &task_result {
                if let Some(should_continue) = result_value.get("should_continue") {
                    if let Some(false) = should_continue.as_bool() {
                        return Ok(Vec::new());
                    }
                }
            }
        }
    }

    let next_actions = find_next_actions(ctx, task, graph).await;
    Ok(next_actions)
}

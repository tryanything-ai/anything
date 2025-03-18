use crate::processor::processor_utils::{create_task_for_action, drop_path_counter, process_task};

use crate::processor::utils::create_workflow_graph;

use crate::processor::parallelizer::PathProcessingContext;
use crate::types::task_types::Task;

pub fn spawn_path_processor(ctx: PathProcessingContext, task: Task) {
    println!(
        "[PATH PROCESSOR] Entering spawn_path_processor for task: {}",
        task.task_id
    );
    tokio::spawn(async move {
        println!(
            "[PATH PROCESSOR] Starting parallel path for action: {} (task: {})",
            task.action_label, task.task_id
        );
        println!(
            "[PATH PROCESSOR] Attempting to acquire semaphore permit for task: {}",
            task.task_id
        );
        let number_of_paths_permit = match ctx.path_semaphore.acquire().await {
            Ok(permit) => permit,
            Err(e) => {
                println!(
                    "[PATH PROCESSOR] Failed to acquire path permit for task {}: {}",
                    task.task_id, e
                );
                // Decrement active paths counter
                {
                    let mut paths = ctx.active_paths.lock().await;
                    *paths -= 1;
                    println!(
                        "[PATH PROCESSOR] Decremented active paths to {} after permit failure",
                        *paths
                    );
                }
                return;
            }
        };

        // Process the path (inline the process_path logic)
        println!(
            "[PATH PROCESSOR] Creating workflow graph for task: {}",
            task.task_id
        );
        let graph = create_workflow_graph(&ctx.workflow_def);

        //Create mutable Task
        let mut current_task = task;

        // Process tasks in this path until completion
        println!("[PATH PROCESSOR] Starting task processing loop for path");
        loop {
            println!(
                "[PATH PROCESSOR] Processing task: {} in loop",
                current_task.task_id
            );
            // Process the current task
            let next_actions = match process_task(&ctx, &current_task, &graph).await {
                Ok(actions) => {
                    println!(
                        "[PATH PROCESSOR] Found {} next actions for task {}",
                        actions.len(),
                        current_task.task_id
                    );
                    actions
                }
                Err(_e) => {
                    println!(
                        "[PATH PROCESSOR] Task {} failed, marking path as failed",
                        current_task.task_id
                    );
                    // Task failed, mark path as failed and exit
                    // check_and_update_workflow_completion(&ctx, false).await;
                    drop_path_counter(&ctx).await;
                    drop(number_of_paths_permit);
                    return;
                }
            };

            // If we have multiple next actions, spawn new paths for all but the first
            if next_actions.len() > 1 {
                println!(
                    "[PATH PROCESSOR] Multiple next actions found ({}), spawning new paths",
                    next_actions.len()
                );
                // Increment active paths counter for the additional paths
                {
                    let mut paths = ctx.active_paths.lock().await;
                    *paths += next_actions.len() - 1; // -1 because we'll process one in this path
                    println!(
                        "[PATH PROCESSOR] Incremented active paths to {} for parallel processing",
                        *paths
                    );
                }

                // Process all but the first action in new paths
                for (idx, next_action) in next_actions.iter().skip(1).enumerate() {
                    println!(
                        "[PATH PROCESSOR] Creating task for parallel path {} of {}",
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
                                "[PATH PROCESSOR] Successfully created task {} for parallel path",
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
                                "[PATH PROCESSOR] Spawning new process path for task: {}",
                                new_task.task_id
                            );
                            spawn_path_processor(new_ctx, new_task);
                        }
                        Err(e) => {
                            println!(
                                "[PATH PROCESSOR] Error creating task for parallel path: {}",
                                e
                            );

                            // Decrement active paths counter for this failed path
                            {
                                let mut paths = ctx.active_paths.lock().await;
                                *paths -= 1;
                                println!("[PATH PROCESSOR] Decremented active paths to {} after task creation failure", *paths);
                            }
                        }
                    }
                }

                // Continue with the first next action in this path
                println!("[PATH PROCESSOR] Continuing with first action in current path");
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
                                "[PATH PROCESSOR] Created next task {} in current path",
                                new_task.task_id
                            );
                            current_task = new_task;
                        }
                        Err(e) => {
                            println!(
                                "[PATH PROCESSOR] Error creating next task in current path: {}",
                                e
                            );

                            // Path is complete with error
                            //TODO: figure how to handle this better.
                            // check_and_update_workflow_completion(&ctx, false).await;
                            drop_path_counter(&ctx).await;
                            drop(number_of_paths_permit);
                            return;
                        }
                    }
                } else {
                    println!("[PATH PROCESSOR] No first action found (unexpected), breaking loop");
                    // No next action (shouldn't happen, but handle it)
                    break;
                }
            } else if next_actions.len() == 1 {
                println!("[PATH PROCESSOR] Single next action found, continuing in current path");
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
                            "[PATH PROCESSOR] Created next task {} in current path",
                            new_task.task_id
                        );
                        current_task = new_task;
                    }
                    Err(e) => {
                        println!(
                            "[PATH PROCESSOR] Error creating next task in current path: {}",
                            e
                        );

                        // Path is complete with error
                        drop_path_counter(&ctx).await;
                        drop(number_of_paths_permit);
                        return;
                    }
                }
            } else {
                // No more actions in this path
                println!("[PATH PROCESSOR] No more actions in path, completing successfully");
                break;
            }
        }

        // Path completed successfully
        println!(
            "[PATH PROCESSOR] Path completed successfully, updating workflow completion status"
        );
        // check_and_update_workflow_completion(&ctx, true).await;
        drop_path_counter(&ctx).await;

        // Release the semaphore permit
        println!("[PATH PROCESSOR] Releasing semaphore permit");
        drop(number_of_paths_permit);
    });
}

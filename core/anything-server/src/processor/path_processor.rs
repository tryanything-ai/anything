use crate::processor::parallelizer::ProcessingContext;
use crate::processor::processor_utils::{create_task_for_action, process_task};
use crate::processor::utils::create_workflow_graph;
use crate::status_updater::{Operation, StatusUpdateMessage};
use crate::types::task_types::Task;
use crate::types::task_types::TaskStatus;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::error;

pub fn process_task_and_branches(
    ctx: Arc<ProcessingContext>,
    initial_task: Task,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let graph = create_workflow_graph(&ctx.workflow_def);
        let current_task = initial_task;

        loop {
            let next_actions = match process_task(&ctx, &current_task, &graph).await {
                Ok(actions) => actions,
                Err(e) => {
                    // Update task status to Failed
                    if let Err(send_err) = ctx
                        .state
                        .task_updater_sender
                        .send(StatusUpdateMessage {
                            operation: Operation::UpdateTask {
                                task_id: current_task.task_id,
                                started_at: None,
                                ended_at: Some(chrono::Utc::now()),
                                status: TaskStatus::Failed,
                                result: None,
                                context: None,
                                error: Some(serde_json::json!({ "error": e.error })),
                            },
                        })
                        .await
                    {
                        println!("[PROCESSOR] Failed to send status update: {}", send_err);
                    }
                    break;
                }
            };

            if next_actions.is_empty() {
                break;
            }

            // Process each branch sequentially
            for action in next_actions {
                match create_task_for_action(&ctx, &action, current_task.processing_order + 1).await
                {
                    Ok(new_task) => {
                        process_task_and_branches(ctx.clone(), new_task).await;
                    }
                    Err(e) => {
                        error!(
                            "[PROCESSOR] Error creating task for action {}: {}",
                            action.action_id, e
                        );

                        // Critical: Update flow session status to failed when branch creation fails
                        if let Err(send_err) = ctx
                            .state
                            .task_updater_sender
                            .send(StatusUpdateMessage {
                                operation: Operation::CompleteWorkflow {
                                    flow_session_id: ctx.flow_session_id,
                                    status: crate::types::task_types::FlowSessionStatus::Failed,
                                    trigger_status:
                                        crate::types::task_types::TriggerSessionStatus::Failed,
                                },
                            })
                            .await
                        {
                            error!(
                                "[PROCESSOR] Failed to send workflow failure status update: {}",
                                send_err
                            );

                            // As fallback, try direct database update
                            if let Err(db_err) =
                                crate::processor::db_calls::update_flow_session_status(
                                    &ctx.state,
                                    &ctx.flow_session_id,
                                    &crate::types::task_types::FlowSessionStatus::Failed,
                                    &crate::types::task_types::TriggerSessionStatus::Failed,
                                )
                                .await
                            {
                                error!("[PROCESSOR] Direct database update for workflow failure also failed: {}", db_err);
                            }
                        }

                        // Break out of the loop since workflow failed
                        return;
                    }
                }
            }
            break; // We still need this break to prevent processing the same level multiple times
        }
    })
}

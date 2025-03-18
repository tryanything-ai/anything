use crate::processor::parallelizer::ProcessingContext;
use crate::processor::processor_utils::{create_task_for_action, process_task};
use crate::processor::utils::create_workflow_graph;
use crate::status_updater::{Operation, StatusUpdateMessage};
use crate::types::task_types::Task;
use crate::types::task_types::TaskStatus;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub fn process_task_and_branches(
    ctx: Arc<ProcessingContext>,
    initial_task: Task,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let graph = create_workflow_graph(&ctx.workflow_def);
        let mut current_task = initial_task;

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
                        println!("[PROCESSOR] Error creating task: {}", e);
                    }
                }
            }
            break; // We still need this break to prevent processing the same level multiple times
        }
    })
}

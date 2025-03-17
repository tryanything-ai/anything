// Define the type of task operation
#[derive(Debug, Clone)]
pub enum TaskOperation {
    Update {
        status: TaskStatus,
        result: Option<Value>,
        context: Option<Value>,
        error: Option<Value>,
    },
    Create {
        input: Task,
    },
}

// Update the message struct to use the operation enum
#[derive(Debug, Clone)]
pub struct TaskMessage {
    pub task_id: Uuid,
    pub operation: TaskOperation,
}

use crate::processor::db_calls::create_task;
use crate::processor::db_calls::update_task_status;
use crate::types::task_types::Task;
use crate::types::task_types::TaskStatus;
use crate::AppState;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

pub async fn task_database_status_processor(
    state: Arc<AppState>,
    mut receiver: Receiver<TaskMessage>,
) {
    while let Some(message) = receiver.recv().await {
        println!(
            "[TASK PROCESSOR] Received message for task: {}",
            message.task_id
        );

        let result = match message.operation {
            TaskOperation::Update {
                status,
                result,
                context,
                error,
            } => {
                update_task_status(
                    state.clone(),
                    &message.task_id,
                    &status,
                    context,
                    result,
                    error,
                )
                .await
            }
            TaskOperation::Create { input } => create_task(state.clone(), &input).await,
        };

        if let Err(e) = result {
            println!("[TASK PROCESSOR] Failed to process task: {}", e);
        } else {
            println!(
                "[TASK PROCESSOR] Successfully processed task: {}",
                message.task_id
            );
        }
    }
}

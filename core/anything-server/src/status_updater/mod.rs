use crate::processor::db_calls::update_flow_session_status;
use crate::types::task_types::{FlowSessionStatus, TaskStatus, TriggerSessionStatus};

use crate::processor::db_calls::create_task;
use crate::processor::db_calls::update_task_status;
use crate::types::task_types::Task;
use crate::AppState;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
use chrono::{DateTime, Utc};
// Define the type of task operation
#[derive(Debug, Clone)]
pub enum Operation {
    UpdateTask {
        started_at: Option<DateTime<Utc>>,
        ended_at: Option<DateTime<Utc>>,
        status: TaskStatus,
        result: Option<Value>,
        context: Option<Value>,
        error: Option<Value>,
    },
    CreateTask {
        input: Task,
    },
    CompleteWorkflow {
        flow_session_id: Uuid,
        status: FlowSessionStatus,
        trigger_status: TriggerSessionStatus,
    },
}

// Update the message struct to use the operation enum
#[derive(Debug, Clone)]
pub struct StatusUpdateMessage {
    pub task_id: Uuid,
    pub operation: Operation,
}

pub async fn task_database_status_processor(
    state: Arc<AppState>,
    mut receiver: Receiver<StatusUpdateMessage>,
) {
    while let Some(message) = receiver.recv().await {
        println!(
            "[TASK PROCESSOR] Received message for task: {}",
            message.task_id
        );

        let result = match message.operation {
            Operation::UpdateTask {
                started_at,
                ended_at,
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
                    started_at,
                    ended_at,
                )
                .await
            }
            Operation::CreateTask { input } => create_task(state.clone(), &input).await,
            Operation::CompleteWorkflow {
                flow_session_id,
                status,
                trigger_status,
            } => {
                update_flow_session_status(&state, &flow_session_id, &status, &trigger_status).await
            }
        };

        if let Err(e) = result {
            println!("[TASK STATUS UPDATER] Failed to process update: {}", e);
        } else {
            println!(
                "[TASK STATUS UPDATER] Successfully processed update: {}",
                message.task_id
            );
        }
    }
}

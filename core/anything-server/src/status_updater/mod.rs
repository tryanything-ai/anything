use crate::processor::db_calls::{create_task, update_flow_session_status, update_task_status};
use crate::types::task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus};
use crate::AppState;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

// Define the type of task operation
#[derive(Debug, Clone)]
pub enum Operation {
    UpdateTask {
        task_id: Uuid,
        started_at: Option<DateTime<Utc>>,
        ended_at: Option<DateTime<Utc>>,
        status: TaskStatus,
        result: Option<Value>,
        context: Option<Value>,
        error: Option<Value>,
    },
    CreateTask {
        task_id: Uuid,
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
    pub operation: Operation,
}

pub async fn task_database_status_processor(
    state: Arc<AppState>,
    mut receiver: Receiver<StatusUpdateMessage>,
) {
    const TIMEOUT_DURATION: Duration = Duration::from_secs(30);
    const MAX_RETRIES: u32 = 3;

    println!("[TASK PROCESSOR] Starting status updater processor");

    loop {
        // Check shutdown signal first
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            println!("[TASK PROCESSOR] Shutdown signal received, stopping status updater");
            break;
        }

        // Try to receive a message with timeout
        match timeout(TIMEOUT_DURATION, receiver.recv()).await {
            Ok(Some(message)) => {
                println!(
                    "[TASK PROCESSOR] Processing status update: {:?}",
                    message.operation
                );

                let mut retries = 0;
                let mut last_error = None;

                // Retry loop for database operations
                while retries < MAX_RETRIES {
                    let result = match &message.operation {
                        Operation::UpdateTask {
                            task_id,
                            started_at,
                            ended_at,
                            status,
                            result,
                            context,
                            error,
                        } => {
                            update_task_status(
                                state.clone(),
                                task_id,
                                status,
                                context.clone(),
                                result.clone(),
                                error.clone(),
                                *started_at,
                                *ended_at,
                            )
                            .await
                        }
                        Operation::CreateTask { task_id: _, input } => {
                            create_task(state.clone(), input).await
                        }
                        Operation::CompleteWorkflow {
                            flow_session_id,
                            status,
                            trigger_status,
                        } => {
                            update_flow_session_status(
                                &state,
                                flow_session_id,
                                status,
                                trigger_status,
                            )
                            .await
                        }
                    };

                    match result {
                        Ok(_) => {
                            println!("[TASK PROCESSOR] Successfully processed update");
                            break;
                        }
                        Err(e) => {
                            last_error = Some(e);
                            retries += 1;
                            if retries < MAX_RETRIES {
                                println!("[TASK PROCESSOR] Retry {} of {}", retries, MAX_RETRIES);
                                tokio::time::sleep(Duration::from_millis(500 * retries as u64))
                                    .await;
                            }
                        }
                    }
                }

                if let Some(e) = last_error {
                    if retries >= MAX_RETRIES {
                        println!(
                            "[TASK PROCESSOR] Failed to process update after {} retries: {}",
                            MAX_RETRIES, e
                        );
                    }
                }
            }
            Ok(None) => {
                // Channel was closed
                println!("[TASK PROCESSOR] Channel was closed unexpectedly");
                if !state
                    .shutdown_signal
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    println!(
                        "[TASK PROCESSOR] ERROR: Channel closed while processor was still running!"
                    );
                }
                break;
            }
            Err(_timeout) => {
                // Timeout occurred - this is normal, just continue
                println!(
                    "[TASK PROCESSOR] No messages received in {:?}",
                    TIMEOUT_DURATION
                );
                continue;
            }
        }
    }

    println!("[TASK PROCESSOR] Status updater processor shutdown complete");
}

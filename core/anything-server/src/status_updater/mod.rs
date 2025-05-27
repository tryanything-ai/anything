use crate::processor::db_calls::{create_task, update_flow_session_status, update_task_status};
use crate::types::task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus};
use crate::websocket::{broadcast_task_update_simple, broadcast_task_update_with_session, broadcast_workflow_completion_simple, UpdateType};
use crate::AppState;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::time::{timeout, Duration};
use tracing::{info, span, Instrument, Level};
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

    info!("[TASK PROCESSOR] Starting status updater processor");

    loop {
        // Check shutdown signal first
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            info!("[TASK PROCESSOR] Shutdown signal received, stopping status updater");
            break;
        }

        // Try to receive a message with timeout
        match timeout(TIMEOUT_DURATION, receiver.recv()).await {
            Ok(Some(message)) => {
                let operation_kind = match &message.operation {
                    Operation::UpdateTask { .. } => "UpdateTask",
                    Operation::CreateTask { .. } => "CreateTask",
                    Operation::CompleteWorkflow { .. } => "CompleteWorkflow",
                };

                let message_span = span!(
                    Level::INFO,
                    "process_status_update_message",
                    operation_type = %operation_kind,
                    otel.kind = "CONSUMER"
                );

                async {
                    info!(
                        "[TASK PROCESSOR] Processing status update: {:?}",
                        message.operation
                    );

                    let mut retries = 0;
                    let mut last_error = None;

                    // Retry loop for database operations
                    while retries < MAX_RETRIES {
                        let db_op_span = span!(
                            Level::INFO,
                            "database_operation_attempt",
                            retry_count = retries
                        );

                        let result = async {
                            match &message.operation {
                                Operation::UpdateTask {
                                    task_id,
                                    started_at,
                                    ended_at,
                                    status,
                                    result,
                                    context,
                                    error,
                                } => {
                                    span!(Level::DEBUG, "update_task_status_db_call", task_id = %task_id, task_status = ?status).in_scope(|| {
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
                                    })
                                    .await
                                }
                                Operation::CreateTask { task_id, input } => {
                                    span!(Level::DEBUG, "create_task_db_call", task_id = %task_id).in_scope(|| {
                                        create_task(state.clone(), input)
                                    }).await
                                }
                                Operation::CompleteWorkflow {
                                    flow_session_id,
                                    status,
                                    trigger_status,
                                } => {
                                    span!(Level::DEBUG, "complete_workflow_db_call", flow_session_id = %flow_session_id, flow_status = ?status).in_scope(|| {
                                        update_flow_session_status(
                                            &state,
                                            flow_session_id,
                                            status,
                                            trigger_status,
                                        )
                                    })
                                    .await
                                }
                            }
                        }
                        .instrument(db_op_span)
                        .await;

                        match result {
                            Ok(_) => {
                                info!("[TASK PROCESSOR] Successfully processed update");
                                
                                // Broadcast WebSocket updates after successful database operations
                                match &message.operation {
                                    Operation::UpdateTask { task_id, status, .. } => {
                                        // We'll broadcast a simple update and let the frontend fetch the task data
                                        let update_type = match status {
                                            TaskStatus::Completed => UpdateType::TaskCompleted,
                                            TaskStatus::Failed => UpdateType::TaskFailed,
                                            _ => UpdateType::TaskUpdated,
                                        };
                                        broadcast_task_update_simple(&state.workflow_broadcaster, task_id, update_type).await;
                                    }
                                    Operation::CreateTask { input, .. } => {
                                        broadcast_task_update_with_session(&state.workflow_broadcaster, &input.account_id.to_string(), &input.flow_session_id.to_string(), &input.task_id, UpdateType::TaskCreated).await;
                                    }
                                    Operation::CompleteWorkflow { flow_session_id, status, .. } => {
                                        let success = matches!(status, FlowSessionStatus::Completed);
                                        broadcast_workflow_completion_simple(&state.workflow_broadcaster, flow_session_id, success).await;
                                    }
                                }
                                
                                break;
                            }
                            Err(e) => {
                                last_error = Some(e);
                                retries += 1;
                                if retries < MAX_RETRIES {
                                    info!("[TASK PROCESSOR] Retry {} of {}", retries, MAX_RETRIES);
                                    tokio::time::sleep(Duration::from_millis(500 * retries as u64))
                                        .await;
                                }
                            }
                        }
                    }

                    if let Some(e) = last_error {
                        if retries >= MAX_RETRIES {
                            tracing::error!(
                                error = %e,
                                retries = MAX_RETRIES,
                                "[TASK PROCESSOR] Failed to process update after max retries"
                            );
                            info!(
                                "[TASK PROCESSOR] Failed to process update after {} retries: {}",
                                MAX_RETRIES, e
                            );
                        }
                    }
                }
                .instrument(message_span)
                .await;
            }
            Ok(None) => {
                // Channel was closed
                info!("[TASK PROCESSOR] Channel was closed unexpectedly");
                if !state
                    .shutdown_signal
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    info!(
                        "[TASK PROCESSOR] ERROR: Channel closed while processor was still running!"
                    );
                }
                break;
            }
            Err(_timeout) => {
                // Timeout occurred - this is normal, just continue
                info!(
                    "[TASK PROCESSOR] No messages received in {:?}",
                    TIMEOUT_DURATION
                );
                continue;
            }
        }
    }

    info!("[TASK PROCESSOR] Status updater processor shutdown complete");
}





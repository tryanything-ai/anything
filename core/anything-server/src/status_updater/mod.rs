use crate::processor::db_calls::{create_task, update_flow_session_status, update_task_status};
use crate::types::task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus};
use crate::AppState;
use crate::metrics::METRICS;
use crate::websocket::WorkflowTestingUpdate;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant; 
use tokio::sync::mpsc::Receiver;
use tracing::{info, span, Instrument, Level};
use uuid::Uuid;

// Define the type of task operation
#[derive(Debug, Clone)]
pub enum Operation {
    UpdateTask {
        task_id: Uuid,
        account_id: Uuid,
        flow_session_id: Uuid,
        started_at: Option<DateTime<Utc>>,
        ended_at: Option<DateTime<Utc>>,
        status: TaskStatus,
        result: Option<Value>,
        context: Option<Value>,
        error: Option<Value>,
    },
    CreateTask {
        task_id: Uuid,
        account_id: Uuid,
        flow_session_id: Uuid,
        input: Task,
    },
    CompleteWorkflow {
        flow_session_id: Uuid,
        account_id: Uuid,
        status: FlowSessionStatus,
        trigger_status: TriggerSessionStatus,
    },
}

// Update the message struct to use the operation enum
#[derive(Debug, Clone)]
pub struct StatusUpdateMessage {
    pub operation: Operation,
}

fn get_operation_type(operation: &Operation) -> &'static str {
    match operation {
        Operation::UpdateTask { .. } => "update_task",
        Operation::CreateTask { .. } => "create_task", 
        Operation::CompleteWorkflow { .. } => "complete_workflow",
    }
}

pub async fn task_database_status_processor(
    state: Arc<AppState>,
    mut receiver: Receiver<StatusUpdateMessage>,
) {
    const MAX_RETRIES: u32 = 3;

    info!("[TASK PROCESSOR] Starting status updater processor");

    while let Some(message) = receiver.recv().await {
        // Check shutdown signal
        if state.shutdown_signal.load(std::sync::atomic::Ordering::SeqCst) {
            info!("[TASK PROCESSOR] Shutdown signal received, stopping status updater");
            break;
        }

        // Record queue wait time
        let queue_wait_start = Instant::now();
        let queue_wait_ms = queue_wait_start.elapsed().as_millis() as u64;
        METRICS.record_status_queue_wait_time(queue_wait_ms);

        // Record operation start and type
        let operation_type = get_operation_type(&message.operation);
        METRICS.record_status_operation_start(operation_type);

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

            let operation_start = Instant::now();
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
                            account_id: _, 
                            flow_session_id: _,
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
                        Operation::CreateTask { task_id, account_id: _, flow_session_id: _, input } => {
                            span!(Level::DEBUG, "create_task_db_call", task_id = %task_id).in_scope(|| {
                                create_task(state.clone(), input)
                            }).await
                        }
                        Operation::CompleteWorkflow {
                            flow_session_id,
                            account_id: _,
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
                        let operation_duration_ms = operation_start.elapsed().as_millis() as u64;
                        METRICS.record_status_operation_success(operation_duration_ms, operation_type);
                        
                        info!("[TASK PROCESSOR] Successfully processed update in {}ms", operation_duration_ms);
                        
                        // Broadcast WebSocket updates after successful database operations
                        broadcast_websocket_update(&state, &message.operation).await;
                        break;
                    }
                    Err(e) => {
                        let error_str = e.to_string();
                        
                        // Record retry and categorize error
                        METRICS.record_status_retry(operation_type);
                        METRICS.record_status_database_error();
                        METRICS.record_status_operation_failure(&error_str, operation_type);
                        
                        last_error = Some(e);
                        retries += 1;
                        
                        tracing::warn!(
                            error = %error_str,
                            retry_count = retries,
                            max_retries = MAX_RETRIES,
                            operation_type = %operation_kind,
                            "[TASK PROCESSOR] Database operation failed, retry {} of {}",
                            retries, MAX_RETRIES
                        );
                        
                        if retries < MAX_RETRIES {
                            info!("[TASK PROCESSOR] Retry {} of {}", retries, MAX_RETRIES);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500 * retries as u64)).await;
                        }
                    }
                }
            }

            if let Some(e) = last_error {
                if retries >= MAX_RETRIES {
                    METRICS.record_status_max_retries_exceeded(operation_type);
                    
                    tracing::error!(
                        error = %e,
                        retries = MAX_RETRIES,
                        operation_type = %operation_kind,
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

    info!("[TASK PROCESSOR] Status updater processor shutdown complete");
}

async fn get_current_tasks_for_session(state: &Arc<AppState>, flow_session_id: &Uuid) -> Option<serde_json::Value> {
    // TODO: Replace with SeaORM query once tasks entity is properly defined
    // For now, return None as placeholder to match migration approach
    println!("[STATUS UPDATER] TODO: Implement SeaORM query for flow_session_id: {}", flow_session_id);
    None
}

async fn broadcast_websocket_update(state: &Arc<AppState>, operation: &Operation) {
    match operation {
        Operation::UpdateTask {
            task_id,
            account_id,
            flow_session_id,
            status,
            result,
            error,
            ..
        } => {
            let update_type = match status {
                TaskStatus::Running => "task_updated",
                TaskStatus::Completed => "task_completed", 
                TaskStatus::Failed => "task_failed",
                _ => "task_updated",
            };

            // Fetch all current tasks for this flow session
            let tasks_data = get_current_tasks_for_session(state, flow_session_id).await;

            let update = WorkflowTestingUpdate {
                r#type: "workflow_update".to_string(),
                update_type: Some(update_type.to_string()),
                flow_session_id: flow_session_id.to_string(),
                data: Some(serde_json::json!({
                    "task_id": task_id,
                    "status": status,
                    "result": result,
                    "error": error
                })),
                tasks: tasks_data,
                complete: None,
            };

            state.websocket_manager.broadcast_workflow_testing_update(
                &account_id.to_string(),
                &flow_session_id.to_string(),
                update,
            );

            info!(
                "[WEBSOCKET] Broadcasted task update for task {} in session {} to account {}",
                task_id, flow_session_id, account_id
            );
        }
        Operation::CreateTask {
            task_id,
            account_id,
            flow_session_id,
            ..
        } => {
            // Fetch all current tasks for this flow session
            let tasks_data = get_current_tasks_for_session(state, flow_session_id).await;

            let update = WorkflowTestingUpdate {
                r#type: "workflow_update".to_string(),
                update_type: Some("task_created".to_string()),
                flow_session_id: flow_session_id.to_string(),
                data: Some(serde_json::json!({
                    "task_id": task_id
                })),
                tasks: tasks_data,
                complete: None,
            };

            state.websocket_manager.broadcast_workflow_testing_update(
                &account_id.to_string(),
                &flow_session_id.to_string(),
                update,
            );

            info!(
                "[WEBSOCKET] Broadcasted task creation for task {} in session {} to account {}",
                task_id, flow_session_id, account_id
            );
        }
        Operation::CompleteWorkflow {
            flow_session_id,
            account_id,
            status,
            trigger_status: _,
        } => {
            let update_type = match status {
                FlowSessionStatus::Completed => "workflow_completed",
                FlowSessionStatus::Failed => "workflow_failed",
                _ => "workflow_updated",
            };

            // Fetch final tasks for this flow session
            let tasks_data = get_current_tasks_for_session(state, flow_session_id).await;

            let update = WorkflowTestingUpdate {
                r#type: "workflow_update".to_string(),
                update_type: Some(update_type.to_string()),
                flow_session_id: flow_session_id.to_string(),
                data: Some(serde_json::json!({
                    "status": status
                })),
                tasks: tasks_data,
                complete: Some(matches!(status, FlowSessionStatus::Completed | FlowSessionStatus::Failed)),
            };

            state.websocket_manager.broadcast_workflow_testing_update(
                &account_id.to_string(),
                &flow_session_id.to_string(),
                update,
            );

            info!(
                "[WEBSOCKET] Broadcasted workflow completion for session {} to account {}",
                flow_session_id, account_id
            );
        }
    }
}





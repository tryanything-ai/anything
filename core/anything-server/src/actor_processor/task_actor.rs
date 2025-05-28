use crate::actor_processor::messages::ActorMessage;
use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, WorkflowExecutionContext};
use crate::processor::execute_task::{execute_task, TaskResult};
use crate::types::task_types::Task;
use crate::AppState;

use opentelemetry::KeyValue;
use postgrest::Postgrest;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

/// Actor for executing individual tasks
pub struct TaskActor {
    id: Uuid,
    state: Arc<AppState>,
    client: Postgrest,
    span_factory: EnhancedSpanFactory,
    metrics_labels: Vec<KeyValue>,
}

impl TaskActor {
    pub fn new(
        id: Uuid,
        state: Arc<AppState>,
        client: Postgrest,
        span_factory: EnhancedSpanFactory,
        metrics_labels: Vec<KeyValue>,
    ) -> Self {
        Self {
            id,
            state,
            client,
            span_factory,
            metrics_labels,
        }
    }

    pub async fn run(mut self, mut receiver: mpsc::Receiver<ActorMessage>) {
        info!("[TASK_ACTOR_{}] Starting task actor", self.id);

        while let Some(message) = receiver.recv().await {
            match message {
                ActorMessage::ExecuteTask {
                    task,
                    respond_to,
                    context,
                } => {
                    let result = self.handle_execute_task(task, context).await;
                    let _ = respond_to.send(result);
                }
                ActorMessage::Shutdown => {
                    info!("[TASK_ACTOR_{}] Shutting down task actor", self.id);
                    break;
                }
                _ => {
                    warn!("[TASK_ACTOR_{}] Received unexpected message type", self.id);
                }
            }
        }

        info!("[TASK_ACTOR_{}] Task actor shutdown complete", self.id);
    }

    #[instrument(skip(self, task, context), fields(
        actor_id = %self.id,
        task_id = %task.task_id,
        plugin_name = ?task.plugin_name
    ))]
    async fn handle_execute_task(
        &self,
        task: Task,
        context: WorkflowExecutionContext,
    ) -> TaskResult {
        let task_span = self.create_task_execution_span(
            task.task_id,
            task.plugin_name.as_ref().map(|p| p.as_str()),
            context.flow_session_id,
        );
        let _task_guard = task_span.enter();

        context.record_stage("executing_task");

        let start_time = Instant::now();
        info!("[TASK_ACTOR_{}] Executing task {}", self.id, task.task_id);

        // Update task status to running
        let update_running_message = crate::status_updater::StatusUpdateMessage {
            operation: crate::status_updater::Operation::UpdateTask {
                task_id: task.task_id,
                started_at: Some(chrono::Utc::now()),
                ended_at: None,
                status: crate::types::task_types::TaskStatus::Running,
                result: None,
                context: None,
                error: None,
            },
        };

        if let Err(e) = self
            .state
            .task_updater_sender
            .send(update_running_message)
            .await
        {
            warn!(
                "[TASK_ACTOR_{}] Failed to send running status update for task {}: {}",
                self.id, task.task_id, e
            );
        }

        // Execute the task with timeout
        let task_timeout = Duration::from_secs(300); // 5 minutes timeout
        let result = timeout(
            task_timeout,
            execute_task(self.state.clone(), &self.client, &task, None),
        )
        .await;

        let execution_duration = start_time.elapsed();
        METRICS.record_task_execution_time(execution_duration, &self.metrics_labels);

        let end_time = chrono::Utc::now();

        match result {
            Ok(task_result) => {
                match &task_result {
                    Ok((result_value, context_value, started_at, ended_at)) => {
                        info!(
                            "[TASK_ACTOR_{}] Task {} completed successfully in {:?}",
                            self.id, task.task_id, execution_duration
                        );
                        context.record_success();

                        // Update task status to completed
                        let update_completed_message = crate::status_updater::StatusUpdateMessage {
                            operation: crate::status_updater::Operation::UpdateTask {
                                task_id: task.task_id,
                                started_at: Some(*started_at),
                                ended_at: Some(*ended_at),
                                status: crate::types::task_types::TaskStatus::Completed,
                                result: result_value.clone(),
                                context: Some(context_value.clone()),
                                error: None,
                            },
                        };

                        if let Err(e) = self
                            .state
                            .task_updater_sender
                            .send(update_completed_message)
                            .await
                        {
                            warn!(
                                "[TASK_ACTOR_{}] Failed to send completed status update for task {}: {}",
                                self.id, task.task_id, e
                            );
                        }
                    }
                    Err(e) => {
                        error!(
                            "[TASK_ACTOR_{}] Task {} failed: {:?}",
                            self.id, task.task_id, e
                        );
                        context.record_error(&format!("Task execution failed: {:?}", e));

                        // Update task status to failed
                        let update_failed_message = crate::status_updater::StatusUpdateMessage {
                            operation: crate::status_updater::Operation::UpdateTask {
                                task_id: task.task_id,
                                started_at: Some(
                                    chrono::Utc::now()
                                        - chrono::Duration::from_std(execution_duration)
                                            .unwrap_or_default(),
                                ),
                                ended_at: Some(end_time),
                                status: crate::types::task_types::TaskStatus::Failed,
                                result: None,
                                context: None,
                                error: Some(e.error.clone()),
                            },
                        };

                        if let Err(send_err) = self
                            .state
                            .task_updater_sender
                            .send(update_failed_message)
                            .await
                        {
                            warn!(
                                "[TASK_ACTOR_{}] Failed to send failed status update for task {}: {}",
                                self.id, task.task_id, send_err
                            );
                        }
                    }
                }
                task_result
            }
            Err(_) => {
                error!(
                    "[TASK_ACTOR_{}] Task {} timed out after {:?}",
                    self.id, task.task_id, task_timeout
                );
                context.record_error("Task execution timeout");

                let timeout_error = serde_json::json!({
                    "message": format!("Task {} timed out after {:?}", task.task_id, task_timeout)
                });

                // Update task status to failed due to timeout
                let update_timeout_message = crate::status_updater::StatusUpdateMessage {
                    operation: crate::status_updater::Operation::UpdateTask {
                        task_id: task.task_id,
                        started_at: Some(
                            chrono::Utc::now()
                                - chrono::Duration::from_std(execution_duration)
                                    .unwrap_or_default(),
                        ),
                        ended_at: Some(end_time),
                        status: crate::types::task_types::TaskStatus::Failed,
                        result: None,
                        context: None,
                        error: Some(timeout_error.clone()),
                    },
                };

                if let Err(e) = self
                    .state
                    .task_updater_sender
                    .send(update_timeout_message)
                    .await
                {
                    warn!(
                        "[TASK_ACTOR_{}] Failed to send timeout status update for task {}: {}",
                        self.id, task.task_id, e
                    );
                }

                Err(crate::processor::execute_task::TaskError {
                    error: timeout_error,
                    context: serde_json::json!({}),
                })
            }
        }
    }

    fn create_task_execution_span(
        &self,
        task_id: Uuid,
        plugin_name: Option<&str>,
        flow_session_id: Uuid,
    ) -> tracing::Span {
        tracing::info_span!(
            "task_execution",
            task_id = %task_id,
            plugin_name = plugin_name,
            flow_session_id = %flow_session_id,
            actor_id = %self.id
        )
    }
}

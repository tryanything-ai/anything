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
                    in_memory_tasks,
                } => {
                    let result = self
                        .handle_execute_task(task, context, in_memory_tasks.as_ref())
                        .await;
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

    #[instrument(skip(self, task, context, in_memory_tasks), fields(
        actor_id = %self.id,
        task_id = %task.task_id,
        plugin_name = ?task.plugin_name
    ))]
    async fn handle_execute_task(
        &self,
        task: Task,
        context: WorkflowExecutionContext,
        in_memory_tasks: Option<&std::collections::HashMap<uuid::Uuid, Task>>,
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

        // 🚀 TASK STARTING - Would normally update task status to running in database
        info!(
            "🚀 TASK STARTING: {} is now RUNNING (skipping database update for debugging)",
            task.task_id
        );

        // Execute the task with timeout
        let task_timeout = Duration::from_secs(300); // 5 minutes timeout
        let result = timeout(
            task_timeout,
            execute_task(self.state.clone(), &self.client, &task, in_memory_tasks),
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

                        // ✅ TASK COMPLETED - Would normally update task status to completed in database
                        info!("✅ TASK COMPLETED: {} finished successfully in {:?} (skipping database update for debugging)", task.task_id, execution_duration);
                    }
                    Err(e) => {
                        error!(
                            "[TASK_ACTOR_{}] Task {} failed: {:?}",
                            self.id, task.task_id, e
                        );
                        context.record_error(&format!("Task execution failed: {:?}", e));

                        // ❌ TASK FAILED - Would normally update task status to failed in database
                        info!("❌ TASK FAILED: {} encountered an error: {:?} (skipping database update for debugging)", task.task_id, e);
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

                // ⏰ TASK TIMEOUT - Would normally update task status to failed due to timeout in database
                info!("⏰ TASK TIMEOUT: {} timed out after {:?} (skipping database update for debugging)", task.task_id, task_timeout);

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

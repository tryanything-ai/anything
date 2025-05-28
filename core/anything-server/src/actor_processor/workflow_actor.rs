use crate::actor_processor::actor_pool::TaskActorPool;
use crate::actor_processor::messages::ActorMessage;
use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext};
use crate::processor::execute_task::TaskResult;
use crate::processor::processor::ProcessorMessage;
use crate::types::task_types::Task;
use crate::AppState;

use opentelemetry::KeyValue;
use postgrest::Postgrest;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

/// Actor for orchestrating workflow execution
pub struct WorkflowActor {
    id: Uuid,
    state: Arc<AppState>,
    client: Postgrest,
    task_actor_pool: TaskActorPool,
    span_factory: EnhancedSpanFactory,
    metrics_labels: Vec<KeyValue>,
}

impl WorkflowActor {
    pub fn new(
        id: Uuid,
        state: Arc<AppState>,
        client: Postgrest,
        task_actor_pool: TaskActorPool,
        span_factory: EnhancedSpanFactory,
        metrics_labels: Vec<KeyValue>,
    ) -> Self {
        Self {
            id,
            state,
            client,
            task_actor_pool,
            span_factory,
            metrics_labels,
        }
    }

    pub async fn run(mut self, mut receiver: mpsc::Receiver<ActorMessage>) {
        info!("[WORKFLOW_ACTOR_{}] Starting workflow actor", self.id);

        while let Some(message) = receiver.recv().await {
            match message {
                ActorMessage::ExecuteWorkflow {
                    message,
                    respond_to,
                } => {
                    let result = self.handle_execute_workflow(message).await;
                    let _ = respond_to.send(result);
                }
                ActorMessage::Shutdown => {
                    info!("[WORKFLOW_ACTOR_{}] Shutting down workflow actor", self.id);
                    break;
                }
                _ => {
                    warn!(
                        "[WORKFLOW_ACTOR_{}] Received unexpected message type",
                        self.id
                    );
                }
            }
        }

        info!(
            "[WORKFLOW_ACTOR_{}] Workflow actor shutdown complete",
            self.id
        );
    }

    #[instrument(skip(self, message), fields(
        actor_id = %self.id,
        flow_session_id = %message.flow_session_id,
        workflow_id = %message.workflow_id
    ))]
    async fn handle_execute_workflow(
        &self,
        message: ProcessorMessage,
    ) -> Result<(), ProcessorError> {
        let workflow_span = self.span_factory.create_workflow_execution_span(
            message.flow_session_id,
            message.task_id,
            message
                .trigger_task
                .as_ref()
                .map(|t| format!("{:?}", t.r#type))
                .as_deref(),
        );
        let _workflow_guard = workflow_span.enter();

        let start_time = Instant::now();
        info!(
            "[WORKFLOW_ACTOR_{}] Starting workflow execution for {}",
            self.id, message.flow_session_id
        );

        METRICS.record_workflow_started(&self.metrics_labels);

        // Create execution context
        let context = WorkflowExecutionContext::new(
            message.flow_session_id,
            message.workflow_id,
            message.task_id,
            workflow_span.clone(),
        );

        context.record_stage("processing_workflow");

        // Process the workflow using actor-based task execution
        let result = self.process_workflow_with_actors(message, &context).await;

        let execution_duration = start_time.elapsed();
        METRICS.record_workflow_completed(execution_duration, &self.metrics_labels);

        match result {
            Ok(_) => {
                info!(
                    "[WORKFLOW_ACTOR_{}] Workflow {} completed successfully in {:?}",
                    self.id, context.flow_session_id, execution_duration
                );
                context.record_success();
                Ok(())
            }
            Err(e) => {
                error!(
                    "[WORKFLOW_ACTOR_{}] Workflow {} failed: {}",
                    self.id, context.flow_session_id, e
                );
                context.record_error(&e.to_string());
                Err(ProcessorError::WorkflowExecutionError(e.to_string()))
            }
        }
    }

    async fn process_workflow_with_actors(
        &self,
        message: ProcessorMessage,
        context: &WorkflowExecutionContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Extract actions from the workflow definition and convert them to tasks
        let actions = &message.workflow_definition.actions;

        if actions.is_empty() {
            info!("[WORKFLOW_ACTOR_{}] No actions to execute", self.id);
            return Ok(());
        }

        // Convert actions to tasks and create them in the database
        let mut tasks = Vec::new();
        for (index, action) in actions.iter().enumerate() {
            let task = self
                .convert_action_to_task(action, &message, index as i32)
                .await?;

            // Create task in database via status updater
            let create_task_message = crate::status_updater::StatusUpdateMessage {
                operation: crate::status_updater::Operation::CreateTask {
                    task_id: task.task_id,
                    input: task.clone(),
                },
            };

            if let Err(e) = self
                .state
                .task_updater_sender
                .send(create_task_message)
                .await
            {
                error!(
                    "[WORKFLOW_ACTOR_{}] Failed to send create task message for {}: {}",
                    self.id, task.task_id, e
                );
                return Err(format!("Failed to create task in database: {}", e).into());
            }

            info!(
                "[WORKFLOW_ACTOR_{}] Created task {} in database",
                self.id, task.task_id
            );

            tasks.push(task);
        }

        // Track completed tasks and their results
        let completed_tasks = Arc::new(RwLock::new(HashMap::<Uuid, TaskResult>::new()));

        // For now, execute all tasks in parallel (simplified approach)
        // In a more sophisticated implementation, you would implement proper dependency resolution
        let mut pending_tasks = Vec::new();

        for task in &tasks {
            let task_context = WorkflowExecutionContext::new(
                context.flow_session_id,
                context.workflow_id,
                Some(task.task_id),
                context.span.clone(),
            );

            // Execute task using actor pool
            let task_future = self
                .task_actor_pool
                .execute_task(task.clone(), task_context);
            pending_tasks.push((task.task_id, task_future));
        }

        // Wait for all tasks to complete
        let mut results = Vec::new();
        for (task_id, task_future) in pending_tasks {
            match task_future.await {
                Ok(result) => {
                    info!("[WORKFLOW_ACTOR_{}] Task {} completed", self.id, task_id);
                    completed_tasks
                        .write()
                        .await
                        .insert(task_id, result.clone());
                    results.push(result);
                }
                Err(e) => {
                    error!(
                        "[WORKFLOW_ACTOR_{}] Task {} failed: {:?}",
                        self.id, task_id, e
                    );

                    // Send workflow failure status
                    let fail_workflow_message = crate::status_updater::StatusUpdateMessage {
                        operation: crate::status_updater::Operation::CompleteWorkflow {
                            flow_session_id: context.flow_session_id,
                            status: crate::types::task_types::FlowSessionStatus::Failed,
                            trigger_status: crate::types::task_types::TriggerSessionStatus::Failed,
                        },
                    };

                    if let Err(send_err) = self
                        .state
                        .task_updater_sender
                        .send(fail_workflow_message)
                        .await
                    {
                        warn!(
                            "[WORKFLOW_ACTOR_{}] Failed to send workflow failure status for {}: {}",
                            self.id, context.flow_session_id, send_err
                        );
                    } else {
                        info!(
                            "[WORKFLOW_ACTOR_{}] Workflow {} marked as failed due to task failure",
                            self.id, context.flow_session_id
                        );
                    }

                    return Err(format!("Task {} failed: {:?}", task_id, e).into());
                }
            }
        }

        info!(
            "[WORKFLOW_ACTOR_{}] All {} tasks completed successfully",
            self.id,
            tasks.len()
        );

        // Send workflow completion status
        let complete_workflow_message = crate::status_updater::StatusUpdateMessage {
            operation: crate::status_updater::Operation::CompleteWorkflow {
                flow_session_id: context.flow_session_id,
                status: crate::types::task_types::FlowSessionStatus::Completed,
                trigger_status: crate::types::task_types::TriggerSessionStatus::Completed,
            },
        };

        if let Err(e) = self
            .state
            .task_updater_sender
            .send(complete_workflow_message)
            .await
        {
            warn!(
                "[WORKFLOW_ACTOR_{}] Failed to send workflow completion status for {}: {}",
                self.id, context.flow_session_id, e
            );
        } else {
            info!(
                "[WORKFLOW_ACTOR_{}] Workflow {} marked as completed",
                self.id, context.flow_session_id
            );
        }

        Ok(())
    }

    async fn convert_action_to_task(
        &self,
        action: &crate::types::action_types::Action,
        message: &ProcessorMessage,
        processing_order: i32,
    ) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
        use crate::types::task_types::{Stage, TaskConfig};

        let task = Task::builder()
            .account_id(message.workflow_version.account_id)
            .flow_id(message.workflow_id)
            .flow_version_id(message.workflow_version.flow_version_id)
            .action_label(action.label.clone())
            .trigger_id(
                message
                    .trigger_task
                    .as_ref()
                    .map(|t| t.task_id.to_string())
                    .unwrap_or_default(),
            )
            .flow_session_id(message.flow_session_id)
            .trigger_session_id(message.trigger_session_id)
            .action_id(action.action_id.clone())
            .r#type(action.r#type.clone())
            .plugin_name(action.plugin_name.clone())
            .plugin_version(action.plugin_version.clone())
            .stage(if message.workflow_version.published {
                Stage::Production
            } else {
                Stage::Testing
            })
            .processing_order(processing_order)
            .config(TaskConfig {
                inputs: Some(action.inputs.clone().unwrap_or_default()),
                inputs_schema: action.inputs_schema.clone(),
                plugin_config: Some(action.plugin_config.clone()),
                plugin_config_schema: Some(action.plugin_config_schema.clone()),
            })
            .build()
            .map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?;

        Ok(task)
    }
}

use crate::actor_processor::actor_pool::TaskActorPool;
use crate::actor_processor::dependency_resolver::DependencyGraph;
use crate::actor_processor::messages::ActorMessage;
use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext};
use crate::processor::execute_task::TaskResult;
use crate::processor::processor::ProcessorMessage;
use crate::types::task_types::Task;
use crate::AppState;

use opentelemetry::KeyValue;
use postgrest::Postgrest;
use std::collections::{HashMap, HashSet};
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
        // Extract actions from the workflow definition
        let actions = &message.workflow_definition.actions;

        if actions.is_empty() {
            info!("[WORKFLOW_ACTOR_{}] No actions to execute", self.id);
            return Ok(());
        }

        // Build dependency graph
        let dependency_graph = DependencyGraph::new(&message.workflow_definition);
        info!(
            "[WORKFLOW_ACTOR_{}] Built dependency graph with execution order: {:?}",
            self.id, dependency_graph.execution_order
        );

        // Track completed tasks (with their results for bundling)
        let completed_tasks = Arc::new(RwLock::new(HashMap::<Uuid, Task>::new()));

        // Track currently running tasks
        let running_tasks = Arc::new(RwLock::new(HashSet::<String>::new()));

        // Process tasks in dependency order
        loop {
            // Get ready actions that can be executed now
            let ready_actions = {
                let completed = completed_tasks.read().await;
                let running = running_tasks.read().await;
                dependency_graph.get_ready_actions(actions, &completed, &running)
            };

            if ready_actions.is_empty() {
                // Check if all tasks are completed
                let completed = completed_tasks.read().await;
                let total_completed = completed.len();

                if total_completed == actions.len() {
                    info!(
                        "[WORKFLOW_ACTOR_{}] All {} tasks completed successfully",
                        self.id, total_completed
                    );
                    break;
                } else {
                    // Check if we have any running tasks
                    let running = running_tasks.read().await;
                    if running.is_empty() {
                        // No ready actions and no running tasks - this indicates a problem
                        let remaining_actions: Vec<String> = actions
                            .iter()
                            .filter(|action| {
                                !completed
                                    .values()
                                    .any(|task| task.action_id == action.action_id)
                            })
                            .map(|action| action.action_id.clone())
                            .collect();

                        error!(
                            "[WORKFLOW_ACTOR_{}] Workflow stuck! No ready actions and no running tasks. Remaining: {:?}",
                            self.id, remaining_actions
                        );
                        return Err(
                            "Workflow execution stuck - possible circular dependency".into()
                        );
                    } else {
                        // We have running tasks, wait a bit and check again
                        info!(
                            "[WORKFLOW_ACTOR_{}] No ready actions, but {} tasks still running. Waiting...",
                            self.id, running.len()
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        continue;
                    }
                }
            }

            // Execute ready actions in parallel
            let mut task_futures = Vec::new();

            for action in ready_actions {
                // Mark as running
                {
                    let mut running = running_tasks.write().await;
                    running.insert(action.action_id.clone());
                }

                // Convert action to task
                let task = self
                    .convert_action_to_task(&action, &message, 0) // processing_order not used in dependency-based execution
                    .await?;

                // 📝 TASK CREATION - Would normally create task in database
                info!("📝 TASK CREATION: Creating task {} for action {} (skipping database creation for debugging)", task.task_id, action.action_id);

                info!(
                    "[WORKFLOW_ACTOR_{}] Created and executing task {} for action {}",
                    self.id, task.task_id, action.action_id
                );

                // Create task context
                let task_context = WorkflowExecutionContext::new(
                    context.flow_session_id,
                    context.workflow_id,
                    Some(task.task_id),
                    context.span.clone(),
                );

                // Execute task using actor pool with in-memory tasks for bundling
                let completed_tasks_clone = Arc::clone(&completed_tasks);
                let running_tasks_clone = Arc::clone(&running_tasks);
                let action_id = action.action_id.clone();
                let task_id = task.task_id;
                let task_actor_pool = self.task_actor_pool.clone();

                let task_future = tokio::spawn(async move {
                    // Get in-memory tasks for bundling
                    let in_memory_tasks = {
                        let completed = completed_tasks_clone.read().await;
                        completed.clone()
                    };

                    // Execute task with bundled context from previous tasks
                    let result = task_actor_pool
                        .execute_task(task, task_context, Some(&in_memory_tasks))
                        .await;

                    // Remove from running tasks
                    {
                        let mut running = running_tasks_clone.write().await;
                        running.remove(&action_id);
                    }

                    (task_id, action_id, result)
                });

                task_futures.push(task_future);
            }

            // Wait for this batch of tasks to complete
            for task_future in task_futures {
                match task_future.await {
                    Ok((task_id, action_id, result)) => {
                        match result {
                            Ok(task_result) => {
                                info!(
                                    "[WORKFLOW_ACTOR_{}] Task {} (action {}) completed successfully",
                                    self.id, task_id, action_id
                                );

                                // Store completed task with its result for future bundling
                                // Create a minimal task for in-memory storage
                                //TODO: this seems kinda dangerous since some of this data is false!
                                let mut completed_task = Task {
                                    task_id,
                                    account_id: Uuid::new_v4(), // Placeholder
                                    task_status: crate::types::task_types::TaskStatus::Completed,
                                    flow_id: context.workflow_id,
                                    flow_version_id: Uuid::new_v4(), // Placeholder
                                    action_label: "".to_string(),    // Placeholder
                                    trigger_id: "".to_string(),      // Placeholder
                                    trigger_session_id: Uuid::new_v4(), // Placeholder
                                    trigger_session_status:
                                        crate::types::task_types::TriggerSessionStatus::Completed,
                                    flow_session_id: context.flow_session_id,
                                    flow_session_status:
                                        crate::types::task_types::FlowSessionStatus::Running,
                                    action_id: action_id.clone(),
                                    r#type: crate::types::action_types::ActionType::Action,
                                    plugin_name: None,
                                    plugin_version: None,
                                    stage: crate::types::task_types::Stage::Production,
                                    test_config: None,
                                    config: crate::types::task_types::TaskConfig {
                                        inputs: None,
                                        inputs_schema: None,
                                        plugin_config: None,
                                        plugin_config_schema: None,
                                    },
                                    context: None,
                                    started_at: None,
                                    ended_at: None,
                                    debug_result: None,
                                    result: None,
                                    error: None,
                                    archived: false,
                                    updated_at: None,
                                    created_at: None,
                                    updated_by: None,
                                    created_by: None,
                                    processing_order: 0,
                                };

                                // Extract result from TaskResult tuple
                                if let Ok((result_value, context_value, _, _)) = &task_result {
                                    completed_task.result = result_value.clone();
                                    completed_task.context = Some(context_value.clone());
                                }

                                {
                                    let mut completed = completed_tasks.write().await;
                                    completed.insert(task_id, completed_task);
                                }
                            }
                            Err(e) => {
                                error!(
                                    "[WORKFLOW_ACTOR_{}] Task {} (action {}) failed: {:?}",
                                    self.id, task_id, action_id, e
                                );

                                // 💥 WORKFLOW FAILURE - Would normally send workflow failure status to database
                                info!("💥 WORKFLOW FAILURE: Workflow {} failed due to task {} failure (skipping database update for debugging)", context.flow_session_id, task_id);
                                //TODO: we should probably send a failure status update for the task as well

                                return Err(format!("Task {} failed: {:?}", task_id, e).into());
                            }
                        }
                    }
                    Err(join_error) => {
                        error!(
                            "[WORKFLOW_ACTOR_{}] Task future panicked: {}",
                            self.id, join_error
                        );
                        return Err(format!("Task execution panicked: {}", join_error).into());
                    }
                }
            }
        }

        // 🎉 WORKFLOW COMPLETED - Would normally send workflow completion status to database
        info!("🎉 WORKFLOW COMPLETED: Workflow {} finished successfully with all tasks completed (skipping database update for debugging)", context.flow_session_id);

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

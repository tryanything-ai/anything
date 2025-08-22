use crate::actor_processor::actor_pool::TaskActorPool;
use crate::actor_processor::dependency_resolver::DependencyGraph;
use crate::actor_processor::messages::ActorMessage;
use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext};

use crate::processor::processor::ProcessorMessage;
use crate::status_updater::{Operation, StatusUpdateMessage};
use crate::types::task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus};
use crate::AppState;

use chrono::Utc;
use opentelemetry::KeyValue;
use sea_orm::DatabaseConnection;
use serde_json::{self, Value};
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
    #[allow(dead_code)]
    client: Arc<DatabaseConnection>,
    task_actor_pool: TaskActorPool,
    span_factory: EnhancedSpanFactory,
    metrics_labels: Vec<KeyValue>,
}

impl WorkflowActor {
    pub fn new(
        id: Uuid,
        state: Arc<AppState>,
        client: Arc<DatabaseConnection>,
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

    pub async fn run(self, mut receiver: mpsc::Receiver<ActorMessage>) {
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

    // #[instrument(skip(self, message), fields(
    //     actor_id = %self.id,
    //     flow_session_id = %message.flow_session_id,
    //     workflow_id = %message.workflow_id
    // ))]
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

        // Track failed filter tasks that should stop dependent actions
        let failed_filters = Arc::new(RwLock::new(HashSet::<String>::new()));

        // Process tasks in dependency order
        loop {
            // Get ready actions that can be executed now
            let ready_actions = {
                let completed = completed_tasks.read().await;
                let running = running_tasks.read().await;
                let failed = failed_filters.read().await;
                
                let mut candidate_actions = dependency_graph.get_ready_actions(actions, &completed, &running);
                
                // Filter out actions that depend on failed filters
                candidate_actions.retain(|action| {
                    // Check if this action depends on any failed filters
                    let depends_on_failed_filter = dependency_graph.dependencies
                        .get(&action.action_id)
                        .map(|deps| {
                            deps.iter().any(|dep_action_id| failed.contains(dep_action_id))
                        })
                        .unwrap_or(false);
                    
                    if depends_on_failed_filter {
                        info!(
                            "[WORKFLOW_ACTOR_{}] Skipping action {} because it depends on a failed filter",
                            self.id, action.action_id
                        );
                        false
                    } else {
                        true
                    }
                });
                
                candidate_actions
            };

            if ready_actions.is_empty() {
                // Check if all runnable tasks are completed
                let completed = completed_tasks.read().await;
                let failed = failed_filters.read().await;
                let total_completed = completed.len();
                
                // Count actions that are blocked by failed filters (will never run)
                let blocked_actions = actions.iter().filter(|action| {
                    // Skip if already completed
                    if completed.values().any(|task| task.action_id == action.action_id) {
                        return false;
                    }
                    
                    // Check if this action depends on any failed filters
                    dependency_graph.dependencies
                        .get(&action.action_id)
                        .map(|deps| {
                            deps.iter().any(|dep_action_id| failed.contains(dep_action_id))
                        })
                        .unwrap_or(false)
                }).count();

                let total_runnable = actions.len() - blocked_actions;
                
                if total_completed == total_runnable {
                    info!(
                        "[WORKFLOW_ACTOR_{}] All {} runnable tasks completed successfully ({} blocked by failed filters)",
                        self.id, total_completed, blocked_actions
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

                // Send task creation message to database
                let create_task_message = StatusUpdateMessage {
                    operation: Operation::CreateTask {
                        task_id: task.task_id,
                        account_id: message.workflow_version.account_id,
                        flow_session_id: context.flow_session_id,
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
                    return Err(format!("Failed to send task creation message: {}", e).into());
                }

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

                // Capture data needed for task completion handling
                let action_data = (
                    action.label.clone(),
                    action.r#type.clone(),
                    action.plugin_name.clone(),
                    action.plugin_version.clone(),
                    action.inputs.clone().unwrap_or_default(),
                    action.inputs_schema.clone(),
                    action.plugin_config.clone(),
                    action.plugin_config_schema.clone(),
                );
                let message_data = (
                    message.workflow_version.account_id,
                    message.workflow_version.flow_version_id,
                    message
                        .trigger_task
                        .as_ref()
                        .map(|t| t.task_id.to_string())
                        .unwrap_or_default(),
                    message.trigger_session_id,
                    message.workflow_version.published,
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

                    (task_id, action_id, result, action_data, message_data)
                });

                task_futures.push(task_future);
            }

            // Wait for this batch of tasks to complete
            for task_future in task_futures {
                match task_future.await {
                    Ok((task_id, action_id, result, action_data, message_data)) => {
                        match result {
                            Ok(task_result) => {
                                info!(
                                    "[WORKFLOW_ACTOR_{}] Task {} (action {}) completed successfully",
                                    self.id, task_id, action_id
                                );

                                // Extract result and context from TaskResult tuple
                                let (result_value, context_value, started_at, ended_at) =
                                    match &task_result {
                                        Ok((result, context, start, end)) => (
                                            result.clone(),
                                            Some(context.clone()),
                                            Some(*start),
                                            Some(*end),
                                        ),
                                        Err(_) => (None, None, None, None),
                                    };

                                // Send task completion update to database
                                let task_update_message = StatusUpdateMessage {
                                    operation: Operation::UpdateTask {
                                        task_id,
                                        account_id: message_data.0, // account_id from message_data
                                        flow_session_id: context.flow_session_id,
                                        status: TaskStatus::Completed,
                                        result: result_value.clone(),
                                        context: context_value.clone(),
                                        error: None,
                                        started_at,
                                        ended_at,
                                    },
                                };

                                if let Err(e) = self
                                    .state
                                    .task_updater_sender
                                    .send(task_update_message)
                                    .await
                                {
                                    error!(
                                        "[WORKFLOW_ACTOR_{}] Failed to send task completion update for {}: {}",
                                        self.id, task_id, e
                                    );
                                }

                                // Store completed task with its result for future bundling
                                // Create a minimal task for in-memory storage
                                let (
                                    action_label,
                                    action_type,
                                    plugin_name,
                                    plugin_version,
                                    inputs,
                                    inputs_schema,
                                    plugin_config,
                                    plugin_config_schema,
                                ) = action_data;
                                let (
                                    account_id,
                                    flow_version_id,
                                    trigger_id,
                                    trigger_session_id,
                                    published,
                                ) = message_data;

                                // Clone values before they get moved into the Task struct
                                let plugin_name_for_filter_check = plugin_name.clone();
                                let result_value_for_filter_check = result_value.clone();

                                info!(
                                    "[WORKFLOW_ACTOR_{}] Completed task {} (action {}) with result {:?}",
                                    self.id, task_id, action_id, result_value
                                );

                                let completed_task = Task {
                                    task_id,
                                    account_id,
                                    task_status: TaskStatus::Completed,
                                    flow_id: context.workflow_id,
                                    flow_version_id,
                                    action_label,
                                    trigger_id,
                                    trigger_session_id,
                                    trigger_session_status: TriggerSessionStatus::Completed,
                                    flow_session_id: context.flow_session_id,
                                    flow_session_status: FlowSessionStatus::Running,
                                    action_id: action_id.clone(),
                                    r#type: action_type,
                                    plugin_name: Some(plugin_name),
                                    plugin_version: Some(plugin_version),
                                    stage: if published {
                                        crate::types::task_types::Stage::Production
                                    } else {
                                        crate::types::task_types::Stage::Testing
                                    },
                                    test_config: None,
                                    config: crate::types::task_types::TaskConfig {
                                        inputs: Some(inputs),
                                        inputs_schema,
                                        plugin_config: Some(plugin_config),
                                        plugin_config_schema: Some(plugin_config_schema),
                                    },
                                    context: context_value,
                                    started_at,
                                    ended_at,
                                    debug_result: None,
                                    result: result_value,
                                    error: None,
                                    archived: false,
                                    updated_at: None,
                                    created_at: None,
                                    updated_by: None,
                                    created_by: None,
                                    processing_order: 0,
                                };

                                // Check if this is a filter task that failed (returned null)
                                // The filter plugin already handles truthiness evaluation and returns null for failed filters
                                if plugin_name_for_filter_check.as_str() == "@anything/filter" {
                                    let should_stop_path = match &result_value_for_filter_check {
                                        Some(Value::Null) => {
                                            info!(
                                                "[WORKFLOW_ACTOR_{}] Filter task {} failed, stopping dependent actions",
                                                self.id, task_id
                                            );
                                            true
                                        }
                                        Some(_) => {
                                            info!(
                                                "[WORKFLOW_ACTOR_{}] Filter task {} passed, continuing execution",
                                                self.id, task_id
                                            );
                                            false
                                        }
                                        None => {
                                            info!(
                                                "[WORKFLOW_ACTOR_{}] Filter task {} returned no result, stopping dependent actions",
                                                self.id, task_id
                                            );
                                            true
                                        }
                                    };

                                    // If the filter failed, add it to the failed filters set
                                    if should_stop_path {
                                        let mut failed = failed_filters.write().await;
                                        failed.insert(action_id.clone());
                                        info!(
                                            "[WORKFLOW_ACTOR_{}] Added failed filter {} to failed_filters set",
                                            self.id, action_id
                                        );
                                    }
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

                                // Send task failure update to database
                                let task_error_message = StatusUpdateMessage {
                                    operation: Operation::UpdateTask {
                                        task_id,
                                        account_id: message_data.0, // account_id from message_data
                                        flow_session_id: context.flow_session_id,
                                        status: TaskStatus::Failed,
                                        result: None,
                                        context: None,
                                        error: Some(serde_json::json!({
                                            "error": e.to_string(),
                                            "error_type": "task_execution_error"
                                        })),
                                        started_at: None,
                                        ended_at: Some(Utc::now()),
                                    },
                                };

                                if let Err(send_err) = self
                                    .state
                                    .task_updater_sender
                                    .send(task_error_message)
                                    .await
                                {
                                    error!(
                                        "[WORKFLOW_ACTOR_{}] Failed to send task error update for {}: {}",
                                        self.id, task_id, send_err
                                    );
                                }

                                // Send workflow failure status to database
                                let workflow_failure_message = StatusUpdateMessage {
                                    operation: Operation::CompleteWorkflow {
                                        flow_session_id: context.flow_session_id,
                                        account_id: message_data.0, // account_id from message_data
                                        status: FlowSessionStatus::Failed,
                                        trigger_status: TriggerSessionStatus::Failed,
                                    },
                                };

                                if let Err(send_err) = self
                                    .state
                                    .task_updater_sender
                                    .send(workflow_failure_message)
                                    .await
                                {
                                    error!(
                                        "[WORKFLOW_ACTOR_{}] Failed to send workflow failure update: {}",
                                        self.id, send_err
                                    );
                                }

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

        // Send workflow completion status to database
        let workflow_completion_message = StatusUpdateMessage {
            operation: Operation::CompleteWorkflow {
                flow_session_id: context.flow_session_id,
                account_id: message.workflow_version.account_id,
                status: FlowSessionStatus::Completed,
                trigger_status: TriggerSessionStatus::Completed,
            },
        };

        if let Err(e) = self
            .state
            .task_updater_sender
            .send(workflow_completion_message)
            .await
        {
            error!(
                "[WORKFLOW_ACTOR_{}] Failed to send workflow completion update: {}",
                self.id, e
            );
        }

        info!(
            "[WORKFLOW_ACTOR_{}] Workflow {} completed successfully with all tasks completed",
            self.id, context.flow_session_id
        );

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

use crate::AppState;
use opentelemetry::KeyValue;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinSet;
use tracing::{error, info, instrument, warn, Span};
use uuid::Uuid;

use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext};
use crate::processor::flow_session_cache::FlowSessionData;
use crate::processor::processor::ProcessorMessage;
use crate::processor::processor_utils::create_task;

use crate::processor::path_processor::process_task_and_branches;
use crate::status_updater::{Operation, StatusUpdateMessage};
use crate::types::action_types::Action;
use crate::types::task_types::{FlowSessionStatus, Task, TriggerSessionStatus};
use crate::types::workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition};

// Constants for parallel processing
pub const MAX_CONCURRENT_BRANCHES: usize = 10;
pub const BRANCH_PROCESSING_TIMEOUT_SECS: u64 = 300; // 5 minutes

#[derive(Clone)]
pub struct ProcessingContext {
    pub state: Arc<AppState>,
    pub client: postgrest::Postgrest,
    pub flow_session_id: Uuid,
    pub workflow_id: Uuid,
    pub trigger_task_id: String,
    pub trigger_session_id: Uuid,
    pub workflow: Arc<DatabaseFlowVersion>,
    pub workflow_def: Arc<WorkflowVersionDefinition>,
    pub workflow_graph: HashMap<String, Vec<String>>, // Pre-computed graph
    pub processed_tasks: Arc<Mutex<HashMap<Uuid, Task>>>, // Track processed tasks in memory
    // Enhanced parallel processing fields
    pub branch_semaphore: Arc<Semaphore>,
    pub active_branches: Arc<Mutex<usize>>,
    pub metrics_labels: Vec<KeyValue>,
    pub span_factory: EnhancedSpanFactory,
}

impl ProcessingContext {
    pub fn new(
        state: Arc<AppState>,
        client: postgrest::Postgrest,
        processor_message: &ProcessorMessage,
    ) -> Self {
        let environment = if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        };

        let service_name = "anything-server".to_string();

        let metrics_labels = vec![
            KeyValue::new("service", service_name.clone()),
            KeyValue::new("environment", environment.to_string()),
            KeyValue::new("workflow_id", processor_message.workflow_id.to_string()),
            KeyValue::new(
                "flow_session_id",
                processor_message.flow_session_id.to_string(),
            ),
        ];

        Self {
            state,
            client,
            flow_session_id: processor_message.flow_session_id,
            workflow_id: processor_message.workflow_id,
            trigger_task_id: processor_message
                .trigger_task
                .as_ref()
                .map(|t| t.task_id.to_string())
                .unwrap_or_default(),
            trigger_session_id: processor_message.trigger_session_id,
            workflow: Arc::new(processor_message.workflow_version.clone()),
            workflow_def: Arc::new(processor_message.workflow_definition.clone()),
            workflow_graph: processor_message.workflow_graph.clone(),
            processed_tasks: Arc::new(Mutex::new(processor_message.existing_tasks.clone())),
            branch_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_BRANCHES)),
            active_branches: Arc::new(Mutex::new(0)),
            metrics_labels,
            span_factory: EnhancedSpanFactory::new(service_name, environment.to_string()),
        }
    }

    pub async fn increment_active_branches(&self) {
        let mut count = self.active_branches.lock().await;
        *count += 1;
        info!("[PARALLELIZER] Active branches: {}", *count);
    }

    pub async fn decrement_active_branches(&self) {
        let mut count = self.active_branches.lock().await;
        *count = count.saturating_sub(1);
        info!("[PARALLELIZER] Active branches: {}", *count);
    }

    pub async fn get_active_branches_count(&self) -> usize {
        *self.active_branches.lock().await
    }
}

/// Enhanced parallel workflow processor
pub struct EnhancedParallelProcessor {
    context: ProcessingContext,
}

impl EnhancedParallelProcessor {
    pub fn new(context: ProcessingContext) -> Self {
        Self { context }
    }

    #[instrument(skip(self, initial_task), fields(
        flow_session_id = %self.context.flow_session_id,
        workflow_id = %self.context.workflow_id,
    ))]
    pub async fn process_workflow_parallel(
        &self,
        initial_task: Task,
    ) -> Result<(), ProcessorError> {
        let workflow_span = self.context.span_factory.create_workflow_lifecycle_span(
            self.context.flow_session_id,
            self.context.workflow_id,
            self.context.workflow.flow_version_id,
            Some(initial_task.task_id),
        );
        let _workflow_guard = workflow_span.enter();

        info!(
            "[PARALLELIZER] Starting parallel workflow processing for flow session: {}",
            self.context.flow_session_id
        );

        METRICS.record_workflow_started(&self.context.metrics_labels);
        let workflow_start = Instant::now();

        // Create the initial task in the database
        if let Err(e) = create_task(&self.context, &initial_task).await {
            error!("[PARALLELIZER] Failed to create initial task: {}", e);
            return Err(ProcessorError::WorkflowExecutionError(e.to_string()));
        }

        // Process the workflow with parallel branch execution
        let processing_result = self.process_branches_parallel(initial_task).await;

        // Update workflow completion status
        let completion_result = self.complete_workflow().await;

        // Record metrics
        let workflow_duration = workflow_start.elapsed();
        METRICS.record_workflow_completed(workflow_duration, &self.context.metrics_labels);

        info!(
            "[PARALLELIZER] Workflow processing completed in {:?}",
            workflow_duration
        );

        // Return the first error if any occurred
        processing_result.and(completion_result)
    }

    #[instrument(skip(self, initial_task), fields(
        task_id = %initial_task.task_id,
        action_label = %initial_task.action_label,
    ))]
    async fn process_branches_parallel(&self, initial_task: Task) -> Result<(), ProcessorError> {
        let process_span = self
            .context
            .span_factory
            .create_task_processing_span(initial_task.task_id, &initial_task.action_label);
        let _process_guard = process_span.enter();

        info!(
            "[PARALLELIZER] Starting parallel branch processing for task: {}",
            initial_task.task_id
        );

        // Use a JoinSet to manage parallel tasks
        let mut join_set = JoinSet::new();

        // Start the initial branch
        self.spawn_branch_processor(&mut join_set, initial_task)
            .await?;

        // Wait for all branches to complete
        let mut completed_branches = 0;
        let mut failed_branches = 0;

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(branch_result) => match branch_result {
                    Ok(_) => {
                        completed_branches += 1;
                        info!(
                            "[PARALLELIZER] Branch completed successfully (total: {})",
                            completed_branches
                        );
                    }
                    Err(e) => {
                        failed_branches += 1;
                        error!(
                            "[PARALLELIZER] Branch failed: {} (total failures: {})",
                            e, failed_branches
                        );
                    }
                },
                Err(join_error) => {
                    failed_branches += 1;
                    error!(
                        "[PARALLELIZER] Branch task panicked: {} (total failures: {})",
                        join_error, failed_branches
                    );
                }
            }

            self.context.decrement_active_branches().await;
        }

        info!(
            "[PARALLELIZER] All branches completed. Success: {}, Failed: {}",
            completed_branches, failed_branches
        );

        if failed_branches > 0 {
            warn!(
                "[PARALLELIZER] Workflow completed with {} failed branches",
                failed_branches
            );
        }

        Ok(())
    }

    async fn spawn_branch_processor(
        &self,
        join_set: &mut JoinSet<Result<(), ProcessorError>>,
        task: Task,
    ) -> Result<(), ProcessorError> {
        // Acquire semaphore permit for branch processing
        let permit_start = Instant::now();
        let permit = self
            .context
            .branch_semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| ProcessorError::SemaphoreError(e.to_string()))?;

        let permit_duration = permit_start.elapsed();
        METRICS.record_semaphore_wait_time(permit_duration, &self.context.metrics_labels);

        self.context.increment_active_branches().await;

        let context = self.context.clone();
        let task_id = task.task_id;

        join_set.spawn(async move {
            let _permit_guard = permit;
            let branch_span = context
                .span_factory
                .create_task_processing_span(task_id, &task.action_label);
            let _branch_guard = branch_span.enter();

            info!("[PARALLELIZER] Processing branch for task: {}", task_id);

            // Create a timeout for branch processing
            let branch_future = process_task_and_branches(Arc::new(context.clone()), task);
            let timeout_duration = tokio::time::Duration::from_secs(BRANCH_PROCESSING_TIMEOUT_SECS);

            match tokio::time::timeout(timeout_duration, branch_future).await {
                Ok(_) => {
                    info!("[PARALLELIZER] Branch {} completed successfully", task_id);
                    Ok(())
                }
                Err(_) => {
                    error!(
                        "[PARALLELIZER] Branch {} timed out after {:?}",
                        task_id, timeout_duration
                    );
                    Err(ProcessorError::WorkflowExecutionError(format!(
                        "Branch {} timed out",
                        task_id
                    )))
                }
            }
        });

        Ok(())
    }

    async fn complete_workflow(&self) -> Result<(), ProcessorError> {
        let update_span = tracing::info_span!("update_flow_session_status");
        let _update_guard = update_span.enter();
        let update_start = Instant::now();

        let task_message = StatusUpdateMessage {
            operation: Operation::CompleteWorkflow {
                flow_session_id: self.context.flow_session_id,
                status: FlowSessionStatus::Completed,
                trigger_status: TriggerSessionStatus::Completed,
            },
        };

        // Enhanced retry logic with exponential backoff
        let mut retry_count = 0;
        const MAX_SEND_RETRIES: usize = 5;
        const BASE_RETRY_DELAY_MS: u64 = 100;

        while retry_count < MAX_SEND_RETRIES {
            match self
                .context
                .state
                .task_updater_sender
                .send(task_message.clone())
                .await
            {
                Ok(_) => {
                    info!("[PARALLELIZER] Successfully sent flow session completion status");
                    break;
                }
                Err(e) => {
                    retry_count += 1;
                    let delay_ms = BASE_RETRY_DELAY_MS * (2_u64.pow(retry_count as u32 - 1));

                    error!(
                        "[PARALLELIZER] Failed to send flow session completion status (attempt {}/{}): {}",
                        retry_count, MAX_SEND_RETRIES, e
                    );

                    if retry_count >= MAX_SEND_RETRIES {
                        // Critical: As a last resort, try direct database update
                        error!("[PARALLELIZER] All status update attempts failed, attempting direct database update");
                        match crate::processor::db_calls::update_flow_session_status(
                            &self.context.state,
                            &self.context.flow_session_id,
                            &FlowSessionStatus::Completed,
                            &TriggerSessionStatus::Completed,
                        )
                        .await
                        {
                            Ok(_) => {
                                info!("[PARALLELIZER] Direct database update succeeded");
                                break;
                            }
                            Err(db_err) => {
                                error!(
                                    "[PARALLELIZER] Direct database update also failed: {}",
                                    db_err
                                );
                                return Err(ProcessorError::WorkflowExecutionError(format!(
                                    "Failed to update workflow status: {}",
                                    db_err
                                )));
                            }
                        }
                    } else {
                        // Exponential backoff
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }

        let update_duration = update_start.elapsed();
        info!(
            "[PARALLELIZER] Flow session status update completed in {:?}",
            update_duration
        );

        Ok(())
    }
}

/// Main entry point for parallel workflow processing
#[instrument(skip(state, client, processor_message))]
pub async fn process_workflow(
    state: Arc<AppState>,
    client: postgrest::Postgrest,
    processor_message: ProcessorMessage,
) {
    let flow_session_id = processor_message.flow_session_id;
    let workflow_id = processor_message.workflow_id;
    let workflow_version_id = processor_message.workflow_version.flow_version_id;
    let task_id = processor_message.task_id;

    // Create root span with task_id for tracing
    let root_span = tracing::info_span!("process_workflow_parallel",
        flow_session_id = %flow_session_id,
        workflow_id = %workflow_id,
        workflow_version_id = %workflow_version_id,
        task_id = task_id.map(|id| id.to_string()).as_deref().unwrap_or("unknown")
    );
    let _root_entered = root_span.enter();
    let workflow_start = Instant::now();

    info!(
        "[PARALLELIZER] Processing workflow for flow session: {}",
        processor_message.flow_session_id
    );
    if let Some(task_id) = task_id {
        info!("[PARALLELIZER] Processing task_id: {}", task_id);
    }

    // Create processing context
    let ctx_start = Instant::now();
    let context = ProcessingContext::new(state, client, &processor_message);
    let ctx_duration = ctx_start.elapsed();
    info!(
        "[PARALLELIZER] ProcessingContext created in {:?}",
        ctx_duration
    );

    // Record message received
    METRICS.record_message_received(&context.metrics_labels);

    // Create enhanced processor
    let processor = EnhancedParallelProcessor::new(context);

    if let Some(task) = processor_message.trigger_task {
        match processor.process_workflow_parallel(task).await {
            Ok(_) => {
                info!("[PARALLELIZER] Workflow processing completed successfully");
            }
            Err(e) => {
                error!("[PARALLELIZER] Workflow processing failed: {}", e);
                METRICS.record_workflow_error(&e.to_string());
                // The error handling is already done in the processor methods
            }
        }
    } else {
        warn!("[PARALLELIZER] No trigger task provided, skipping workflow processing");
    }

    let workflow_duration = workflow_start.elapsed();
    info!(
        "[PARALLELIZER] process_workflow total duration: {:?}",
        workflow_duration
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::task_types::TaskStatus;

    #[tokio::test]
    async fn test_processing_context_creation() {
        // This would require setting up a mock ProcessorMessage
        // Implementation depends on your test infrastructure
    }

    #[test]
    fn test_constants() {
        assert!(MAX_CONCURRENT_BRANCHES > 0);
        assert!(BRANCH_PROCESSING_TIMEOUT_SECS > 0);
    }
}

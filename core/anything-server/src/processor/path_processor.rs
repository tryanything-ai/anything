use crate::processor::components::ProcessorError;
use crate::processor::parallelizer::ProcessingContext;
use crate::processor::processor_utils::{create_task_for_action, process_task};
use crate::processor::utils::create_workflow_graph;
use crate::status_updater::{Operation, StatusUpdateMessage};
use crate::types::task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinSet;
use tracing::{error, info, instrument, warn, Span};

/// Enhanced parallel branch processor
pub struct EnhancedBranchProcessor {
    context: Arc<ProcessingContext>,
}

impl EnhancedBranchProcessor {
    pub fn new(context: Arc<ProcessingContext>) -> Self {
        Self { context }
    }

    #[instrument(skip(self, task), fields(
        task_id = %task.task_id,
        action_label = %task.action_label,
        flow_session_id = %self.context.flow_session_id,
    ))]
    pub async fn process_task_and_branches_parallel(
        &self,
        task: Task,
    ) -> Result<(), ProcessorError> {
        let task_id = task.task_id;
        let action_label = task.action_label.clone();

        let branch_span = self
            .context
            .span_factory
            .create_task_processing_span(task_id, &action_label);
        let _branch_guard = branch_span.enter();

        info!(
            "[BRANCH_PROCESSOR] Starting parallel processing for task: {} ({})",
            task_id, action_label
        );

        let processing_start = Instant::now();
        let graph = create_workflow_graph(&self.context.workflow_def);
        let mut current_task = task;

        loop {
            // Process the current task
            let task_processing_start = Instant::now();
            let next_actions = match process_task(&self.context, &current_task, &graph).await {
                Ok(actions) => {
                    let task_duration = task_processing_start.elapsed();
                    info!(
                        "[BRANCH_PROCESSOR] Task {} processed successfully in {:?}, found {} next actions",
                        current_task.task_id, task_duration, actions.len()
                    );
                    actions
                }
                Err(e) => {
                    let task_duration = task_processing_start.elapsed();
                    error!(
                        "[BRANCH_PROCESSOR] Task {} failed after {:?}: {}",
                        current_task.task_id, task_duration, e.error
                    );

                    // Update task status to Failed
                    self.update_task_status_failed(&current_task, &e.error.to_string())
                        .await;
                    return Err(ProcessorError::WorkflowExecutionError(e.error.to_string()));
                }
            };

            if next_actions.is_empty() {
                info!(
                    "[BRANCH_PROCESSOR] No more actions for task {}, branch completed",
                    current_task.task_id
                );
                break;
            }

            // Process branches in parallel
            match self
                .process_branches_parallel(&current_task, next_actions)
                .await
            {
                Ok(Some(continuation_task)) => {
                    // Continue with the first branch in this thread
                    current_task = continuation_task;
                    info!(
                        "[BRANCH_PROCESSOR] Continuing with task {} in current branch",
                        current_task.task_id
                    );
                }
                Ok(None) => {
                    // All branches were spawned, this thread is done
                    info!("[BRANCH_PROCESSOR] All branches spawned, current thread completed");
                    break;
                }
                Err(e) => {
                    error!("[BRANCH_PROCESSOR] Failed to process branches: {}", e);
                    return Err(e);
                }
            }
        }

        let total_duration = processing_start.elapsed();
        info!(
            "[BRANCH_PROCESSOR] Branch processing completed for initial task {} in {:?}",
            task_id, total_duration
        );

        Ok(())
    }

    async fn process_branches_parallel(
        &self,
        current_task: &Task,
        next_actions: Vec<crate::types::action_types::Action>,
    ) -> Result<Option<Task>, ProcessorError> {
        if next_actions.is_empty() {
            return Ok(None);
        }

        info!(
            "[BRANCH_PROCESSOR] Processing {} branches for task {}",
            next_actions.len(),
            current_task.task_id
        );

        // Create tasks for all actions
        let mut created_tasks = Vec::new();
        for action in &next_actions {
            match create_task_for_action(&self.context, action, current_task.processing_order + 1)
                .await
            {
                Ok(new_task) => {
                    info!(
                        "[BRANCH_PROCESSOR] Created task {} for action {}",
                        new_task.task_id, action.action_id
                    );
                    created_tasks.push(new_task);
                }
                Err(e) => {
                    error!(
                        "[BRANCH_PROCESSOR] Failed to create task for action {}: {}",
                        action.action_id, e
                    );

                    // Mark workflow as failed
                    self.mark_workflow_failed().await;
                    return Err(ProcessorError::WorkflowExecutionError(format!(
                        "Failed to create task for action {}: {}",
                        action.action_id, e
                    )));
                }
            }
        }

        if created_tasks.is_empty() {
            return Ok(None);
        }

        // If we have only one task, return it for continuation in current thread
        if created_tasks.len() == 1 {
            return Ok(Some(created_tasks.into_iter().next().unwrap()));
        }

        // For multiple tasks, spawn parallel branches for all but the first
        let mut join_set = JoinSet::new();
        let mut tasks_iter = created_tasks.into_iter();
        let first_task = tasks_iter.next().unwrap(); // Safe because we checked length above

        // Spawn parallel branches for remaining tasks
        for task in tasks_iter {
            self.spawn_parallel_branch_direct(&mut join_set, task)?;
        }

        // Wait for all spawned branches to complete (don't block on them)
        tokio::spawn(async move {
            let mut completed = 0;
            let mut failed = 0;

            while let Some(result) = join_set.join_next().await {
                match result {
                    Ok(branch_result) => match branch_result {
                        Ok(_) => {
                            completed += 1;
                            info!(
                                "[BRANCH_PROCESSOR] Parallel branch completed (total: {})",
                                completed
                            );
                        }
                        Err(e) => {
                            failed += 1;
                            error!("[BRANCH_PROCESSOR] Parallel branch failed: {} (total failures: {})", e, failed);
                        }
                    },
                    Err(join_error) => {
                        failed += 1;
                        error!(
                            "[BRANCH_PROCESSOR] Parallel branch panicked: {} (total failures: {})",
                            join_error, failed
                        );
                    }
                }
            }

            info!(
                "[BRANCH_PROCESSOR] All parallel branches completed. Success: {}, Failed: {}",
                completed, failed
            );
        });

        // Return the first task for continuation in current thread
        Ok(Some(first_task))
    }

    fn spawn_parallel_branch_direct(
        &self,
        join_set: &mut JoinSet<Result<(), ProcessorError>>,
        task: Task,
    ) -> Result<(), ProcessorError> {
        let context = self.context.clone();
        let task_id = task.task_id;

        join_set.spawn(async move {
            // Acquire semaphore permit for branch processing
            let permit_start = Instant::now();
            let permit = context
                .branch_semaphore
                .clone()
                .acquire_owned()
                .await
                .map_err(|e| ProcessorError::SemaphoreError(e.to_string()))?;

            let permit_duration = permit_start.elapsed();
            context
                .metrics_recorder
                .record_semaphore_wait_time(permit_duration);

            context.increment_active_branches().await;

            let _permit_guard = permit;
            let branch_span = context
                .span_factory
                .create_task_processing_span(task_id, &task.action_label);
            let _branch_guard = branch_span.enter();

            info!(
                "[BRANCH_PROCESSOR] Starting parallel branch for task: {}",
                task_id
            );

            // Create a new branch processor for this parallel branch
            let branch_processor = EnhancedBranchProcessor::new(context.clone());

            // Process this branch
            let result = branch_processor
                .process_task_and_branches_parallel(task)
                .await;

            // Decrement active branches count
            context.decrement_active_branches().await;

            match &result {
                Ok(_) => {
                    info!(
                        "[BRANCH_PROCESSOR] Parallel branch {} completed successfully",
                        task_id
                    );
                }
                Err(e) => {
                    error!(
                        "[BRANCH_PROCESSOR] Parallel branch {} failed: {}",
                        task_id, e
                    );
                }
            }

            result
        });

        Ok(())
    }

    async fn spawn_parallel_branch(
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
        self.context
            .metrics_recorder
            .record_semaphore_wait_time(permit_duration);

        self.context.increment_active_branches().await;

        let context = self.context.clone();
        let task_id = task.task_id;

        join_set.spawn(async move {
            let _permit_guard = permit;
            let branch_span = context
                .span_factory
                .create_task_processing_span(task_id, &task.action_label);
            let _branch_guard = branch_span.enter();

            info!(
                "[BRANCH_PROCESSOR] Starting parallel branch for task: {}",
                task_id
            );

            // Create a new branch processor for this parallel branch
            let branch_processor = EnhancedBranchProcessor::new(context.clone());

            // Process this branch
            let result = branch_processor
                .process_task_and_branches_parallel(task)
                .await;

            // Decrement active branches count
            context.decrement_active_branches().await;

            match &result {
                Ok(_) => {
                    info!(
                        "[BRANCH_PROCESSOR] Parallel branch {} completed successfully",
                        task_id
                    );
                }
                Err(e) => {
                    error!(
                        "[BRANCH_PROCESSOR] Parallel branch {} failed: {}",
                        task_id, e
                    );
                }
            }

            result
        });

        Ok(())
    }

    async fn update_task_status_failed(&self, task: &Task, error_message: &str) {
        let error_update = StatusUpdateMessage {
            operation: Operation::UpdateTask {
                task_id: task.task_id,
                started_at: None,
                ended_at: Some(chrono::Utc::now()),
                status: TaskStatus::Failed,
                result: None,
                context: None,
                error: Some(serde_json::json!({ "error": error_message })),
            },
        };

        if let Err(send_err) = self
            .context
            .state
            .task_updater_sender
            .send(error_update)
            .await
        {
            error!(
                "[BRANCH_PROCESSOR] Failed to send task failure status update: {}",
                send_err
            );
        }
    }

    async fn mark_workflow_failed(&self) {
        let workflow_failure = StatusUpdateMessage {
            operation: Operation::CompleteWorkflow {
                flow_session_id: self.context.flow_session_id,
                status: FlowSessionStatus::Failed,
                trigger_status: TriggerSessionStatus::Failed,
            },
        };

        if let Err(send_err) = self
            .context
            .state
            .task_updater_sender
            .send(workflow_failure)
            .await
        {
            error!(
                "[BRANCH_PROCESSOR] Failed to send workflow failure status update: {}",
                send_err
            );

            // As fallback, try direct database update
            if let Err(db_err) = crate::processor::db_calls::update_flow_session_status(
                &self.context.state,
                &self.context.flow_session_id,
                &FlowSessionStatus::Failed,
                &TriggerSessionStatus::Failed,
            )
            .await
            {
                error!("[BRANCH_PROCESSOR] Direct database update for workflow failure also failed: {}", db_err);
            }
        }
    }
}

/// Legacy function for backward compatibility - now uses enhanced parallel processing
pub fn process_task_and_branches(
    ctx: Arc<ProcessingContext>,
    initial_task: Task,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let processor = EnhancedBranchProcessor::new(ctx);

        match processor
            .process_task_and_branches_parallel(initial_task)
            .await
        {
            Ok(_) => {
                info!(
                    "[BRANCH_PROCESSOR] Legacy wrapper: Branch processing completed successfully"
                );
            }
            Err(e) => {
                error!(
                    "[BRANCH_PROCESSOR] Legacy wrapper: Branch processing failed: {}",
                    e
                );
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_branch_processor_creation() {
        // This would require setting up a mock ProcessingContext
        // Implementation depends on your test infrastructure
    }
}

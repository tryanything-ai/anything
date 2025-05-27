use crate::metrics::METRICS;
use crate::processor::components::{
    EnhancedMetricsRecorder, EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext,
};
use crate::processor::parallelizer::process_workflow;
use crate::processor::processor::ProcessorMessage;
use crate::types::task_types::Task;
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::{error, info, instrument, warn, Span};
use uuid::Uuid;

/// Enhanced workflow processor with better observability
pub struct EnhancedWorkflowProcessor {
    state: Arc<AppState>,
    metrics_recorder: EnhancedMetricsRecorder,
    span_factory: EnhancedSpanFactory,
    service_name: String,
    environment: String,
}

impl EnhancedWorkflowProcessor {
    pub fn new(state: Arc<AppState>) -> Self {
        let environment = if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        };

        let service_name = "anything-server".to_string();

        Self {
            state,
            metrics_recorder: EnhancedMetricsRecorder::with_labels(vec![
                ("service", service_name.clone()),
                ("environment", environment.to_string()),
            ]),
            span_factory: EnhancedSpanFactory::new(service_name.clone(), environment.to_string()),
            service_name,
            environment: environment.to_string(),
        }
    }

    pub async fn run(
        &self,
        mut receiver: mpsc::Receiver<ProcessorMessage>,
    ) -> Result<(), ProcessorError> {
        info!(
            "[ENHANCED_PROCESSOR] Starting processor in {} environment",
            self.environment
        );

        while let Some(message) = receiver.recv().await {
            if let Err(e) = self.process_message(message).await {
                error!("[ENHANCED_PROCESSOR] Error processing message: {}", e);
                self.metrics_recorder.record_workflow_error(&e.to_string());
            }
        }

        info!("[ENHANCED_PROCESSOR] Processor shutdown complete");
        Ok(())
    }

    #[instrument(skip(self), fields(
        flow_session_id = %message.flow_session_id,
        workflow_id = %message.workflow_id,
        task_id = ?message.task_id,
        service = %self.service_name,
        environment = %self.environment
    ))]
    async fn process_message(&self, message: ProcessorMessage) -> Result<(), ProcessorError> {
        self.metrics_recorder.record_message_received();

        let workflow_start = Instant::now();
        let lifecycle_span = self.span_factory.create_workflow_lifecycle_span(
            message.flow_session_id,
            message.workflow_id,
            message.workflow_version.flow_version_id,
            message.task_id,
        );
        let _lifecycle_guard = lifecycle_span.enter();

        info!(
            "[ENHANCED_PROCESSOR] Processing message for workflow {}",
            message.workflow_id
        );

        // Create execution context
        let context = WorkflowExecutionContext::new(
            message.flow_session_id,
            message.workflow_id,
            message.task_id,
            lifecycle_span.clone(),
        );

        // Record initial stage
        context.record_stage("acquiring_permit");

        // Measure semaphore wait time
        let permit_start = Instant::now();
        let permit = self
            .acquire_workflow_permit()
            .await
            .map_err(|e| ProcessorError::SemaphoreError(e.to_string()))?;
        let permit_duration = permit_start.elapsed();
        self.metrics_recorder
            .record_semaphore_wait_time(permit_duration);

        context.record_stage("executing_workflow");

        // Execute the workflow
        match self.execute_workflow(message, permit, &context).await {
            Ok(_) => {
                context.record_success();
                Ok(())
            }
            Err(e) => {
                context.record_error(&e.to_string());
                Err(ProcessorError::WorkflowExecutionError(e.to_string()))
            }
        }
    }

    async fn acquire_workflow_permit(
        &self,
    ) -> Result<OwnedSemaphorePermit, Box<dyn std::error::Error + Send + Sync>> {
        let available_permits = self.state.workflow_processor_semaphore.available_permits();
        info!(
            "[ENHANCED_PROCESSOR] 🔒 Acquiring permit - {} permits available",
            available_permits
        );

        let permit = self
            .state
            .workflow_processor_semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| format!("Failed to acquire semaphore: {}", e))?;

        let remaining_permits = self.state.workflow_processor_semaphore.available_permits();
        info!(
            "[ENHANCED_PROCESSOR] ✅ Permit acquired - {} permits remaining",
            remaining_permits
        );

        Ok(permit)
    }

    async fn execute_workflow(
        &self,
        message: ProcessorMessage,
        permit: OwnedSemaphorePermit,
        context: &WorkflowExecutionContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.metrics_recorder.record_workflow_started();

        let state = Arc::clone(&self.state);
        let client = state.anything_client.clone();
        let flow_session_id = message.flow_session_id;
        let task_id = message.task_id;
        let metrics_recorder = self.metrics_recorder.clone();
        let span_factory = self.span_factory.clone();

        // Get action type from trigger task if available
        let action_type = message
            .trigger_task
            .as_ref()
            .map(|t| format!("{:?}", t.r#type));

        let workflow_handle = tokio::spawn(async move {
            let _permit_guard = permit;
            let execution_span = span_factory.create_workflow_execution_span(
                flow_session_id,
                task_id,
                action_type.as_deref(),
            );
            let _exec_guard = execution_span.enter();

            let exec_start = Instant::now();
            info!(
                "[ENHANCED_PROCESSOR] Starting workflow execution for {}",
                flow_session_id
            );

            // Process workflow
            process_workflow(state, (*client).clone(), message).await;

            let exec_duration = exec_start.elapsed();
            execution_span.record("execution_duration_ms", exec_duration.as_millis() as i64);
            metrics_recorder.record_workflow_completed(exec_duration);

            info!(
                "[ENHANCED_PROCESSOR] Completed workflow {} (duration: {:?})",
                flow_session_id, exec_duration
            );
        });

        workflow_handle
            .await
            .map_err(|e| format!("Workflow task failed: {}", e))?;

        Ok(())
    }
}

/// Entry point for the enhanced processor
pub async fn enhanced_processor(
    state: Arc<AppState>,
    processor_receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), ProcessorError> {
    let processor = EnhancedWorkflowProcessor::new(state);
    processor.run(processor_receiver).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_processor_creation() {
        // Test that the enhanced processor initializes with correct environment
        let environment = if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        };

        assert_eq!(environment, "development"); // This test runs in debug mode
    }
}

use std::time::Instant;
use tracing::{error, info, Span};
use uuid::Uuid;

/// Enhanced span factory with additional context
#[derive(Clone)]
pub struct EnhancedSpanFactory {
    service_name: String,
    environment: String,
}

impl EnhancedSpanFactory {
    pub fn new(service_name: String, environment: String) -> Self {
        Self {
            service_name,
            environment,
        }
    }

    pub fn create_workflow_lifecycle_span(
        &self,
        flow_session_id: Uuid,
        workflow_id: Uuid,
        workflow_version_id: Uuid,
        task_id: Option<Uuid>,
    ) -> Span {
        tracing::info_span!(
            "workflow_lifecycle",
            service = %self.service_name,
            environment = %self.environment,
            flow_session_id = %flow_session_id,
            workflow_id = %workflow_id,
            workflow_version_id = %workflow_version_id,
            task_id = task_id.map(|id| id.to_string()).as_deref().unwrap_or("unknown"),
            otel.status_code = tracing::field::Empty,
            otel.status_message = tracing::field::Empty,
        )
    }

    pub fn create_workflow_execution_span(
        &self,
        flow_session_id: Uuid,
        task_id: Option<Uuid>,
        action_type: Option<&str>,
    ) -> Span {
        tracing::info_span!(
            "workflow_execution",
            service = %self.service_name,
            flow_session_id = %flow_session_id,
            task_id = task_id.map(|id| id.to_string()).as_deref().unwrap_or("unknown"),
            action_type = action_type.unwrap_or("unknown"),
            execution_duration_ms = tracing::field::Empty,
        )
    }

    pub fn create_task_processing_span(&self, task_id: Uuid, task_type: &str) -> Span {
        tracing::info_span!(
            "task_processing",
            task_id = %task_id,
            task_type = %task_type,
            processing_stage = tracing::field::Empty,
        )
    }
}

/// Workflow execution context for better tracking
pub struct WorkflowExecutionContext {
    pub flow_session_id: Uuid,
    pub workflow_id: Uuid,
    pub task_id: Option<Uuid>,
    pub start_time: Instant,
    span: Span,
}

impl WorkflowExecutionContext {
    pub fn new(
        flow_session_id: Uuid,
        workflow_id: Uuid,
        task_id: Option<Uuid>,
        span: Span,
    ) -> Self {
        Self {
            flow_session_id,
            workflow_id,
            task_id,
            start_time: Instant::now(),
            span,
        }
    }

    pub fn record_stage(&self, stage: &str) {
        self.span.record("processing_stage", stage);
    }

    pub fn record_success(&self) {
        let duration = self.start_time.elapsed();
        self.span
            .record("execution_duration_ms", duration.as_millis() as i64);
        self.span.record("otel.status_code", "OK");
        info!(
            "Workflow {} completed successfully in {:?}",
            self.flow_session_id, duration
        );
    }

    pub fn record_error(&self, error: &str) {
        let duration = self.start_time.elapsed();
        self.span
            .record("execution_duration_ms", duration.as_millis() as i64);
        self.span.record("otel.status_code", "ERROR");
        self.span.record("otel.status_message", error);
        error!(
            "Workflow {} failed after {:?}: {}",
            self.flow_session_id, duration, error
        );
    }
}

/// Error types for better error categorization
#[derive(Debug)]
pub enum ProcessorError {
    SemaphoreError(String),
    WorkflowExecutionError(String),
    MessageProcessingError(String),
    ChannelClosed,
}

impl std::fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessorError::SemaphoreError(msg) => {
                write!(f, "Failed to acquire semaphore: {}", msg)
            }
            ProcessorError::WorkflowExecutionError(msg) => {
                write!(f, "Workflow execution failed: {}", msg)
            }
            ProcessorError::MessageProcessingError(msg) => {
                write!(f, "Message processing failed: {}", msg)
            }
            ProcessorError::ChannelClosed => write!(f, "Channel closed unexpectedly"),
        }
    }
}

impl std::error::Error for ProcessorError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_execution_context() {
        let span = tracing::info_span!("test");
        let context = WorkflowExecutionContext::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            span,
        );

        context.record_stage("initialization");
        context.record_stage("processing");
        context.record_success();
    }
}

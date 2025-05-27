use crate::metrics::METRICS;
use crate::processor::parallelizer::process_workflow;
use crate::types::task_types::Task;
use crate::types::workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition};
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use std::collections::HashMap;
use tracing::{error, info, instrument, warn, Span};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProcessorMessage {
    pub workflow_id: Uuid,
    pub workflow_version: DatabaseFlowVersion,
    pub workflow_definition: WorkflowVersionDefinition,
    pub flow_session_id: Uuid,
    pub trigger_session_id: Uuid,
    pub trigger_task: Option<Task>,
    pub task_id: Option<Uuid>,               // Add task_id for tracing
    pub existing_tasks: HashMap<Uuid, Task>, // Add any existing tasks from hydration
    pub workflow_graph: HashMap<String, Vec<String>>, // Pre-computed workflow graph
}

/// Main processor struct that encapsulates the processing logic
pub struct WorkflowProcessor {
    state: Arc<AppState>,
    metrics_recorder: MetricsRecorder,
    span_factory: SpanFactory,
}

impl WorkflowProcessor {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            metrics_recorder: MetricsRecorder::new(),
            span_factory: SpanFactory::new(),
        }
    }

    /// Main processing loop
    pub async fn run(
        &self,
        mut receiver: mpsc::Receiver<ProcessorMessage>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[PROCESSOR] Starting processor");

        while let Some(message) = receiver.recv().await {
            if let Err(e) = self.process_message(message).await {
                error!("[PROCESSOR] Error processing message: {}", e);
                // Continue processing other messages
            }
        }

        info!("[PROCESSOR] Processor shutdown complete");
        Ok(())
    }

    /// Process a single message
    #[instrument(skip(self), fields(
        flow_session_id = %message.flow_session_id,
        workflow_id = %message.workflow_id,
        task_id = ?message.task_id
    ))]
    async fn process_message(
        &self,
        message: ProcessorMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.metrics_recorder.record_message_received();

        let workflow_start = Instant::now();
        let _root_span = self.span_factory.create_workflow_lifecycle_span(&message);

        info!("[PROCESSOR] Received a new message for processing");

        // Acquire permit for rate limiting
        let permit = self.acquire_workflow_permit().await?;

        // Execute the workflow
        self.execute_workflow(message, permit).await?;

        let workflow_duration = workflow_start.elapsed();
        info!(
            "[PROCESSOR] Total workflow lifecycle duration: {:?}",
            workflow_duration
        );

        Ok(())
    }

    /// Acquire a permit from the semaphore for rate limiting
    async fn acquire_workflow_permit(
        &self,
    ) -> Result<OwnedSemaphorePermit, Box<dyn std::error::Error + Send + Sync>> {
        self.state
            .workflow_processor_semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| format!("Failed to acquire semaphore: {}", e).into())
    }

    /// Execute the workflow in a separate task
    async fn execute_workflow(
        &self,
        message: ProcessorMessage,
        permit: OwnedSemaphorePermit,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.metrics_recorder.record_workflow_started();

        let state = Arc::clone(&self.state);
        let client = state.anything_client.clone();
        let flow_session_id = message.flow_session_id;
        let task_id = message.task_id;
        let metrics_recorder = self.metrics_recorder.clone();
        let span_factory = self.span_factory.clone();

        let workflow_handle = tokio::spawn(async move {
            let _permit_guard = permit; // Ensure permit is released when this task completes
            let _workflow_span =
                span_factory.create_workflow_execution_span(flow_session_id, task_id);

            let exec_start = Instant::now();
            info!("[PROCESSOR] Starting workflow {}", flow_session_id);

            // Process workflow
            process_workflow(state, (*client).clone(), message).await;

            let exec_duration = exec_start.elapsed();
            metrics_recorder.record_workflow_completed(exec_duration);
            info!(
                "[PROCESSOR] Completed workflow {} (duration: {:?})",
                flow_session_id, exec_duration
            );
        });

        // Await the workflow task to ensure proper error handling
        workflow_handle
            .await
            .map_err(|e| format!("Workflow failed: {}", e))?;

        Ok(())
    }
}

/// Handles span creation for better tracing
#[derive(Clone)]
struct SpanFactory;

impl SpanFactory {
    fn new() -> Self {
        Self
    }

    fn create_workflow_lifecycle_span(&self, message: &ProcessorMessage) -> Span {
        tracing::info_span!(
            "workflow_lifecycle",
            flow_session_id = %message.flow_session_id,
            workflow_id = %message.workflow_id,
            workflow_version_id = %message.workflow_version.flow_version_id,
            task_id = message.task_id.map(|id| id.to_string()).as_deref().unwrap_or("unknown")
        )
    }

    fn create_workflow_execution_span(&self, flow_session_id: Uuid, task_id: Option<Uuid>) -> Span {
        tracing::info_span!(
            "workflow_execution",
            flow_session_id = %flow_session_id,
            task_id = task_id.map(|id| id.to_string()).as_deref().unwrap_or("unknown")
        )
    }
}

/// Handles metrics recording
#[derive(Clone)]
struct MetricsRecorder;

impl MetricsRecorder {
    fn new() -> Self {
        Self
    }

    fn record_message_received(&self) {
        METRICS.processor_messages_received.add(1, &[]);
    }

    fn record_workflow_started(&self) {
        METRICS.processor_active_workflows.add(1, &[]);
    }

    fn record_workflow_completed(&self, duration: std::time::Duration) {
        METRICS
            .processor_workflow_duration
            .record(duration.as_secs_f64(), &[]);
        METRICS.processor_active_workflows.add(-1, &[]);
    }
}

/// Entry point for the processor
pub async fn processor(
    state: Arc<AppState>,
    processor_receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let processor = WorkflowProcessor::new(state);
    processor.run(processor_receiver).await
}

use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext};
use crate::processor::parallelizer::process_workflow;
use crate::processor::processor::ProcessorMessage;

use crate::AppState;
use opentelemetry::KeyValue;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::OwnedSemaphorePermit;
use tracing::{error, info, instrument, warn};

/// Enhanced workflow processor with better observability using SeaORM
pub struct EnhancedWorkflowProcessor {
    state: Arc<AppState>,
    metrics_labels: Vec<KeyValue>,
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

        // Add runtime verification
        info!(
            "[ENHANCED_PROCESSOR SEAORM] Runtime info - Current thread: {:?}, Available parallelism: {:?}",
            std::thread::current().name(),
            std::thread::available_parallelism()
        );

        // Check if we're in a Tokio runtime context
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            info!(
                "[ENHANCED_PROCESSOR SEAORM] Tokio runtime detected - Metrics: {:?}",
                handle.metrics()
            );
        }

        let metrics_labels = vec![
            KeyValue::new("service", service_name.clone()),
            KeyValue::new("environment", environment.to_string()),
        ];

        Self {
            state,
            metrics_labels,
            span_factory: EnhancedSpanFactory::new(service_name.clone(), environment.to_string()),
            service_name,
            environment: environment.to_string(),
        }
    }

    #[instrument(
        skip(self, receiver),
        fields(
            service.name = %self.service_name,
            service.environment = %self.environment
        )
    )]
    pub async fn start_processing(
        &self,
        mut receiver: mpsc::Receiver<ProcessorMessage>,
    ) -> Result<(), ProcessorError> {
        info!("[ENHANCED_PROCESSOR SEAORM] Starting enhanced workflow processor");

        // Record processor startup
        METRICS.record_processor_started(&self.metrics_labels);

        while let Some(message) = receiver.recv().await {
            // Acquire semaphore permit for workflow processing
            match self.state.workflow_processor_semaphore.clone().acquire_owned().await {
                Ok(permit) => {
                    info!(
                        "[ENHANCED_PROCESSOR SEAORM] Processing workflow session: {}",
                        message.flow_session_id
                    );

                    if let Err(e) = self.process_workflow_message(message, permit).await {
                        error!("[ENHANCED_PROCESSOR SEAORM] Workflow processing failed: {:?}", e);
                        METRICS.record_workflow_failed(&self.metrics_labels);
                    }
                }
                Err(e) => {
                    error!("[ENHANCED_PROCESSOR SEAORM] Failed to acquire semaphore permit: {:?}", e);
                    METRICS.record_workflow_failed(&self.metrics_labels);
                }
            }
        }

        warn!("[ENHANCED_PROCESSOR SEAORM] Workflow processor receiver closed");
        Ok(())
    }

    /// Process a single workflow message with enhanced observability
    async fn process_workflow_message(
        &self,
        message: ProcessorMessage,
        permit: OwnedSemaphorePermit,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        METRICS.record_workflow_started(&self.metrics_labels);

        let state = Arc::clone(&self.state);
        // Note: No longer using Postgrest client - SeaORM database connection is in state.db
        let client = state.db.clone();
        let flow_session_id = message.flow_session_id;
        let task_id = message.task_id;
        let metrics_labels = self.metrics_labels.clone();
        let span_factory = self.span_factory.clone();

        // Get action type from trigger task if available
        let action_type = message
            .trigger_task
            .as_ref()
            .map(|t| format!("{:?}", t.r#type));

        let workflow_handle = tokio::spawn(async move {
            let start_time = Instant::now();

            // Create workflow execution context
            let span = span_factory.create_workflow_execution_span(
                flow_session_id, 
                task_id, 
                action_type.as_deref()
            );
            let context = WorkflowExecutionContext::new(
                flow_session_id,
                message.workflow_id,
                task_id,
                span,
            );

            info!(
                "[ENHANCED_PROCESSOR SEAORM] Starting workflow execution for session: {}",
                flow_session_id
            );

            // Process the workflow using SeaORM  
            process_workflow(
                state.clone(),
                client.clone(),
                message,
            )
            .await;

            let duration = start_time.elapsed();

            info!(
                "[ENHANCED_PROCESSOR SEAORM] Workflow completed successfully in {:?} for session: {}",
                duration, flow_session_id
            );
            METRICS.record_workflow_completed(duration, &metrics_labels);

            // Drop the permit to allow other workflows to process
            drop(permit);

            Ok(())
        });

        // Await the workflow completion
        match workflow_handle.await {
            Ok(Ok(_)) => {
                info!("[ENHANCED_PROCESSOR SEAORM] Workflow handle completed successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("[ENHANCED_PROCESSOR SEAORM] Workflow execution error: {:?}", e);
                Err(e)
            }
            Err(e) => {
                error!("[ENHANCED_PROCESSOR SEAORM] Workflow task join error: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Health check for the processor
    pub async fn health_check(&self) -> Result<(), ProcessorError> {
        // Check database connection
        match self.state.db.ping().await {
            Ok(_) => {
                info!("[ENHANCED_PROCESSOR SEAORM] Health check passed - database connection OK");
                Ok(())
            }
            Err(e) => {
                error!("[ENHANCED_PROCESSOR SEAORM] Health check failed - database connection error: {:?}", e);
                Err(ProcessorError::DatabaseError(format!("Database ping failed: {}", e)))
            }
        }
    }

    /// Get processor metrics
    pub fn get_metrics(&self) -> Vec<KeyValue> {
        let mut metrics = self.metrics_labels.clone();
        
        // Add dynamic metrics
        metrics.push(KeyValue::new("semaphore_available_permits", 
            self.state.workflow_processor_semaphore.available_permits() as i64));
        metrics.push(KeyValue::new("flow_completions_count", 
            self.state.flow_completions.len() as i64));
        
        metrics
    }
}

/// Factory function to create an enhanced processor instance
pub fn create_enhanced_processor(state: Arc<AppState>) -> EnhancedWorkflowProcessor {
    EnhancedWorkflowProcessor::new(state)
}

/// Start the enhanced processor with better error handling
pub async fn start_enhanced_processing(
    state: Arc<AppState>,
    receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), ProcessorError> {
    let processor = create_enhanced_processor(state);
    
    // Perform initial health check
    processor.health_check().await?;
    
    // Start processing
    processor.start_processing(receiver).await
}

use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError, WorkflowExecutionContext};
use crate::processor::parallelizer::process_workflow;
use crate::processor::processor::ProcessorMessage;

use crate::AppState;
use opentelemetry::KeyValue;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::sync::OwnedSemaphorePermit;
use tokio::time::timeout;
use tracing::{error, info, instrument, warn};

/// Enhanced workflow processor with better observability and keepalive
pub struct EnhancedWorkflowProcessor {
    state: Arc<AppState>,
    metrics_labels: Vec<KeyValue>,
    span_factory: EnhancedSpanFactory,
    service_name: String,
    environment: String,
    // Keepalive configuration
    keepalive_interval: Duration,
    message_timeout: Duration,
    semaphore_timeout: Duration,
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
            "[ENHANCED_PROCESSOR] Runtime info - Current thread: {:?}, Available parallelism: {:?}",
            std::thread::current().name(),
            std::thread::available_parallelism()
        );

        // Check if we're in a Tokio runtime context
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            info!(
                "[ENHANCED_PROCESSOR] Tokio runtime detected - Metrics: {:?}",
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
            // Configure timeouts - more aggressive in production
            keepalive_interval: if cfg!(debug_assertions) {
                Duration::from_secs(30) // 30s in dev
            } else {
                Duration::from_secs(10) // 10s in prod
            },
            message_timeout: Duration::from_secs(300), // 5 minutes max per message
            semaphore_timeout: Duration::from_secs(30), // 30s max to acquire permit
        }
    }

    pub async fn run(
        &self,
        mut receiver: mpsc::Receiver<ProcessorMessage>,
    ) -> Result<(), ProcessorError> {
        info!(
            "[ENHANCED_PROCESSOR] Starting processor in {} environment with keepalive every {:?}",
            self.environment, self.keepalive_interval
        );

        let mut last_activity = Instant::now();
        let mut keepalive_counter = 0u64;

        loop {
            // Use select! to handle both messages and keepalive
            tokio::select! {
                // Handle incoming messages with timeout
                message_result = timeout(self.keepalive_interval, receiver.recv()) => {
                    match message_result {
                        Ok(Some(message)) => {
                            last_activity = Instant::now();

                            // Process message with overall timeout
                            match timeout(self.message_timeout, self.process_message(message)).await {
                                Ok(Ok(())) => {
                                    // Success - continue
                                }
                                Ok(Err(e)) => {
                                    error!("[ENHANCED_PROCESSOR] Error processing message: {}", e);
                                    METRICS.record_workflow_error(&e.to_string());
                                }
                                Err(_) => {
                                    error!("[ENHANCED_PROCESSOR] Message processing timed out after {:?}", self.message_timeout);
                                    METRICS.record_workflow_error("message_timeout");
                                }
                            }
                        }
                        Ok(None) => {
                            // Channel closed - but don't exit immediately, keep trying to reconnect
                            warn!("[ENHANCED_PROCESSOR] Message channel closed, but keeping processor alive");
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                        Err(_) => {
                            // Timeout occurred - this is normal, continue to keepalive
                        }
                    }
                }

                // Keepalive heartbeat
                _ = tokio::time::sleep(self.keepalive_interval) => {
                    keepalive_counter += 1;
                    let idle_duration = last_activity.elapsed();

                    info!(
                        "[ENHANCED_PROCESSOR] 💓 Keepalive #{} - Idle for {:?}, {} permits available",
                        keepalive_counter,
                        idle_duration,
                        self.state.workflow_processor_semaphore.available_permits()
                    );

                    // Record keepalive metrics
                    METRICS.record_processor_keepalive(idle_duration, &self.metrics_labels);

                    // Perform health checks during idle time
                    self.perform_health_checks().await;
                }
            }
        }
    }

    async fn perform_health_checks(&self) {
        // Check semaphore health
        let available_permits = self.state.workflow_processor_semaphore.available_permits();
        if available_permits == 0 {
            warn!(
                "[ENHANCED_PROCESSOR] ⚠️  All semaphore permits are in use - potential bottleneck"
            );
        }

        // Check if we can acquire a permit quickly (health check)
        match timeout(
            Duration::from_millis(100),
            self.state
                .workflow_processor_semaphore
                .clone()
                .acquire_owned(),
        )
        .await
        {
            Ok(Ok(permit)) => {
                // Release immediately - this was just a health check
                drop(permit);
            }
            Ok(Err(e)) => {
                warn!("[ENHANCED_PROCESSOR] Semaphore health check failed: {}", e);
            }
            Err(_) => {
                warn!("[ENHANCED_PROCESSOR] Semaphore health check timed out - potential deadlock");
            }
        }
    }

    #[instrument(skip(self, message), fields(
        flow_session_id = %message.flow_session_id,
        workflow_id = %message.workflow_id,
        task_id = ?message.task_id,
        service = %self.service_name,
        environment = %self.environment
    ))]
    async fn process_message(&self, message: ProcessorMessage) -> Result<(), ProcessorError> {
        METRICS.record_message_received(&self.metrics_labels);

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

        // Measure semaphore wait time with timeout
        let permit_start = Instant::now();
        let permit = timeout(self.semaphore_timeout, self.acquire_workflow_permit())
            .await
            .map_err(|_| {
                ProcessorError::SemaphoreError(format!(
                    "Semaphore acquisition timed out after {:?}",
                    self.semaphore_timeout
                ))
            })?
            .map_err(|e| ProcessorError::SemaphoreError(e.to_string()))?;

        let permit_duration = permit_start.elapsed();
        METRICS.record_semaphore_wait_time(permit_duration, &self.metrics_labels);

        context.record_stage("executing_workflow");

        // Execute the workflow
        match self.execute_workflow(message, permit).await {
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
        // context: &WorkflowExecutionContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        METRICS.record_workflow_started(&self.metrics_labels);

        let state = Arc::clone(&self.state);
        let client = state.anything_client.clone();
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
            METRICS.record_workflow_completed(exec_duration, &metrics_labels);

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

use crate::actor_processor::actor_pool::TaskActorPool;
use crate::actor_processor::messages::ActorMessage;
use crate::actor_processor::workflow_actor::WorkflowActor;
use crate::metrics::METRICS;
use crate::processor::components::{EnhancedSpanFactory, ProcessorError};
use crate::processor::processor::ProcessorMessage;
use crate::AppState;

use opentelemetry::KeyValue;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};
use tracing::{error, info, instrument};
use uuid::Uuid;

/// Main actor-based processor system
pub struct ActorProcessor {
    state: Arc<AppState>,
    workflow_actors: Vec<mpsc::Sender<ActorMessage>>,
    task_actor_pool: TaskActorPool,
    span_factory: EnhancedSpanFactory,
    metrics_labels: Vec<KeyValue>,
    current_workflow_index: Arc<RwLock<usize>>,
}

impl ActorProcessor {
    pub fn new(state: Arc<AppState>) -> Self {
        let environment = if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        };

        let service_name = "anything-server".to_string();
        let span_factory = EnhancedSpanFactory::new(service_name.clone(), environment.to_string());

        let metrics_labels = vec![
            KeyValue::new("service", service_name),
            KeyValue::new("environment", environment.to_string()),
        ];

        // Create task actor pool (configurable via environment variables)
        let task_pool_size = std::env::var("TASK_ACTOR_POOL_SIZE")
            .unwrap_or_else(|_| "50".to_string())
            .parse()
            .unwrap_or(50);

        let task_actor_pool = TaskActorPool::new(
            task_pool_size,
            state.clone(),
            (*state.anything_client).clone(),
            span_factory.clone(),
            metrics_labels.clone(),
        );

        // Create workflow actors (fewer than task actors since they orchestrate)
        let workflow_pool_size = std::env::var("WORKFLOW_ACTOR_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        let mut workflow_actors = Vec::with_capacity(workflow_pool_size);

        for i in 0..workflow_pool_size {
            let (tx, rx) = mpsc::channel(100);
            let actor_id = Uuid::new_v4();

            let actor = WorkflowActor::new(
                actor_id,
                state.clone(),
                (*state.anything_client).clone(),
                task_actor_pool.clone(),
                span_factory.clone(),
                metrics_labels.clone(),
            );

            tokio::spawn(async move {
                actor.run(rx).await;
            });

            workflow_actors.push(tx);
            info!(
                "[WORKFLOW_ACTOR_POOL] Started workflow actor {} (index {})",
                actor_id, i
            );
        }

        info!(
            "[ACTOR_PROCESSOR] Initialized with {} task actors and {} workflow actors",
            task_pool_size, workflow_pool_size
        );

        Self {
            state,
            workflow_actors,
            task_actor_pool,
            span_factory,
            metrics_labels,
            current_workflow_index: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn run(
        &self,
        mut receiver: mpsc::Receiver<ProcessorMessage>,
    ) -> Result<(), ProcessorError> {
        info!("[ACTOR_PROCESSOR] Starting actor-based processor");

        while let Some(message) = receiver.recv().await {
            if let Err(e) = self.process_message(message).await {
                error!("[ACTOR_PROCESSOR] Error processing message: {}", e);
                METRICS.record_workflow_error(&e.to_string());
            }
        }

        info!("[ACTOR_PROCESSOR] Shutting down actor processor");
        self.shutdown().await;
        Ok(())
    }

    #[instrument(skip(self, message), fields(
        flow_session_id = %message.flow_session_id,
        workflow_id = %message.workflow_id
    ))]
    async fn process_message(&self, message: ProcessorMessage) -> Result<(), ProcessorError> {
        METRICS.record_message_received(&self.metrics_labels);

        // Round-robin load balancing for workflow actors
        let mut index = self.current_workflow_index.write().await;
        let actor_index = *index;
        *index = (*index + 1) % self.workflow_actors.len();
        drop(index);

        let workflow_actor = &self.workflow_actors[actor_index];
        let (tx, rx) = oneshot::channel();

        workflow_actor
            .send(ActorMessage::ExecuteWorkflow {
                message,
                respond_to: tx,
            })
            .await
            .map_err(|e| {
                ProcessorError::WorkflowExecutionError(format!(
                    "Failed to send workflow to actor: {}",
                    e
                ))
            })?;

        rx.await.map_err(|e| {
            ProcessorError::WorkflowExecutionError(format!(
                "Failed to receive workflow result: {}",
                e
            ))
        })?
    }

    async fn shutdown(&self) {
        info!("[ACTOR_PROCESSOR] Initiating shutdown sequence");

        // Shutdown workflow actors
        for actor in &self.workflow_actors {
            let _ = actor.send(ActorMessage::Shutdown).await;
        }

        // Shutdown task actor pool
        self.task_actor_pool.shutdown().await;

        info!("[ACTOR_PROCESSOR] Shutdown complete");
    }

    // Getter methods for monitoring and debugging
    pub fn get_workflow_pool_size(&self) -> usize {
        self.workflow_actors.len()
    }

    pub async fn get_current_workflow_index(&self) -> usize {
        *self.current_workflow_index.read().await
    }
}

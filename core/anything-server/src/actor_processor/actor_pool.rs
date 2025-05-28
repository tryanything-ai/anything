use crate::actor_processor::messages::ActorMessage;
use crate::actor_processor::task_actor::TaskActor;
use crate::processor::components::{EnhancedSpanFactory, WorkflowExecutionContext};
use crate::processor::execute_task::TaskResult;
use crate::types::task_types::Task;
use crate::AppState;

use opentelemetry::KeyValue;
use postgrest::Postgrest;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};
use tracing::info;
use uuid::Uuid;

/// Pool of task actors for load balancing
pub struct TaskActorPool {
    actors: Vec<mpsc::Sender<ActorMessage>>,
    current_index: Arc<RwLock<usize>>,
}

impl TaskActorPool {
    pub fn new(
        pool_size: usize,
        state: Arc<AppState>,
        client: Postgrest,
        span_factory: EnhancedSpanFactory,
        metrics_labels: Vec<KeyValue>,
    ) -> Self {
        let mut actors = Vec::with_capacity(pool_size);

        for i in 0..pool_size {
            let (tx, rx) = mpsc::channel(1000);
            let actor_id = Uuid::new_v4();

            let actor = TaskActor::new(
                actor_id,
                state.clone(),
                client.clone(),
                span_factory.clone(),
                metrics_labels.clone(),
            );

            tokio::spawn(async move {
                actor.run(rx).await;
            });

            actors.push(tx);
            info!(
                "[TASK_ACTOR_POOL] Started task actor {} (index {})",
                actor_id, i
            );
        }

        Self {
            actors,
            current_index: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn execute_task(
        &self,
        task: Task,
        context: WorkflowExecutionContext,
    ) -> Result<TaskResult, Box<dyn std::error::Error + Send + Sync>> {
        // Round-robin load balancing
        let mut index = self.current_index.write().await;
        let actor_index = *index;
        *index = (*index + 1) % self.actors.len();
        drop(index);

        let actor = &self.actors[actor_index];
        let (tx, rx) = oneshot::channel();

        actor
            .send(ActorMessage::ExecuteTask {
                task,
                respond_to: tx,
                context,
            })
            .await
            .map_err(|e| format!("Failed to send task to actor: {}", e))?;

        rx.await
            .map_err(|e| format!("Failed to receive task result: {}", e).into())
    }

    pub async fn shutdown(&self) {
        info!("[TASK_ACTOR_POOL] Shutting down task actor pool");
        for actor in &self.actors {
            let _ = actor.send(ActorMessage::Shutdown).await;
        }
    }
}

// Make TaskActorPool cloneable for sharing between workflow actors
impl Clone for TaskActorPool {
    fn clone(&self) -> Self {
        Self {
            actors: self.actors.clone(),
            current_index: self.current_index.clone(),
        }
    }
}

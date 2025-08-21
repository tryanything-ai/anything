use crate::actor_processor::actor_system::ActorProcessor;
use crate::processor::components::ProcessorError;
use crate::processor::processor::ProcessorMessage;
use crate::AppState;

use std::sync::Arc;
use tokio::sync::mpsc;

/// Entry point for the actor-based processor
///
/// This function creates and runs the actor-based workflow processor system.
/// It replaces the enhanced_processor with a highly parallel, asynchronous design
/// using the actor model for better scalability and fault isolation.
///
/// # Architecture
///
/// The actor processor consists of:
/// - **Task Actors**: Execute individual plugin tasks (configurable pool size)
/// - **Workflow Actors**: Orchestrate workflow execution and coordinate task actors
/// - **Actor Pool**: Manages load balancing across task actors using round-robin
/// - **Message System**: Type-safe communication between actors using oneshot channels
///
/// # Configuration
///
/// Pool sizes can be configured via environment variables:
/// - `TASK_ACTOR_POOL_SIZE`: Number of task actors (default: 50)
/// - `WORKFLOW_ACTOR_POOL_SIZE`: Number of workflow actors (default: 10)
///
/// # Benefits over Enhanced Processor
///
/// - **Higher Parallelism**: Independent actors can process tasks concurrently
/// - **Better Fault Isolation**: Actor failures don't affect other actors
/// - **Improved Load Balancing**: Round-robin distribution across actor pools
/// - **Enhanced Observability**: Per-actor tracing and metrics
/// - **Graceful Shutdown**: Coordinated shutdown of all actors
/// - **Configurable Scaling**: Environment-based pool sizing
pub async fn actor_processor(
    state: Arc<AppState>,
    processor_receiver: mpsc::Receiver<ProcessorMessage>,
) -> Result<(), ProcessorError> {
    let processor = ActorProcessor::new(state);
    processor.run(processor_receiver).await
}

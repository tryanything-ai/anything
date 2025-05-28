# Actor-Based Workflow Processor

This module implements a highly parallel, asynchronous workflow processor using the actor model pattern. It replaces the enhanced processor with a more scalable and fault-tolerant design.

## Architecture

The actor processor consists of several key components:

### Core Components

- **TaskActor** (`task_actor.rs`): Executes individual plugin tasks with timeout handling
- **WorkflowActor** (`workflow_actor.rs`): Orchestrates workflow execution and coordinates task actors
- **TaskActorPool** (`actor_pool.rs`): Manages a pool of task actors with round-robin load balancing
- **ActorProcessor** (`actor_system.rs`): Main coordinator that manages workflow actors and the task pool
- **ActorMessage** (`messages.rs`): Type-safe message definitions for actor communication

### Message Flow

```
ProcessorMessage → ActorProcessor → WorkflowActor → TaskActorPool → TaskActor
                                                                      ↓
                                                                 execute_task()
                                                                      ↓
                                                                   Plugin
```

## Configuration

The actor system can be configured via environment variables:

- `TASK_ACTOR_POOL_SIZE`: Number of task actors (default: 50)
- `WORKFLOW_ACTOR_POOL_SIZE`: Number of workflow actors (default: 10)

## Benefits over Enhanced Processor

### Higher Parallelism

- Independent actors can process tasks concurrently without blocking
- Configurable pool sizes allow tuning for different workloads
- Round-robin load balancing distributes work evenly

### Better Fault Isolation

- Actor failures don't affect other actors
- Failed tasks don't crash the entire processor
- Timeout handling prevents stuck tasks from blocking others

### Enhanced Observability

- Per-actor tracing with unique actor IDs
- Detailed metrics for task execution times
- Workflow-level and task-level span tracking

### Graceful Shutdown

- Coordinated shutdown of all actors
- In-flight tasks are allowed to complete
- Clean resource cleanup

## Usage

The actor processor is used as a drop-in replacement for the enhanced processor:

```rust
use crate::actor_processor::actor_processor;

// Spawn the actor processor
tokio::spawn(async move {
    if let Err(e) = actor_processor(state, processor_rx).await {
        error!("Actor processor failed: {}", e);
    }
});
```

## Monitoring

The actor processor provides several metrics:

- `anything_processor_messages_received_total`: Total messages processed
- `anything_workflow_processing_duration_seconds`: Workflow execution time
- `anything_task_execution_duration_seconds`: Individual task execution time
- `anything_processor_workflow_errors_total`: Error count

## Testing

Basic tests are provided in `tests.rs`. To run them:

```bash
cargo test actor_processor
```

## Future Enhancements

- **Dependency Resolution**: Implement proper task dependency graphs
- **Backpressure**: Add flow control when actors are overloaded
- **Dynamic Scaling**: Automatically adjust pool sizes based on load
- **Persistence**: Add actor state persistence for crash recovery
- **Circuit Breakers**: Implement circuit breaker pattern for failing plugins

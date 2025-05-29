# Actor-Based Processor System

This module implements an actor-based processing system for executing workflows and tasks in a scalable, fault-tolerant manner.

## Architecture Overview

The system consists of three main components:

1. **ActorProcessor** - The main coordinator that receives workflow execution requests
2. **WorkflowActors** - Actors that orchestrate workflow execution
3. **TaskActors** - Actors that execute individual tasks

```
┌─────────────────┐
│ ActorProcessor  │
└────────┬────────┘
         │
    ┌────▼────┐
    │ Round   │
    │ Robin   │
    └────┬────┘
         │
┌────────▼─────────┬─────────────────┬─────────────────┐
│ WorkflowActor 1  │ WorkflowActor 2 │ WorkflowActor N │
└────────┬─────────┴────────┬────────┴────────┬────────┘
         │                  │                  │
         └──────────┬───────┴──────────────────┘
                    │
              ┌─────▼─────┐
              │   Task    │
              │   Pool    │
              └─────┬─────┘
                    │
      ┌─────────────┼─────────────┐
      │             │             │
┌─────▼─────┬──────▼──────┬──────▼──────┐
│TaskActor 1│ TaskActor 2 │ TaskActor M │
└───────────┴─────────────┴─────────────┘
```

## Key Features

- **Load Balancing**: Round-robin distribution of workflows and tasks
- **Scalability**: Configurable pool sizes for workflow and task actors
- **Fault Isolation**: Errors in one task don't affect others
- **Observability**: Integrated tracing and metrics

## Configuration

Set these environment variables to configure the actor pools:

- `WORKFLOW_ACTOR_POOL_SIZE`: Number of workflow actors (default: 10)
- `TASK_ACTOR_POOL_SIZE`: Number of task actors (default: 50)

## Implementation Details

### WorkflowActor

Responsible for:

- Managing workflow state
- Orchestrating task execution
- Handling task dependencies
- Error recovery and retries

### TaskActor

Responsible for:

- Executing individual tasks
- Timeout management
- Result reporting
- Resource cleanup

### Message Types

The system uses the following message types:

- `ExecuteWorkflow`: Start workflow execution
- `ExecuteTask`: Execute a single task
- `Shutdown`: Gracefully shutdown an actor

## Performance Considerations

- Task actors use a larger pool size since tasks are the unit of work
- Workflow actors use a smaller pool since they primarily orchestrate
- Channel sizes are tuned to prevent backpressure

## RustyScript (JavaScript) Execution

For details on JavaScript/TypeScript execution optimization and troubleshooting, see [RUSTYSCRIPT_OPTIMIZATION.md](./RUSTYSCRIPT_OPTIMIZATION.md).

## Future Improvements

- [ ] Dynamic pool sizing based on load
- [ ] Priority-based task scheduling
- [ ] Task result caching
- [ ] Distributed actor support

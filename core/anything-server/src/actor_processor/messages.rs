use crate::processor::components::{ProcessorError, WorkflowExecutionContext};
use crate::processor::execute_task::TaskResult;
use crate::processor::processor::ProcessorMessage;
use crate::types::task_types::Task;
use tokio::sync::oneshot;

/// Messages that can be sent to actors
#[derive(Debug)]
pub enum ActorMessage {
    /// Execute a single task
    ExecuteTask {
        task: Task,
        respond_to: oneshot::Sender<TaskResult>,
        context: WorkflowExecutionContext,
    },
    /// Execute a workflow (collection of tasks)
    ExecuteWorkflow {
        message: ProcessorMessage,
        respond_to: oneshot::Sender<Result<(), ProcessorError>>,
    },
    /// Shutdown the actor
    Shutdown,
}

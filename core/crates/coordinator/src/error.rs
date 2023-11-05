use anything_runtime::RuntimeError;
use thiserror::Error;

pub type CoordinatorResult<T> = Result<T, CoordinatorError>;

#[derive(Debug, Error)]
pub enum CoordinatorError {
    #[error("manager not ready to start yet")]
    ManagerNotPrepared,

    #[error("runtime error")]
    RuntimeError,

    #[error("internal error: {0}")]
    InternalError(String),

    #[error(transparent)]
    RunnerError(#[from] RuntimeError),

    #[error("error running graph")]
    GraphRunTaskError,

    #[error("error running graph: {0}")]
    TaskExecutionError(String),

    #[error("io error")]
    IoError(#[from] std::io::Error),

    #[error("Anything error")]
    AnythingError(#[from] anything_common::error::AnythingError),

    #[error("No flows found")]
    NoFlowsFound,

    #[error("Flow not found: {0}")]
    FlowNotFound(String),
}

impl<M> From<postage::sink::SendError<M>> for CoordinatorError {
    fn from(_value: postage::sink::SendError<M>) -> Self {
        CoordinatorError::RuntimeError
    }
}

impl From<anyhow::Error> for CoordinatorError {
    fn from(error: anyhow::Error) -> Self {
        CoordinatorError::InternalError(error.to_string())
    }
}

impl From<tokio::sync::TryLockError> for CoordinatorError {
    fn from(value: tokio::sync::TryLockError) -> Self {
        CoordinatorError::InternalError(value.to_string())
    }
}

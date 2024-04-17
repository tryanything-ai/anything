use anything_persistence::error::PersistenceError;
use anything_runtime::RuntimeError;
use ractor::ActorProcessingErr;
use thiserror::Error;

// use crate::processing::processor::ProcessorMessage;

pub type CoordinatorResult<T> = Result<T, CoordinatorError>;
pub type CoordinatorActorResult<T> = Result<T, ActorProcessingErr>;

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

    #[error("repo not initialized")]
    RepoNotInitialized,

    #[error("persistence error: {0}")]
    PersistenceError(PersistenceError),

    #[error("actor error: {0}")]
    ActorNotInitialized(String),

    // #[error("processor send error {0}")]
    // ProcessorSendError(tokio::sync::mpsc::error::SendError<ProcessorMessage>),
    #[error("processor execution error: {0}")]
    ProcessorExecutionError(String),

    #[error("processor error: {0}")]
    ParsingError(String),
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

impl From<PersistenceError> for CoordinatorError {
    fn from(value: PersistenceError) -> Self {
        CoordinatorError::PersistenceError(value)
    }
}

// impl From<tokio::sync::mpsc::error::SendError<ProcessorMessage>> for CoordinatorError {
//     fn from(e: tokio::sync::mpsc::error::SendError<ProcessorMessage>) -> Self {
//         CoordinatorError::ProcessorSendError(e)
//     }
// }

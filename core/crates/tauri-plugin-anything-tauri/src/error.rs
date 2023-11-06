use anything_coordinator::CoordinatorError;
use anything_persistence::error::PersistenceError;
use serde::{Deserialize, Serialize};

pub type FlowResult<T> = Result<T, Error>;
pub type EventResult<T> = Result<T, Error>;

/// The error types.
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Error {
    #[error("Coordinator not initialized")]
    CoordinatorNotInitialized,

    #[error("No flows found")]
    NoFlowsFound,

    #[error("Runtime error")]
    RuntimeError,

    #[error("Persistence error: {0}")]
    PersistenceError(String),
}

impl From<CoordinatorError> for Error {
    fn from(value: CoordinatorError) -> Self {
        match value {
            CoordinatorError::NoFlowsFound => Error::NoFlowsFound,
            _ => Error::NoFlowsFound,
        }
    }
}

impl From<PersistenceError> for Error {
    fn from(value: PersistenceError) -> Self {
        match value {
            PersistenceError::DatabaseError(e) => Error::PersistenceError(e.to_string()),
            PersistenceError::InvalidDatabaseType => {
                Error::PersistenceError("Invalid database type".to_string())
            }
            PersistenceError::MigrationError(e) => Error::PersistenceError(e.to_string()),
            _ => Error::PersistenceError("Unknown error".to_string()),
        }
    }
}

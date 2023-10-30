use anything_coordinator::CoordinatorError;
use serde::{Deserialize, Serialize};

/// The error types.
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Error {
    #[error("Coordinator not initialized")]
    CoordinatorNotInitialized,

    #[error("No flows found")]
    NoFlowsFound,
}

impl From<CoordinatorError> for Error {
    fn from(value: CoordinatorError) -> Self {
        match value {
            CoordinatorError::NoFlowsFound => Error::NoFlowsFound,
            _ => Error::NoFlowsFound,
        }
    }
}

use tokio::task::JoinError;

use crate::types::ChangeMessage;

pub type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug)]
pub enum StoreError {
    UnableToCreateDirectory {
        path: std::path::PathBuf,
        err: std::io::Error,
    },
    UnableToDeleteDirectory {
        path: std::path::PathBuf,
        err: std::io::Error,
    },
    UnableToWriteFile {
        path: std::path::PathBuf,
        err: std::io::Error,
    },
    UnableToReadDirectory {
        path: std::path::PathBuf,
        err: std::io::Error,
    },
    UnableToReadFile {
        path: std::path::PathBuf,
        err: std::io::Error,
    },
    NotifierError(String),
    InternalError(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::UnableToCreateDirectory { path, err } => {
                write!(f, "Unable to create directory {}: {}", path.display(), err)
            }
            StoreError::UnableToDeleteDirectory { path, err } => {
                write!(f, "Unable to delete directory {}: {}", path.display(), err)
            }
            StoreError::UnableToReadDirectory { path, err } => {
                write!(f, "Unable to read directory {}: {}", path.display(), err)
            }
            StoreError::UnableToWriteFile { path, err } => {
                write!(f, "Unable to write file {}: {}", path.display(), err)
            }
            StoreError::UnableToReadFile { path, err } => {
                write!(f, "Unable to read file {}: {}", path.display(), err)
            }
            StoreError::NotifierError(msg) => write!(f, "Notifier error: {}", msg),
            StoreError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl From<notify::Error> for StoreError {
    fn from(value: notify::Error) -> Self {
        StoreError::InternalError(value.to_string())
    }
}

impl From<JoinError> for StoreError {
    fn from(value: JoinError) -> Self {
        StoreError::InternalError(value.to_string())
    }
}

impl From<tokio::sync::mpsc::error::SendError<ChangeMessage>> for StoreError {
    fn from(value: tokio::sync::mpsc::error::SendError<ChangeMessage>) -> Self {
        StoreError::NotifierError(value.to_string())
    }
}

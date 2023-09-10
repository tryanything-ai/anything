use anyhow::Error;
use anything_core::error::AnythingError;
use thiserror::Error;

pub type EventsResult<T> = Result<T, EventsError>;

#[derive(Error, Debug)]
pub enum EventsError {
    #[error("config error: {0}")]
    ConfigLibraryError(#[from] config::ConfigError),

    #[error("configuration error: {0}")]
    ConfigError(String),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error("configuration error")]
    ConfigurationError(#[from] AnythingError),

    #[error(transparent)]
    DecodingError(#[from] serde_json::Error),

    #[error("encoding error")]
    EncodingError,
}

use anything_core::error::AnythingError;
use thiserror::Error;

pub type EventsResult<T> = Result<T, EventsError>;

#[derive(Error, Debug)]
pub enum EventsError {
    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error("configuration error")]
    ConfigurationError(#[from] AnythingError),
}

use anything_core::error::AnythingError;
use sqlx::error::BoxDynError;
use thiserror::Error;

pub type EventsResult<T> = Result<T, EventsError>;

#[derive(Error, Debug)]
pub enum EventsError {
    #[error("config error: {0}")]
    ConfigLibraryError(#[from] config::ConfigError),

    #[error("configuration error: {0}")]
    ConfigError(String),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("migration error {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("server error")]
    EventServerError(#[from] tonic::transport::Error),

    #[error("configuration error")]
    ConfigurationError(#[from] AnythingError),

    #[error(transparent)]
    DecodingError(#[from] serde_json::Error),

    #[error("encoding error")]
    EncodingError,

    #[error("not found: {0}")]
    NotFoundError(String),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("{0}")]
    DBError(#[source] BoxDynError),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

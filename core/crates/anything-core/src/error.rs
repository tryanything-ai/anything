use thiserror::Error;

pub type AnythingResult<T> = Result<T, AnythingError>;

#[derive(Debug, Error)]
pub enum AnythingError {
    #[error("Database error")]
    DB(DatabaseError),

    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error("message decoding error")]
    MessageDecodingError,

    #[error("invalid server config")]
    InvalidServerConfigError,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DatabaseError {
    #[error("Database is not available")]
    NotAvailable,
}

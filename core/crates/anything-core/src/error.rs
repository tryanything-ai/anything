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

    #[error("error in trigger: {0}")]
    TriggerError(String),

    #[error("parsing error: {0}")]
    ParsingError(String),

    #[error(transparent)]
    JsonParsingError(#[from] serde_json::Error),

    #[error(transparent)]
    UrlParsingError(#[from] url::ParseError),

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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnythingError {
    #[error("validation did not pass: {0}")]
    ValidationError(String),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
}

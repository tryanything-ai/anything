use thiserror::Error;

pub type AnythingResult<T> = Result<T, AnythingError>;

#[derive(Debug, Error)]
pub enum AnythingError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("not found: {0}")]
    NotFoundError(String),
}

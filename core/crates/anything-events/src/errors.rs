use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventurousError {
    #[error("config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

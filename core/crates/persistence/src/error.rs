use anything_store::errors::StoreError as FileStoreError;
use sqlx::{migrate::MigrateError, sqlite::SqliteError};
use thiserror::Error;
pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("Migration error")]
    MigrationError(#[from] MigrateError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Flow not found: {0}")]
    FlowNotFound(String),

    #[error("Unable to read flow in directory")]
    UnableToReadFlow,

    #[error("Invalid database")]
    InvalidDatabaseType,

    #[error("Runtime error")]
    RuntimeError,

    #[error("sqlx error: {0}")]
    SqlxError(SqliteError),

    #[error("file store error: {0}")]
    StoreError(FileStoreError),
}

impl From<SqliteError> for PersistenceError {
    fn from(e: SqliteError) -> Self {
        Self::SqlxError(e)
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(_value: std::io::Error) -> Self {
        Self::RuntimeError
    }
}

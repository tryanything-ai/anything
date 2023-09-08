use thiserror::Error;

pub type AnythingResult<T> = Result<T, AnythingError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnythingError {
    #[error("Database error")]
    DB(DatabaseError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DatabaseError {
    #[error("Database is not available")]
    NotAvailable,
}

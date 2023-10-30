use thiserror::Error;

pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[derive(Error, Debug)]
pub enum PersistenceError {}

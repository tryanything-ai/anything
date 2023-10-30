use std::sync::TryLockError;

use thiserror::Error;

pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type PluginResult<T> = Result<T, PluginError>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("error parsing file")]
    ParseError,
    #[error("render error: {0}")]
    RenderError(#[from] tera::Error),
    #[error("invalid variable name")]
    InvalidVariableName,
    #[error("empty variable value")]
    EmptyVariableValue,
    #[error("scope error")]
    ScopeError,
    #[error("invalid interpreter")]
    InvalidInterpreter,
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("code execution error: {0}")]
    Code(i32),
    #[error("no code provided")]
    NoCodeProvided,
    #[error("code status error")]
    Signal,
    #[error("plugin manager error")]
    PluginManagerError,
    #[error("error getting options for plugin")]
    PluginOptionError,
    #[error(transparent)]
    PluginError(#[from] PluginError),
    #[error("unimplemented")]
    Unimplemented,
    #[error("empty task command")]
    EmptyTaskCommand,
    #[error("no execution result found")]
    NoExecutionResultFound,
    #[error("runtime error")]
    RuntimeError,
}

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("runtime error")]
    RuntimeError(#[from] std::io::Error),
    #[error("anyhow plugin error: {0}")]
    AnythingError(#[from] anyhow::Error),
    #[error("plugin load error: {0}")]
    LoadingError(#[from] libloading::Error),
    #[error("plugin not found: {0}")]
    NotFound(String),
    #[error("execution error: {0}")]
    Custom(String),
}

impl<T> From<TryLockError<T>> for RuntimeError {
    fn from(_value: TryLockError<T>) -> Self {
        RuntimeError::RuntimeError
    }
}

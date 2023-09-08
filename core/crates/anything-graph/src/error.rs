use std::{io::Error as IoError, path::StripPrefixError, process::ExitStatus};
use thiserror::Error;

#[cfg(feature = "tracing")]
use tracing::error;

use tera::Error as TeraError;

use crate::flow::node::NodeBuilderError;

pub type AppResult<T> = anyhow::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Panic: {0}")]
    Panic(String),

    #[error("Config error: {0}")]
    ConfigError(ConfigError),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error(transparent)]
    IOError(IoError),

    #[error("Flow error: {0}")]
    FlowError(String),

    #[error("Flow node error: {0}")]
    FlowNodeError(String),

    #[error("Step action run error: {0}")]
    ActionError(ActionError),
}

impl From<NodeBuilderError> for AppError {
    fn from(value: NodeBuilderError) -> Self {
        AppError::FlowError(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Signal: {0}")]
    Signal(ExitStatus),

    #[error("Error running: {0}")]
    RunError(String),

    #[error("IO Error")]
    IOError(IoError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid variable modifier: {0}")]
    InvalidVariableModifier(String),

    #[error("Error rendering template: {0}")]
    TemplateRenderError(TeraError),

    #[error("Error invalid key: {0}")]
    InvalidKey(String),

    #[error("Error invalid value: {0}")]
    InvalidValue(String),

    #[error("Error empty value: {0}")]
    EmptyValue(String),

    #[error("variable type unimplemented: {0}")]
    VariableTypeNotImplemented(String),
}

impl From<tera::Error> for ConfigError {
    fn from(value: tera::Error) -> Self {
        ConfigError::TemplateRenderError(value)
    }
}

impl From<tera::Error> for AppError {
    fn from(value: tera::Error) -> Self {
        AppError::ConfigError(ConfigError::TemplateRenderError(value))
    }
}

impl From<StripPrefixError> for AppError {
    fn from(_: StripPrefixError) -> Self {
        AppError::RuntimeError("Unable to strip prefix".to_string())
    }
}

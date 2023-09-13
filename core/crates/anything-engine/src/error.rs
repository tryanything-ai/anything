use anything_graph::error::AppError;
use miette::{Diagnostic, ErrReport};
use thiserror::Error;

pub type EngineResult<T> = Result<T, EngineError>;

#[derive(Error, Debug, Diagnostic)]
pub enum EngineError {
    #[error("shell error: {0}")]
    ShellError(String),

    #[error("process has not been run")]
    ShellProcessHasNotRunError,

    #[error("copy files error")]
    CopyFilesError(#[from] fs_extra::error::Error),

    #[error("error running flow")]
    FlowRunError(#[from] AppError),
}

impl From<ErrReport> for EngineError {
    fn from(value: ErrReport) -> Self {
        Self::ShellError(value.to_string())
    }
}

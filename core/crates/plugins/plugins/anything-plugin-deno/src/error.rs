use std::fmt::{Debug, Display};

use anything_runtime::PluginError;
use deno_runtime::deno_core::ModuleResolutionError;

pub type DenoPluginResult<T> = Result<T, DenoPluginError>;

pub enum DenoPluginError {
    ParamsIsNotAnObject,
    StdError(std::io::Error),
    NoDefaultExportFound,
}

impl Into<String> for DenoPluginError {
    fn into(self) -> String {
        match self {
            Self::ParamsIsNotAnObject => "Params is not an object".to_string(),
            Self::NoDefaultExportFound => "No default export found".to_string(),
            Self::StdError(e) => e.to_string(),
        }
    }
}

impl Into<PluginError> for DenoPluginError {
    fn into(self) -> PluginError {
        PluginError::Custom(format!("{}", self.to_string()))
    }
}

impl Debug for DenoPluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DenoPluginError::ParamsIsNotAnObject => write!(f, "Params is not an object"),
            DenoPluginError::NoDefaultExportFound => write!(f, "No default export found"),
            DenoPluginError::StdError(e) => write!(f, "{}", e),
        }
    }
}

impl Display for DenoPluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DenoPluginError::ParamsIsNotAnObject => write!(f, "Params is not an object"),
            DenoPluginError::NoDefaultExportFound => write!(f, "No default export found"),
            DenoPluginError::StdError(e) => write!(f, "{}", e),
        }
    }
}

impl From<anyhow::Error> for DenoPluginError {
    fn from(error: anyhow::Error) -> Self {
        Self::StdError(std::io::Error::new(std::io::ErrorKind::Other, error))
    }
}

impl From<ModuleResolutionError> for DenoPluginError {
    fn from(value: ModuleResolutionError) -> Self {
        Self::StdError(std::io::Error::new(std::io::ErrorKind::Other, value))
    }
}

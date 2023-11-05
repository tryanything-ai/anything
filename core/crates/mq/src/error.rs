use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ClientId;

pub type MqResult<T> = Result<T, MqError>;

#[derive(Error, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum MqError {
    #[error("invalid topic string: {0}")]
    InvalidTopicString(String),

    #[error("closed channel by peer")]
    ChannelClosed,

    #[error("failed to deliver to client {0}: {1}")]
    FailedtoDeliverToClientError(ClientId, String),

    #[error("request failed: {0}")]
    RequestFailed(String),

    #[error("runtime error")]
    RuntimeError,

    #[error("request timeout error")]
    RequestTimeoutError,

    #[error("unknown client: {0}")]
    UnknownClientError(ClientId),

    #[error("Deserialize message failed. Protocol: {0}. Error: {1}")]
    MalformedMessage(String, String),
    #[error("Serialize message failed: {0}")]
    UnserializableMessage(String),

    #[error("IO error: {0}")]
    IOError(String),
    #[error("internal error: {0}")]
    InternalError(String),
}

impl From<tokio::task::JoinError> for MqError {
    fn from(value: tokio::task::JoinError) -> Self {
        MqError::InternalError(value.to_string())
    }
}

impl From<serde_json::error::Error> for MqError {
    fn from(error: serde_json::error::Error) -> Self {
        MqError::UnserializableMessage(error.to_string())
    }
}

impl From<serde_cbor::error::Error> for MqError {
    fn from(error: serde_cbor::error::Error) -> Self {
        MqError::UnserializableMessage(error.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for MqError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        MqError::InternalError(error.to_string())
    }
}

impl From<anyhow::Error> for MqError {
    fn from(error: anyhow::Error) -> Self {
        MqError::InternalError(error.to_string())
    }
}

impl<M> From<async_channel::SendError<M>> for MqError {
    fn from(_value: async_channel::SendError<M>) -> Self {
        MqError::RuntimeError
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for MqError {
    fn from(_error: tokio::sync::mpsc::error::SendError<T>) -> Self {
        MqError::ChannelClosed
    }
}

impl From<std::io::Error> for MqError {
    fn from(error: std::io::Error) -> Self {
        MqError::IOError(error.to_string())
    }
}

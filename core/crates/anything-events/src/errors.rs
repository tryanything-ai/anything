use crate::prelude::*;
use anything_core::error::AnythingError;
use thiserror::Error;

use crate::internal_notification::ShutdownNotification;

pub type EventsResult<T> = Result<T, EventsError>;

#[derive(Error, Debug)]
pub enum EventsError {
    #[error("config error: {0}")]
    ConfigLibraryError(#[from] config::ConfigError),

    #[error("configuration error: {0}")]
    ConfigError(String),

    #[error("tcp address parse error: {0}")]
    TcpListeningError(#[from] std::net::AddrParseError),

    #[error("postoffice send error: {0}")]
    PostOfficeSendError(#[from] postage::sink::SendError<ShutdownNotification>),
    #[error("trigger send error: {0}")]
    TriggerSendError(#[from] postage::sink::SendError<Trigger>),
    #[error("event send error: {0}")]
    EventSendError(#[from] postage::sink::SendError<Event>),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error("sending update error")]
    SenderError,

    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("migration error {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("server error")]
    EventServerError(#[from] tonic::transport::Error),

    #[error("trigger error: {0}")]
    TriggerError(String),

    #[error("configuration error")]
    ConfigurationError(#[from] AnythingError),

    #[error(transparent)]
    DecodingError(#[from] serde_json::Error),

    #[error("encoding error")]
    EncodingError,

    #[error("not found: {0}")]
    NotFoundError(String),
}

impl From<EventsError> for tonic::Status {
    fn from(value: EventsError) -> Self {
        match value {
            EventsError::SenderError => tonic::Status::internal("Sender error"),
            _ => tonic::Status::internal(value.to_string()),
        }
    }
}

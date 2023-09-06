use thiserror::Error;
use tmq::{publish::Publish, TmqError};

#[derive(Error, Debug)]
pub enum AnythingError {
    #[error("capacity must be a power of 2")]
    InvalidRingBufferCapacityError,

    #[error("capcity must be greater than 0 and less than buffer size")]
    RingCapacityOutOfBoundsError,

    #[error(transparent)]
    DeviceError(#[from] DeviceError),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error(transparent)]
    SocketError(#[from] zmq::Error),

    #[error("unable to publish")]
    UnableToPublishError,

    #[error("unable to subscribe")]
    UnableToSubscribeError,

    #[error("already publishing on this socket")]
    AlreadyPublishingError,

    #[error("already subscribing on this socket")]
    AlreadySubscribingError,

    #[error("not subscribed or published")]
    DeviceNotAvailableError,
}

// impl From<TmqError> for DeviceError {
//     fn from(value: TmqError) -> Self {
//         DeviceError::SocketError(value)
//     }
// }

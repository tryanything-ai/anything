use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AnythingError {
    #[error("capacity must be a power of 2")]
    InvalidRingBufferCapacityError,

    #[error("capcity must be greater than 0 and less than buffer size")]
    RingCapacityOutOfBoundsError,
}

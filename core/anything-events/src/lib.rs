pub mod event;

pub mod types;

pub mod error;
pub(crate) mod utils;
pub(crate) mod validation;

mod pb {
    tonic::include_proto!("event");
}

pub mod handler;

mod models;
mod observable;

#[cfg(test)]
pub(crate) mod internal;

pub use observable::delegate::{Delegate, Response, Subscription};
pub use observable::Observable;

pub use models::*;

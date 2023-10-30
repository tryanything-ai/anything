pub(crate) mod config;
pub mod error;
pub(crate) mod events;
pub(crate) mod handlers;
pub(crate) mod models;
pub(crate) mod processing;

#[cfg(debug_assertions)]
pub(crate) mod test_helper;

pub mod manager;
pub use config::AnythingConfig;
pub use error::*;
pub use manager::{start, Manager};

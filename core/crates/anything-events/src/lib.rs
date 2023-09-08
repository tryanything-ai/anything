use errors::EventurousError;

pub type EvtResult<T> = std::result::Result<T, EventurousError>;

pub(crate) mod bootstrap;
pub mod cli;
pub mod config;
pub(crate) mod context;
pub(crate) mod errors;
pub(crate) mod executor;
pub(crate) mod macros;
pub(crate) mod models;
pub(crate) mod post_office;
pub(crate) mod server;
pub(crate) mod store;
pub(crate) mod utils;

pub mod messages;

#[cfg(test)]
pub(crate) mod internal;

pub(crate) mod constants;

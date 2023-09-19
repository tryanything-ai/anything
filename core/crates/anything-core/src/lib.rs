extern crate lazy_static;

pub mod config;
pub mod error;
pub mod macros;
pub mod spawning;
pub mod tracing;
pub mod utils;

pub use config::AnythingConfig;
pub use spawning::{build_runtime, spawn_or_crash};
pub mod posix;

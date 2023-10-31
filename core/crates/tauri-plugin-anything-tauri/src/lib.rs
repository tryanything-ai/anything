use std::sync::Arc;

use anything_coordinator::{AnythingConfig, Manager as AnythingManager};
use tauri::async_runtime::Mutex;

mod anything;
mod builder;
mod error;

pub use anything::*;
pub use error::Error;

pub use builder::{Anything, Builder as AnythingBuilder};

#[derive(Default)]
pub struct AnythingState {
    inner: Mutex<Arc<AnythingManager>>,
    stop_tx: Option<tokio::sync::mpsc::Sender<()>>,
    anything_config: AnythingConfig,
}

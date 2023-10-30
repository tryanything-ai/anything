use std::sync::Arc;

use anything_coordinator::{start, AnythingConfig, Manager as AnythingManager};
use tauri::{AppHandle, Runtime};

pub async fn init_anything<R: Runtime>(
    _app: AppHandle<R>,
    stop_rx: tokio::sync::mpsc::Receiver<()>,
    ready_tx: tokio::sync::mpsc::Sender<Arc<AnythingManager>>,
) {
    let anything_config = AnythingConfig::default();
    // let (stop_tx, stop_rx) = tokio::sync::mpsc::channel(1);
    // let (ready_tx, mut ready_rx) = tokio::sync::mpsc::channel(1);

    let ready_tx_clone = ready_tx.clone();
    tauri::async_runtime::spawn(async move {
        start(anything_config, stop_rx, ready_tx_clone)
            .await
            .unwrap();
    });

    // app.manage(init_state);
}

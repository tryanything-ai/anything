use std::sync::Arc;

use anything_coordinator::{start, AnythingConfig, Manager as AnythingManager};
use tauri::{AppHandle, Runtime};

pub async fn init_anything<R: Runtime>(
    _app: AppHandle<R>,
    anything_config: AnythingConfig,
    stop_rx: tokio::sync::mpsc::Receiver<()>,
    ready_tx: tokio::sync::mpsc::Sender<Arc<AnythingManager>>,
) {
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

pub struct AnythingBuilder<R: Runtime> {
    app: AppHandle<R>,
    anything_config: AnythingConfig,
}

impl<R: Runtime> AnythingBuilder<R> {
    pub fn new(app: AppHandle<R>, anything_config: AnythingConfig) -> Self {
        Self {
            app,
            anything_config,
        }
    }

    pub fn build(self) -> Anything<R> {
        Anything {
            app: self.app,
            config: self.anything_config,
        }
    }
}

#[derive(Clone)]
pub struct Anything<R: Runtime> {
    app: AppHandle<R>,
    config: AnythingConfig,
}

impl<R: Runtime> Anything<R> {
    pub fn builder(app: AppHandle<R>, anything_config: AnythingConfig) -> AnythingBuilder<R> {
        AnythingBuilder::new(app, anything_config)
    }

    pub async fn start(&self) -> Arc<AnythingManager> {
        let (stop_tx, stop_rx) = tokio::sync::mpsc::channel(1);
        let (ready_tx, mut ready_rx) = tokio::sync::mpsc::channel(1);

        let ready_tx_clone = ready_tx.clone();
        let config = self.config.clone();
        tauri::async_runtime::spawn(async move {
            start(config, stop_rx, ready_tx_clone).await.unwrap();
        });

        let arc_manager = ready_rx.recv().await;
        let arc_manager = arc_manager.unwrap().clone();
        arc_manager
    }
}

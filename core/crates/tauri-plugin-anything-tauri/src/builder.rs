use std::sync::Arc;

use crate::{anything::*, AnythingState};
use anything_common::{setup_tracing, AnythingConfig};
use anything_coordinator::{manager::start, Manager as AnythingManager};
use tauri::{
    plugin::{self, TauriPlugin},
    AppHandle, Manager, Runtime,
};
use tokio::sync::Mutex;

pub struct Builder<R: Runtime> {
    app: Option<AppHandle<R>>,
    anything_config: AnythingConfig,
}

impl<R: Runtime> Default for Builder<R> {
    fn default() -> Self {
        Self {
            app: None,
            anything_config: AnythingConfig::default(),
        }
    }
}

impl<R: Runtime> Builder<R> {
    pub fn new(app: AppHandle<R>, anything_config: AnythingConfig) -> Self {
        Self {
            app: Some(app),
            anything_config,
        }
    }

    pub fn config(mut self, anything_config: AnythingConfig) -> Self {
        self.anything_config = anything_config;
        self
    }

    pub fn build(self) -> TauriPlugin<R> {
        plugin::Builder::new("anything")
            .invoke_handler(tauri::generate_handler![
                initialize,
                setup,
                stop,
                get_flows,
                get_flow_by_name,
                create_flow,
                update_flow,
                create_flow_version,
                execute_flow,
            ])
            .setup(move |app_handle| {
                let (stop_tx, stop_rx) = tokio::sync::mpsc::channel(1);
                let (ready_tx, mut ready_rx) = tokio::sync::mpsc::channel(1);
                let anything_config = self.anything_config.clone();

                let ready_tx_clone = ready_tx.clone();
                tauri::async_runtime::spawn(async move {
                    start(anything_config, stop_rx, ready_tx_clone)
                        .await
                        .unwrap();
                });

                setup_tracing("anything")?;

                let anything_config = self.anything_config.clone();
                tauri::async_runtime::block_on(async move {
                    let arc_manager = match ready_rx.recv().await {
                        Some(arc_manager) => arc_manager,
                        None => {
                            panic!("Failed to start anything");
                        }
                    };
                    let inner = match Arc::try_unwrap(arc_manager) {
                        Ok(inner) => Arc::new(Mutex::new(inner)),
                        Err(_) => {
                            panic!("Failed to start anything");
                        }
                    };
                    // arc_manager.refresh_flows().await.unwrap();
                    let init_state = AnythingState {
                        inner,
                        stop_tx: Some(stop_tx),
                        anything_config,
                    };
                    println!("Anything is ready to go!");
                    app_handle.manage(init_state);
                });
                Ok(())
            })
            .build()
    }
}

#[derive(Clone)]
pub struct Anything<R: Runtime> {
    app: AppHandle<R>,
    config: AnythingConfig,
}

impl<R: Runtime> Anything<R> {
    pub fn builder(app: AppHandle<R>, anything_config: AnythingConfig) -> Builder<R> {
        Builder::new(app, anything_config)
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

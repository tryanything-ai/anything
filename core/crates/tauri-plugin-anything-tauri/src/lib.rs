use std::sync::Arc;

use anything_coordinator::{start, AnythingConfig, Manager as AnythingManager};
use init::init_anything;
use tauri::{
    async_runtime::Mutex,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, RunEvent, Runtime,
};

mod error;
mod flows;
mod init;
pub use error::Error;
pub use flows::*;

#[derive(Default)]
pub struct AnythingState {
    inner: Mutex<Arc<AnythingManager>>,
    stop_tx: Option<tokio::sync::mpsc::Sender<()>>,
}

#[tauri::command]
fn initialize() {
    println!("In initialize for anything tauri plugin");
}

#[tauri::command]
async fn setup() {
    // println!("In initialize for anything tauri plugin");

    // let (stop_tx, stop_rx) = tokio::sync::mpsc::channel(1);
    // let (ready_tx, mut ready_rx) = tokio::sync::mpsc::channel(1);
    // let anything_config = AnythingConfig::default();

    // let ready_tx_clone = ready_tx.clone();
    // tauri::async_runtime::spawn(async move {
    //     start(anything_config, stop_rx, ready_tx_clone)
    //         .await
    //         .unwrap();
    // });

    // tauri::async_runtime::block_on(async move {
    //     let arc_manager = ready_rx.recv().await;
    //     let arc_manager = arc_manager.unwrap().clone();
    //     let init_state = AnythingState {
    //         inner: Mutex::new(arc_manager.clone()),
    //         stop_tx: Some(stop_tx),
    //     };
    //     println!("Anything is ready to go!");
    //     app_handle.manage(init_state);
    // });
}

#[tauri::command]
async fn stop(state: tauri::State<'_, AnythingState>) -> Result<(), Error> {
    let stop_tx = state.stop_tx.as_ref().unwrap();
    stop_tx.send(()).await.unwrap();
    Ok(())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("anything")
        .invoke_handler(tauri::generate_handler![
            initialize,
            setup,
            stop,
            get_flows,
            create_flow
        ])
        .setup(|app_handle| {
            let (stop_tx, stop_rx) = tokio::sync::mpsc::channel(1);
            let (ready_tx, mut ready_rx) = tokio::sync::mpsc::channel(1);
            let anything_config = AnythingConfig::default();

            tauri::async_runtime::spawn(init_anything::<R>(
                app_handle.clone(),
                anything_config,
                stop_rx,
                ready_tx,
            ));
            tauri::async_runtime::block_on(async move {
                let arc_manager = ready_rx.recv().await;
                let arc_manager = arc_manager.unwrap().clone();
                let init_state = AnythingState {
                    inner: Mutex::new(arc_manager.clone()),
                    stop_tx: Some(stop_tx),
                };
                println!("Anything is ready to go!");
                app_handle.manage(init_state);
            });
            Ok(())
        })
        .on_event(|_app_handle, event| {
            if let RunEvent::Exit = event {
                // let state = app_handle.state::<AppStateProc<R>>().unwrap();
                println!("Closing down shop...");
            }
        })
        .build()
}

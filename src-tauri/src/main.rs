// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sql;
mod events;
mod local_models;

use local_models::config::get_logs_dir; 
use local_models::models::ModelManager;
use local_models::cancellation::Canceller;

use sql::plugin::Builder; 
use std::fs; 
use events::scheduler; 

use std::fs::create_dir_all;
use tracing::info;

use std::sync::Mutex;

use tracing_subscriber::EnvFilter;

pub struct ManagerState(Mutex<Option<ModelManager>>);

fn main() {

    let log_file_path = get_logs_dir().expect("getting log directory");
    create_dir_all(&log_file_path).expect("creating log directory");
    let log_file = fs::File::create(log_file_path.join("app.log")).expect("creating log file");
    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    info!("starting...");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(Builder::default().build())
        .invoke_handler(tauri::generate_handler![local_models::get_architectures, local_models::get_models, local_models::get_prompt_templates, local_models::download_model, local_models::start, local_models::prompt, local_models::get_downloaded_models])
        // .plugin(local_models::init())
        .setup(|app| {

            let app_handle = app.handle();
              // Spawn a new asynchronous task for scheduler
              tauri::async_runtime::spawn(async move {
                scheduler(&app_handle).await;
            });

            Ok(())
        })
        .manage(ManagerState(Mutex::new(None)))
        .manage(Canceller::default())
        .run(tauri::generate_context!())    
        .expect("error while running tauri application");
}

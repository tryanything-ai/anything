// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod events;
mod file_manager;
mod local_models;
mod notifications;
mod sql;

use anything_core::build_runtime;
use anything_core::spawn_or_crash;
use anything_events::Context;
// use anything_core::spawn_or_crash;
use config::get_logs_dir;
use local_models::cancellation::Canceller;
use local_models::models::ModelManager;

use events::scheduler;
use sql::plugin::Builder;
use std::fs;
use tauri::App;
use tauri::AppHandle;

use std::fs::create_dir_all;
use tracing::info;

use std::sync::Mutex;

use anything_events::config as anything_events_config;

use tracing_subscriber::EnvFilter;

pub struct ManagerState(Mutex<Option<ModelManager>>);

// Run core server
async fn setup_anything_server(_app: AppHandle) -> anyhow::Result<()> {
    let config = anything_events_config::load(None)?;

    let context = Context::new(config.clone()).await?;
    let server = anything_events::Server::new(context).await?;
    info!("Setting up anything server");
    // let rt = build_runtime()?;
    // rt.spawn(async move {
    // });
    tokio::spawn(async move {
        info!("Spawning server");
        server.run_server().await
    });

    info!("started server");

    Ok(())
}
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
        .invoke_handler(tauri::generate_handler![
            local_models::get_architectures,
            local_models::get_models,
            local_models::get_prompt_templates,
            local_models::download_model,
            local_models::start,
            local_models::prompt,
            local_models::get_downloaded_models,
            file_manager::get_chat_flows,
        ])
        // .plugin(local_models::init())
        .setup(|app| {
            let app_handle = app.handle();
            // let window = app_handle.get_window("main").unwrap();
            // Spawn a new asynchronous task for scheduler
            tauri::async_runtime::spawn(async move {
                // spawn_or_crash("anything-server", app_handle.clone(), setup_anything_server);
                setup_anything_server(app_handle.clone()).await.unwrap();
                scheduler(&app_handle).await;
            });

            Ok(())
        })
        .manage(ManagerState(Mutex::new(None)))
        .manage(Canceller::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

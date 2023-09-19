// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod events;
mod file_manager;
mod local_models;
mod notifications;
mod sql;

use anything_core::{build_runtime, spawn_or_crash};
// use anything_core::spawn_or_crash;
use config::get_logs_dir;
use local_models::cancellation::Canceller;
use local_models::models::ModelManager;

use events::scheduler;
use sql::plugin::Builder;
use std::fs;

use std::fs::create_dir_all;
use std::path::PathBuf;
use tracing::info;

use std::sync::Mutex;

use anything_events::config as anything_events_config;

use tracing_subscriber::EnvFilter;

pub struct ManagerState(Mutex<Option<ModelManager>>);

// Run core server
async fn setup_anything_server(_nothing: ()) -> anyhow::Result<()> {
    println!("Setting up anything core");
    let config_file = &PathBuf::from("../config/events.toml");
    println!("Loading config from {:?}", config_file.exists());
    let config = anything_events_config::load(Some(config_file)).expect("error loading config");
    println!("Loaded config");
    let context = anything_events::bootstrap::bootstrap(&config).await?;

    // let context = Context::new(config.clone()).await?;
    let server = anything_events::Server::new(context).await?;
    println!("Setting up anything server");
    // let rt = build_runtime()?;
    // rt.spawn(async move {
    // });
    // tokio::spawn(async move {
    let _ = server.run_server().await;
    // });
    // tokio::spawn(async move { server.run_server() });

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

    let rt = build_runtime().expect("building runtime");
    rt.spawn(async move {
        println!("Spawning anything-server");
        spawn_or_crash("anything-server", (), setup_anything_server);
    });

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
                scheduler(&app_handle).await;
            });

            Ok(())
        })
        .manage(ManagerState(Mutex::new(None)))
        .manage(Canceller::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

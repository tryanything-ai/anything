// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core_messages;
use anything_core::{build_runtime, spawn_or_crash};
use anything_events::config as anything_events_config;
use std::path::PathBuf;
use tracing::info;

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
    // let rt = build_runtime().expect("building runtime");
    // rt.spawn(async move {
    //     println!("Spawning anything-server");
    //     spawn_or_crash("anything-server", (), setup_anything_server);
    // });

    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                println!("Spawning anything-server");
                spawn_or_crash("anything-server", (), setup_anything_server);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core_messages::get_flows,
            core_messages::get_chat_flows,
            core_messages::get_flow,
            core_messages::get_flow_by_name,
            core_messages::get_flow_node,
            core_messages::get_nodes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // .invoke_handler(tauri::generate_handler![
    //     local_models::get_architectures,
    //     local_models::get_models,
    //     local_models::get_prompt_templates,
    //     local_models::download_model,
    //     local_models::start,
    //     local_models::prompt,
    //     local_models::get_downloaded_models,
    //     file_manager::get_chat_flows,
    // ])
    // .plugin(local_models::init())
    // .setup(|app| {
    //     let app_handle = app.handle();
    //     // let window = app_handle.get_window("main").unwrap();
    //     // Spawn a new asynchronous task for scheduler
    //     tauri::async_runtime::spawn(async move {
    //         scheduler(&app_handle).await;
    //     });

    //     Ok(())
    // })
    // .manage(ManagerState(Mutex::new(None)))
    // .manage(Canceller::default())
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core_messages; 
use anything_core::{build_runtime, spawn_or_crash};
use std::path::PathBuf;
use tracing::info;
use anything_events::config as anything_events_config;

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

    let rt = build_runtime().expect("building runtime");
    rt.spawn(async move {
        println!("Spawning anything-server");
        spawn_or_crash("anything-server", (), setup_anything_server);
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .invoke_handler(tauri::generate_handler![
            core_messages::get_flows,
            core_messages::get_chat_flows,
            core_messages::get_flow,
            core_messages::get_flow_node,
            core_messages::get_nodes,
            core_messages::create_flow,
            core_messages::create_event
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

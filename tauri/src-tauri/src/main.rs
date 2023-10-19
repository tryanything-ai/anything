// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core_messages;
use anything_core::spawn_or_crash;
use anything_events::config as anything_events_config;
use std::env;
use std::path::PathBuf;
use tracing::info;
extern crate dotenv;

use tauri::Manager;

//https://github.com/FabianLars/tauri-plugin-deep-link/blob/main/example/main.rs
use tauri_plugin_deep_link; 

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
    match dotenv::dotenv() {
        Ok(_) => println!("Successfully loaded .env"),
        Err(error) => {
            println!("Warning: couldn't load .env - {:?}", error);
        }
    }
    
    //Deeplink 
    tauri_plugin_deep_link::prepare("xyz.anything.dev");

    //Sentry
    let dsn = env::var("SENTRY_DSN").unwrap_or_default();

    let client = sentry_tauri::sentry::init((
        dsn,
        sentry_tauri::sentry::ClientOptions {
            release: sentry_tauri::sentry::release_name!(),
            ..Default::default() // TODO: mark dev vs prod and alpha vs public etc
        },
    ));

    let _guard = sentry_tauri::minidump::init(&client);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(sentry_tauri::plugin())
        .setup(|app| {
            //DEEPLINK
            // If you need macOS support this must be called in .setup() !
            // Otherwise this could be called right after prepare() but then you don't have access to tauri APIs
            let handle = app.handle();
            tauri_plugin_deep_link::register(
              "anything",
              move |request| {
                dbg!(&request);
                println!("Got deep link request: {:?}", request);
                handle.emit_all("scheme-request-received", request).unwrap();
              },
            )
            .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);
              
            // If you also need the url when the primary instance was started by the custom scheme, you currently have to read it yourself
            /*
            #[cfg(not(target_os = "macos"))] // on macos the plugin handles this (macos doesn't use cli args for the url)
            if let Some(url) = std::env::args().nth(1) {
              app.emit_all("scheme-request-received", url).unwrap();
            }
            */
      
            Ok(())
          })
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                println!("Spawning anything-server");
                spawn_or_crash("anything-server", (), setup_anything_server);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core_messages::get_flows,
            // core_messages::get_flow_versions,
            core_messages::get_chat_flows,
            core_messages::get_flow,
            core_messages::get_flow_by_name,
            core_messages::get_flow_node,
            core_messages::get_nodes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

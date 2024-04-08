// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use anything_events::config as anything_events_config;
use std::env;
use anything_common::AnythingConfigBuilder;
use anything_runtime::RuntimeConfigBuilder;
use tauri::Manager;
extern crate dotenv;

//https://github.com/FabianLars/tauri-plugin-deep-link/blob/main/example/main.rs
use tauri_plugin_deep_link; 
use tauri_plugin_anything_tauri;

fn main() {
    println!("Running Main!");
    //Load .env
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

    // Initialize sentry
    let client = sentry_tauri::sentry::init((
        dsn,
        sentry_tauri::sentry::ClientOptions {
            release: sentry_tauri::sentry::release_name!(),
            ..Default::default() // TODO: mark dev vs prod and alpha vs public etc
        },
    ));

    let _guard = sentry_tauri::minidump::init(&client);

    //create local dir
    let base_dir = dirs::home_dir().unwrap().join(".anything");
    let runtime_config = RuntimeConfigBuilder::default()
    .base_dir(base_dir).build().unwrap();
    let anything_config = AnythingConfigBuilder::default().runtime_config(runtime_config).build().unwrap();
    tauri::Builder::default()
        // .plugin(tauri_plugin_fs_watch::init())
        .plugin(sentry_tauri::plugin())
        .plugin(tauri_plugin_anything_tauri::AnythingBuilder::default().config(anything_config).build())
        .setup(|app| {
            //DEEPLINK
            // If you need macOS support this must be called in .setup() !
            // Otherwise this could be called right after prepare() but then you don't have access to tauri APIs
            // let main_window = app.get_window("main").unwrap();
            let handle = app.handle();
            
            tauri_plugin_deep_link::register(
              "anything",
              move |request| {
                dbg!(&request);
               
                println!("Got deep link request: {:?}", request); 
                // main_window.emit("deeplink", request.clone()).unwrap();
                handle.emit_all("deeplink", request).unwrap();
              },
            )
            .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

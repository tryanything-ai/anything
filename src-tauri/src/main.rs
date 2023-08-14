// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sql;
mod events;

use sql::plugin::Builder; 
use events::scheduler; 

fn main() {
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(Builder::default().build())
        .setup(|app| {

            let app_handle = app.handle();
              // Spawn a new asynchronous task for scheduler
              tauri::async_runtime::spawn(async move {
                scheduler(&app_handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())    
        .expect("error while running tauri application");
}

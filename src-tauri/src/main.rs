// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

mod sql; 
mod events;

use sql::plugin::Builder; 
use events::task_to_run_every_minute;

// #[tokio::main]
fn main() {
    // tauri::async_runtime::set(tokio::runtime::Handle::current());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(Builder::default().build())
        // .setup(|app| {
        //     thread::spawn(task_to_run_every_minute);
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



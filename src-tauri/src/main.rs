// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

mod sql;
mod events;
mod events2; 

// use sql::plugin::Builder; 
use events2::scheduler; 

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .setup(|_app| {
            tokio::task::spawn(scheduler()); 
            Ok(())
        })
        .run(tauri::generate_context!())    
        .expect("error while running tauri application");
}



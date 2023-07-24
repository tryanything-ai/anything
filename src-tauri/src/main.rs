// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

fn task_to_run_every_minute() {
    loop {
        // Do your work here...
        println!("Hello, world from taks_to_run_every_minute!");
        // Sleep for a minute
        thread::sleep(Duration::from_secs(10));
    }
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|_app| {
            // This is a Tauri-specific setup function where you can set up
            // things that need to be done at the start of your application.

            // Start our task in the background
            thread::spawn(task_to_run_every_minute);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

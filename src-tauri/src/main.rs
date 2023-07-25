// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use std::time::{SystemTime, UNIX_EPOCH};
// fn do_work() {
//     //TODO: make it so we can also call this from teh front end. 
//     //or that it listens to emitted events so that we can call it from the front end
//     println!("Hello, world from do_work!");
// }
// Thoughts on events based architefture
//https://discord.com/channels/616186924390023171/731495028677148753/1133165388981620837
fn task_to_run_every_minute() {
    loop {
        // Do your work here...
        println!("Hello, world from taks_to_run_every_minute!");
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        println!("{:?}", since_the_epoch);
        //TODO: check if there is some work we are supposed to do
        //if (work){
        //    do_work();
        // }
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

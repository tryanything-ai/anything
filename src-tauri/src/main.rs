// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command as ProcessCommand;

mod db; 

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

        let output = ProcessCommand::new("who")
        .output()
        .expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout).to_string();

        //   let is_logged_in = output.lines().any(|line| line.contains(&username));

        println!("output: {:?}", output);

        println!("Hello, world from taks_to_run_every_minute!");
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        println!("{:?}", since_the_epoch);
        
        //TODO: mark an event from the SQL Db as done if there is one we need
        // Sleep for a minute
        thread::sleep(Duration::from_secs(10));
    }
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|_app| {
            thread::spawn(task_to_run_every_minute);
            Ok(())
        })
        .setup(|_app| {
            db::init();
            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



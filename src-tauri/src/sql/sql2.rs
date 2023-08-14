//Try to write the embedded sqlite db without making a plugin using sqlx

//create or connect to an existing database on load
//make it so we can write or read from the database in rust

//https://github.com/launchbadge/sqlx#usage
// use sqlx::sqlite::PgPoolOptions;
use std::{fs::create_dir_all, path::PathBuf};

use tauri::{
    AppHandle,  Runtime, 
};

pub const DB_STRING: &'static str = "sqlite:test.db";

pub async fn load_db<R: Runtime>(app: &AppHandle<R>) {

    
    //create path to db
    //implementj this later

}


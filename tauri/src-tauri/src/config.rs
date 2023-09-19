use anyhow::Result;
use std::fs::create_dir_all;
use std::path::PathBuf;
use tauri::api::path::document_dir;

//TODO: harmonize for one type of error handling. using anyhow
pub fn get_app_dir() -> Result<PathBuf> {
    Ok(document_dir()
        .ok_or(anyhow::anyhow!("Could not find document directory"))?
        .join("Anything"))
}

pub fn get_flows_dir() -> Result<PathBuf> {
    let dir = get_app_dir()?.join("flows");
    println!("flows dir: {:?}", dir);
    create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_models_dir() -> Result<PathBuf> {
    let dir = get_app_dir()?.join("models");
    println!("models dir: {:?}", dir);
    create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_logs_dir() -> Result<PathBuf> {
    let dir = get_app_dir()?.join("model_logs");
    create_dir_all(&dir)?;
    Ok(dir)
}
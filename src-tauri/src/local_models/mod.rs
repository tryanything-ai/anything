//Show, List, and Download Models
//Allow Rust to call and run a model with all parameters
//Make models available to any node that wants to use them
//There is no idea of a "selected model" because we hop back and forth alot

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };

pub mod config; 
pub mod prompt;
pub mod models; 
use prompt::Template; 
use models::{ Architecture, Model };

#[tauri::command]
fn get_prompt_templates() -> Vec<Template> {
    prompt::AVAILABLE_TEMPLATES.clone()
}

#[tauri::command]
fn get_architectures() -> Vec<Architecture> {
    models::AVAILABLE_ARCHITECTURES.clone()
}

#[tauri::command]
async fn get_models() -> Result<Vec<Model>, String> {
    models::get_available_models()
        .await
        .map_err(|err| err.to_string())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("local_models")
      .invoke_handler(tauri::generate_handler![get_architectures, get_models, get_prompt_templates])
      .build()
}
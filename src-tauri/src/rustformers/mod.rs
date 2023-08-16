extern crate llm;

pub mod config; 
pub mod prompt;

use prompt::Template; 

pub mod models;
use models::{Architecture, Model}; 

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };

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
    Builder::new("rustformers")
      .invoke_handler(tauri::generate_handler![get_prompt_templates, get_architectures, get_models])
      .build()
  }
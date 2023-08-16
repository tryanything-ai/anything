extern crate llm;

pub mod config; 
pub mod prompt;

use prompt::Template; 

pub mod models;
use models::Architecture; 

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
  
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("rustformers")
      .invoke_handler(tauri::generate_handler![get_prompt_templates, get_architectures])
      .build()
  }
extern crate llm;

pub mod prompt;
use prompt::Template; 

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };

#[tauri::command]
fn get_prompt_templates() -> Vec<Template> {
    prompt::AVAILABLE_TEMPLATES.clone()
}
  
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("rustformers")
      .invoke_handler(tauri::generate_handler![get_prompt_templates])
      .build()
  }
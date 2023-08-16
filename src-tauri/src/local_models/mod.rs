//Show, List, and Download Models
//Allow Rust to call and run a model with all parameters
//Make models available to any node that wants to use them
//There is no idea of a "selected model" because we hop back and forth alot

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
    Window,
  };

pub mod config; 
pub mod prompt;
pub mod models; 
pub mod events;

use events::Event; 
use prompt::Template; 
use models::{ Architecture, Model };
use bytesize::ByteSize;
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

#[tauri::command]
async fn download_model(  
    // window: Window,
    filename: &str
) -> Result<(), String> { 
    let path = models::get_local_model(filename, |downloaded, total, progress| {
        let message = format!(
            "Downloading model ({} / {})",
            ByteSize(downloaded),
            ByteSize(total)
        );
        println!("{}", message); 
        // Event::ModelLoading { message, progress }.send(&window);
    })
    .await
    .map_err(|err| err.to_string())?;
    Ok(())
}



pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("local_models")
      .invoke_handler(tauri::generate_handler![get_architectures, get_models, get_prompt_templates, download_model])
      .build()
}
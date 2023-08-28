use serde::Serialize;
use tauri::{AppHandle, Window, Runtime, Manager};
// use tauri::{
//     Window,
//     command,
//     plugin::{Builder as PluginBuilder, TauriPlugin},
//     api::path::document_dir,
//     AppHandle, Manager, RunEvent, Runtime, State,
// };
use tracing::error;

#[derive(Serialize, Debug)]
#[serde(tag = "untagged")]
pub enum Event {
    ModelLoading { message: String, progress: f32 },
    PromptResponse { message: String },
    EventProcessing { message: String, event_id: String, node_id: String, flow_id: String },
}

impl Event {
    pub fn name(&self) -> &str {
        match self {
            Event::ModelLoading { .. } => "model_loading",
            Event::PromptResponse { .. } => "prompt_response",
            Event::EventProcessing { .. } => "event_processing",
        }
    }

    pub fn send(&self, window: &Window) {
        if let Err(error) = window.emit(self.name(), self) {
            error!(
                error = error.to_string(),
                event = format!("{:?}", self),
                "sending event"
            );
        }
    }

}
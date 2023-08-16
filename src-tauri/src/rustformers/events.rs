use serde::Serialize;
use tauri::Window;
use tracing::error;

#[derive(Serialize, Debug)]
#[serde(tag = "untagged")]
pub enum Event {
    ModelLoading { message: String, progress: f32 },
    PromptResponse { message: String },
}

impl Event {
    pub fn name(&self) -> &str {
        match self {
            Event::ModelLoading { .. } => "model_loading",
            Event::PromptResponse { .. } => "prompt_response",
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
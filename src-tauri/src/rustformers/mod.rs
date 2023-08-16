#[cfg_attr(
    all(not(test), any(target_os = "windows", target_os = "linux")),
    feature(cublas)
)]
extern crate llm;


use tauri::{Manager, Window};

pub mod config; 
pub mod prompt;
pub mod events; 

use prompt::Template; 

pub mod models;
pub mod cancellation; 

use models::{get_local_model, Architecture, Model, ModelManager};
use cancellation::Canceller;
use bytesize::ByteSize;

use llm::{InferenceResponse, LoadProgress};

use std::sync::Mutex;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };

struct ManagerState(Mutex<Option<ModelManager>>);

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
async fn start(
    window: Window,
    state: tauri::State<'_, ManagerState>,
    canceller: tauri::State<'_, Canceller>,
    model_filename: String,
    architecture: String,
    tokenizer: String,
    context_size: usize,
    use_gpu: bool,
    prompt: Template,
    context_files: Vec<String>,
) -> Result<bool, String> {
    canceller.reset();
    let context = context_files
        .iter()
        .map(|path| context_file::read(PathBuf::from(path)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?
        .join("\n");

    let warmup_prompt = if !context.is_empty() {
        format!("{}\n{}", context, prompt.warmup)
    } else {
        prompt.warmup.clone()
    };

    let path = get_local_model(&model_filename, |downloaded, total, progress| {
        let message = format!(
            "Downloading model ({} / {})",
            ByteSize(downloaded),
            ByteSize(total)
        );
        Event::ModelLoading { message, progress }.send(&window);
    })
    .await
    .map_err(|err| err.to_string())?;
    let architecture = models::AVAILABLE_ARCHITECTURES
        .iter()
        .find(|v| *v.id == architecture)
        .ok_or("Architecture not found")?;
    let tokenizer = match tokenizer.as_str() {
        "embedded" => llm::TokenizerSource::Embedded,
        _ => return Err("Tokenizer not supported".to_string()),
    };

    info!(
        gpu = use_gpu,
        model = path.to_str().unwrap_or_default(),
        "starting model"
    );

    let params = llm::ModelParameters {
        use_gpu,
        context_size,
        ..Default::default()
    };
    let model = llm::load_dynamic(
        Some(architecture.inner),
        path.as_path(),
        tokenizer,
        params,
        |progress| match progress {
            LoadProgress::HyperparametersLoaded => Event::ModelLoading {
                message: "Hyper-parameters loaded".to_string(),
                progress: 0.05,
            }
            .send(&window),
            LoadProgress::ContextSize { .. } => Event::ModelLoading {
                message: "Context created".to_string(),
                progress: 0.1,
            }
            .send(&window),
            LoadProgress::LoraApplied { .. } => Event::ModelLoading {
                message: "LoRA applied".to_string(),
                progress: 0.15,
            }
            .send(&window),
            LoadProgress::TensorLoaded {
                current_tensor,
                tensor_count,
            } => {
                // Once we start loading tensors, we're at 20%, once we're finished, we're at 50%
                // and intermediate tensor loads should be linearly interpolated.
                let start = 0.2;
                let end = 0.5;
                let progress =
                    start + (end - start) * (current_tensor as f32 / tensor_count as f32);
                Event::ModelLoading {
                    message: format!("Loading tensor {}/{}", current_tensor, tensor_count),
                    progress,
                }
                .send(&window)
            }
            LoadProgress::Loaded { .. } => Event::ModelLoading {
                message: "Model loaded".to_string(),
                progress: 0.6,
            }
            .send(&window),
        },
    )
    .map_err(|e| format!("Error loading model: {}", e))?;

    let mut session = model.start_session(Default::default());

    // When you feed a prompt, progress is going to be determined by how far
    // through repeating the warmup prompt we are.
    let mut progress_length = 0;
    session
        .feed_prompt(
            model.as_ref(),
            warmup_prompt.as_str(),
            &mut Default::default(),
            llm::feed_prompt_callback(|res| match res {
                InferenceResponse::PromptToken(t) => {
                    progress_length += t.len();
                    let progress = progress_length as f32 / warmup_prompt.len() as f32;
                    Event::ModelLoading {
                        message: format!("Warming up model ({:.0}%)", progress * 100.0),
                        progress,
                    }
                    .send(&window);
                    canceller.inference_feedback()
                }
                _ => canceller.inference_feedback(),
            }),
        )
        .map_err(|e| format!("Error feeding prompt: {}", e))?;
    Event::ModelLoading {
        message: "Model loaded".to_string(),
        progress: 1.0,
    }
    .send(&window);

    if canceller.is_cancelled() {
        return Ok(false);
    }

    info!("finished warm-up prompt");
    *state.0.lock().unwrap() = Some(ModelManager {
        model,
        session,
        template: prompt,
    });

    Ok(true)
}

  
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("rustformers")
      .invoke_handler(tauri::generate_handler![get_prompt_templates, get_architectures, get_models])
      .build()
  }
//Show, List, and Download Models
//Allow Rust to call and run a model with all parameters
//Make models available to any node that wants to use them
//There is no idea of a "selected model" because we hop back and forth alot

#[cfg_attr(
    all(not(test), any(target_os = "windows", target_os = "linux")),
    feature(cublas)
)]
use tauri::Window;

extern crate llm;
use crate::ManagerState;

use llm::{InferenceResponse, LoadProgress};

use crate::config;

pub mod cancellation;
pub mod models;
pub mod prompt;
use cancellation::Canceller;

use crate::notifications::Event;

use bytesize::ByteSize;
use models::{get_local_model, Architecture, Model, ModelManager};
use prompt::Template;
use serde::Serialize;
use tracing::info;

#[tauri::command]
pub fn get_prompt_templates() -> Vec<Template> {
    prompt::AVAILABLE_TEMPLATES.clone()
}

#[tauri::command]
pub fn get_architectures() -> Vec<Architecture> {
    models::AVAILABLE_ARCHITECTURES.clone()
}

#[tauri::command]
pub async fn get_models() -> Result<Vec<Model>, String> {
    models::get_available_models()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_downloaded_models() -> Result<Vec<Model>, String> {
    models::get_downloaded_models()
        .await
        .map_err(|err| err.to_string())
}

//TODO: abstract this tauri command into code that can be called from rust event processing system

#[tauri::command]
pub async fn start(
    window: Window,
    state: tauri::State<'_, ManagerState>,
    canceller: tauri::State<'_, Canceller>,
    model_filename: String,
    architecture: String,
    tokenizer: String,
    context_size: usize,
    use_gpu: bool,
    prompt: Template,
    // context_files: Vec<String>,
) -> Result<bool, String> {
    canceller.reset();
    // let context = context_files
    //     .iter()
    //     .map(|path| context_file::read(PathBuf::from(path)))
    //     .collect::<Result<Vec<_>, _>>()
    //     .map_err(|err| err.to_string())?
    //     .join("\n");

    // let warmup_prompt = if !context.is_empty() {
    //     format!("{}\n{}", context, prompt.warmup)
    // } else {
    //     prompt.warmup.clone()
    // };

    //Not using context this way.

    let warmup_prompt = prompt.warmup.clone();

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

#[derive(Serialize)]
pub struct PromptResponse {
    pub stats: llm::InferenceStats,
    pub message: String,
}

#[tracing::instrument(skip(window, state, canceller, message))]
#[tauri::command]
pub async fn prompt(
    window: Window,
    state: tauri::State<'_, ManagerState>,
    canceller: tauri::State<'_, Canceller>,
    message: String,
) -> Result<PromptResponse, String> {
    info!("received prompt");

    let mut binding = state
        .0
        .lock()
        .map_err(|e| format!("Unable to lock the backend: {e}"))?;
    let manager: &mut ModelManager = (*binding).as_mut().ok_or("Model not started".to_string())?;
    let mut response = String::new();

    let stats = manager.infer(&message, |res| match res {
        InferenceResponse::InferredToken(tokens) => {
            response.push_str(&tokens);
            Event::PromptResponse { message: tokens }.send(&window);
            canceller.inference_feedback()
        }
        _ => canceller.inference_feedback(),
    })?;

    info!("finished prompt response");
    Event::PromptResponse {
        message: Default::default(),
    }
    .send(&window);

    Ok(PromptResponse {
        stats,
        message: response.replace(&message, "").trim().to_string(),
    })
}

// #[tauri::command]d
// async fn call_model(
//     window: Window,
//     prompt: Template,
//     model_filename: String,
//     context_size: usize,
//     use_gpu: bool,
//     architecture: String,
//     tokenizer: String,
// ) -> Result<(), String> {
//     //Get Path to local model and download if not found
//     let path = models::get_local_model(&model_filename, |downloaded, total, progress| {
//         let message = format!(
//             "Downloading model ({} / {})",
//             ByteSize(downloaded),
//             ByteSize(total)
//         );
//         println!("{}", message);
//         Event::ModelLoading { message, progress }.send(&window);
//     })
//     .await
//     .map_err(|err| err.to_string())?;

//     let architecture = models::AVAILABLE_ARCHITECTURES
//     .iter()
//     .find(|v| *v.id == architecture)
//     .ok_or("Architecture not found")?;
//     let tokenizer = match tokenizer.as_str() {
//         "embedded" => llm::TokenizerSource::Embedded,
//         _ => return Err("Tokenizer not supported".to_string()),
//     };

//     let params = llm::ModelParameters {
//         use_gpu,
//         context_size,
//         ..Default::default()
//     };

//     let model = llm::load_dynamic(
//         Some(architecture.inner),
//         path.as_path(),
//         tokenizer,
//         params,
//         // llm::load_progress_callback_stdout,
//         // TODO: loading states
//         |progress| match progress {
//             LoadProgress::HyperparametersLoaded => Event::ModelLoading {
//                 message: "Hyper-parameters loaded".to_string(),
//                 progress: 0.05,
//             }
//             .send(&window),
//             LoadProgress::ContextSize { .. } => Event::ModelLoading {
//                 message: "Context created".to_string(),
//                 progress: 0.1,
//             }
//             .send(&window),
//             LoadProgress::LoraApplied { .. } => Event::ModelLoading {
//                 message: "LoRA applied".to_string(),
//                 progress: 0.15,
//             }
//             .send(&window),
//             LoadProgress::TensorLoaded {
//                 current_tensor,
//                 tensor_count,
//             } => {
//                 // Once we start loading tensors, we're at 20%, once we're finished, we're at 50%
//                 // and intermediate tensor loads should be linearly interpolated.
//                 let start = 0.2;
//                 let end = 0.5;
//                 let progress =
//                     start + (end - start) * (current_tensor as f32 / tensor_count as f32);
//                 Event::ModelLoading {
//                     message: format!("Loading tensor {}/{}", current_tensor, tensor_count),
//                     progress,
//                 }
//                 .send(&window)
//             }
//             LoadProgress::Loaded { .. } => Event::ModelLoading {
//                 message: "Model loaded".to_string(),
//                 progress: 0.6,
//             }
//             .send(&window),
//         },
//     )
//     .map_err(|e| format!("Error loading model: {}", e))?;

//     let mut session = model.start_session(Default::default());

//      // When you feed a prompt, progress is going to be determined by how far
//     // through repeating the warmup prompt we are.
//     let mut progress_length = 0;
//     session
//         .feed_prompt(
//             model.as_ref(),
//             "", //TODO: don't do this
//             // warmup_prompt.as_str(), //TODO: match this to models
//             &mut Default::default(),
//             llm::feed_prompt_callback(|resp| match resp {
//                 llm::InferenceResponse::PromptToken(t)
//                 | llm::InferenceResponse::InferredToken(t) => {
//                     print_token(t);

//                     Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue)
//                 }
//                 _ => Ok(llm::InferenceFeedback::Continue),
//             }),
//         )
//         //     llm::feed_prompt_callback(|res| match res {
//         //         InferenceResponse::PromptToken(t) => {
//         //             progress_length += t.len();
//         //             let progress = progress_length as f32 / warmup_prompt.len() as f32;
//         //             Event::ModelLoading {
//         //                 message: format!("Warming up model ({:.0}%)", progress * 100.0),
//         //                 progress,
//         //             }
//         //             .send(&window);
//         //             canceller.inference_feedback()
//         //         }
//         //         _ => canceller.inference_feedback(),
//         //     }),
//         // )
//         .map_err(|e| format!("Error feeding prompt: {}", e))?;
//     Event::ModelLoading {
//         message: "Model loaded".to_string(),
//         progress: 1.0,
//     }
//     .send(&window);

//     Ok(())
// }

#[tauri::command]
pub async fn download_model(filename: &str) -> Result<(), String> {
    let _path = models::get_local_model(filename, |downloaded, total, _progress| {
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

// pub fn init<R: Runtime>() -> TauriPlugin<R> {
//     Builder::new("local_models")
//       .invoke_handler(tauri::generate_handler![get_architectures, get_models, get_prompt_templates, download_model, start, prompt])
//       .build()
// }

// fn print_token(t: String) {
//     print!("{t}");
//     std::io::stdout().flush().unwrap();
// }

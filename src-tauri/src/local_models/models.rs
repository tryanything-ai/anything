use crate::local_models::config::get_models_dir;
use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs; 
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::cmp::min;

use futures_util::StreamExt;
// use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};
// use std::cmp::min;
// use std::convert::Infallible;
// use std::fs;
// use std::fs::create_dir_all;
use std::io::Write;
// use std::path::PathBuf;
// use tracing::info;

lazy_static! {
    pub static ref AVAILABLE_MODELS: Vec<Model> =
        serde_json::from_str(include_str!("./data/models.json")).unwrap();
    pub static ref AVAILABLE_ARCHITECTURES: Vec<Architecture> = vec![
        Architecture {
            name: "LLaMA".to_string(),
            id: "llama".to_string(),
            inner: llm::ModelArchitecture::Llama,
        },
        Architecture {
            name: "GPT-2".to_string(),
            id: "gpt-2".to_string(),
            inner: llm::ModelArchitecture::Gpt2,
        },
        Architecture {
            name: "GPT-J".to_string(),
            id: "gpt-j".to_string(),
            inner: llm::ModelArchitecture::GptJ,
        },
        // Architecture { //TODO: something is unhappy here
        //     name: "GPT-NeoX".to_string(),
        //     id: "gpt-neo-x".to_string(),
        //     inner: llm::ModelArchitecture::GptNeoX,
        // },
        // Architecture {
        //     name: "MPT".to_string(),
        //     id: "mpt".to_string(),
        //     inner: llm::ModelArchitecture::Mpt,
        // },
        Architecture {
            name: "BLOOM".to_string(),
            id: "bloom".to_string(),
            inner: llm::ModelArchitecture::Bloom,
        },
    ];
}

pub async fn get_available_models() -> Result<Vec<Model>> {
    let dir = get_models_dir()?;
    let mut known_models = AVAILABLE_MODELS.clone();
    let mut models = fs::read_dir(dir)?
        .filter_map(|file| {
            if let Ok(file) = file {
                if let Some(filename) = file.file_name().to_str() {
                    if filename.ends_with(".bin")
                        && !known_models.iter().any(|m| m.filename.as_str() == filename)
                    {
                        return Some(Model {
                            name: filename.to_string(),
                            filename: filename.to_string(),
                            custom: true,
                            ..Default::default()
                        });
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();
    models.append(&mut known_models);
    models.sort_by(|a, b| b.custom.cmp(&a.custom));
    Ok(models)
}

pub async fn get_local_model<F>(filename: &str, progress: F) -> Result<PathBuf>
where
    F: Fn(u64, u64, f32),
{
    let models_dir = get_models_dir()?;
    if !models_dir.join(filename).exists() {
        let model = AVAILABLE_MODELS
            .iter()
            .find(|m| m.filename == filename)
            .ok_or(anyhow::anyhow!("Model not found"))?;
        download_file(&model.url, &models_dir, &model.filename, progress).await?;
        // info!(filename = model.filename, "finished downloading model");
    }
    Ok(models_dir.join(filename))
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    name: String,
    url: String,
    #[serde(default)]
    pub custom: bool,
    #[serde(default)]
    pub recommended: bool,
    pub filename: String,
    pub description: String,
    pub quantization: Option<Quantization>,
    pub parameter_count: Option<String>,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Quantization {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "8-bit")]
    Bit8,
    #[serde(rename = "6-bit")]
    Bit6,
    #[serde(rename = "5-bit")]
    Bit5,
    #[serde(rename = "4-bit")]
    Bit4,
    #[serde(rename = "2-bit")]
    Bit2,
}

// #[derive(Serialize, Clone)] didnt work cause modelArchitecture does not implement seriealize
#[derive(Serialize, Clone)]
pub struct Architecture {
    name: String,
    pub id: String,
    #[serde(skip_serializing)] ///carl added cause LLM does not implement serialize and made everything angry
    pub inner: llm::ModelArchitecture,
}

async fn download_file<F>(
    url: &str,
    destination: &PathBuf,
    filename: &str,
    progress: F,
) -> Result<PathBuf>
where
    F: Fn(u64, u64, f32),
{
    create_dir_all(destination)?;
    let destination = destination.join(filename);
    let destination = destination.to_str().unwrap();
    println!("downloading model to {}", destination);
    let response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();
    let mut file = std::fs::File::create(destination)?;
    let mut downloaded: u64 = 0;
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new_downloaded = min(downloaded + chunk.len() as u64, total_size);
        if total_size > 0 {
            let p = new_downloaded as f32 / total_size as f32;
            progress(downloaded, total_size, p);
        }
        downloaded = new_downloaded;
    }
    Ok(PathBuf::from(destination))
}
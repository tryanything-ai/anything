use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

use crate::rustformers::prompt::Template;
use crate::rustformers::config::get_models_dir;

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

/// Returns a list of all .bin files available in the models directory
/// (with associated metadata if we have them in our models.json file)
/// and if the model is a model that we don't know about, then we return
/// it first.
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

pub struct ModelManager {
    pub model: Box<dyn llm::Model>,
    pub session: llm::InferenceSession,
    pub template: Template,
}
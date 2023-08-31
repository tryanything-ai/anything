use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref AVAILABLE_TEMPLATES: Vec<Template> =
        serde_json::from_str(include_str!("./data/prompt-templates.json")).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    #[serde(default)]
    pub name: String,
    pub warmup: String,
    pub template: String,
}

impl Template {
    pub fn process(&self, prompt: &str) -> String {
        self.template
            .replace("{{prompt}}", prompt)
            .trim()
            .to_string()
    }
}
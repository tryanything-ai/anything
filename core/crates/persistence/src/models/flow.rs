use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFlow {
    pub name: String,
    pub active: Option<bool>,
    pub version: Option<String>,
}

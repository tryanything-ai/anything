use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    #[serde(flatten)]
    pub inner: HashMap<String, Value>,
}

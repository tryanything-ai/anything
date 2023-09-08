use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PackageData {
    pub id: Uuid,
    pub label: String,
    pub author: Option<String>,
    pub version: Option<String>,
}

impl Default for PackageData {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            label: String::default(),
            author: None,
            version: None,
        }
    }
}

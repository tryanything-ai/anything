use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Computable {
    #[allow(dead_code)]
    exec: String,
}

impl Into<String> for Computable {
    fn into(self) -> String {
        self.exec
    }
}

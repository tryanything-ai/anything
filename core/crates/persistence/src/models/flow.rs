use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct StoredFlow(anything_graph::Flow);

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateFlow {
    pub name: String,
    pub active: Option<bool>,
    pub version: Option<String>,
}

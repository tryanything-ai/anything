use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodePresentation {
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandleProps {
    pub id: String,
    pub r#type: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub source_handle: Option<String>,
    pub target: String,
    pub target_handle: Option<String>,
    pub r#type: String,
}

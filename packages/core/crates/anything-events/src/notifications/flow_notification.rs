#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FlowFileNotification {
    pub path: String,
    pub event: String,
}

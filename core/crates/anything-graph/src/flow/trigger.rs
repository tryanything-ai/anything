use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Trigger {
    Empty(EmptyTrigger),
    FileChange(FileChangeTrigger),
    Webhook(WebhookTrigger),
    Schedule(ScheduleTrigger),
    Manual(ManualTrigger),
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Empty(EmptyTrigger {
            settings: serde_json::json!({}),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct EmptyTrigger {
    pub settings: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookTrigger {
    pub settings: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ManualTrigger {
    pub name: String,
    pub settings: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ScheduleTrigger {
    pub settings: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FileChangeTrigger {
    pub settings: Value,
}

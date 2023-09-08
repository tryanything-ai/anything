use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Trigger {
    Empty(EmptyTrigger),
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmptyTrigger {
    pub settings: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebhookTrigger {
    pub settings: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ManualTrigger {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScheduleTrigger {
    pub settings: Value,
}

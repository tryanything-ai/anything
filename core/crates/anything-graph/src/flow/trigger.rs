use anything_core::{error::AnythingResult, parsing::parse_from_value_to_string};
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

impl Trigger {}

impl TriggerType for Trigger {
    fn is_match(&self, event_path: &str, payload: &Value) -> AnythingResult<bool> {
        match self {
            Trigger::Empty(trigger) => trigger.is_match(event_path, payload),
            Trigger::FileChange(trigger) => trigger.is_match(event_path, payload),
            Trigger::Webhook(trigger) => trigger.is_match(event_path, payload),
            Trigger::Schedule(trigger) => trigger.is_match(event_path, payload),
            Trigger::Manual(trigger) => trigger.is_match(event_path, payload),
        }
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Empty(EmptyTrigger {
            settings: serde_json::json!({}),
        })
    }
}

pub trait TriggerType {
    fn is_match(&self, root_url: &str, payload: &serde_json::Value) -> AnythingResult<bool>;
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EmptyTrigger {
    pub settings: Value,
}

impl TriggerType for EmptyTrigger {
    fn is_match(&self, event_path: &str, _payload: &Value) -> AnythingResult<bool> {
        Ok(event_path.starts_with("empty"))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WebhookTrigger {
    pub settings: Value,
}

impl TriggerType for WebhookTrigger {
    fn is_match(&self, event_path: &str, payload: &serde_json::Value) -> AnythingResult<bool> {
        let from_url = parse_from_value_to_string("from_url", &self.settings)?;
        let uri = url::Url::parse(&from_url)?;
        let from_url = format!("{}{}", uri.host_str().unwrap(), uri.path());

        let match_url = parse_from_value_to_string("match_url", &payload)?;
        let uri = url::Url::parse(&match_url)?;
        let match_url = format!("{}{}", uri.host_str().unwrap(), uri.path());

        Ok(event_path.starts_with("webhook") && from_url == match_url)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ManualTrigger {
    pub name: String,
    pub settings: Value,
}

impl TriggerType for ManualTrigger {
    fn is_match(&self, event_path: &str, payload: &serde_json::Value) -> AnythingResult<bool> {
        let from = parse_from_value_to_string("name", &self.settings)?;
        let to = parse_from_value_to_string("name", &payload)?;
        Ok(event_path.starts_with("manual") && from == to)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ScheduleTrigger {
    pub settings: Value,
}

impl TriggerType for ScheduleTrigger {
    fn is_match(&self, event_path: &str, _payload: &serde_json::Value) -> AnythingResult<bool> {
        // let from = parse_from_value_to_string("at", self.settings)?;
        // let to = parse_from_value_to_string("time", payload)?;
        Ok(event_path.starts_with("schedule"))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FileChangeTrigger {
    pub settings: Value,
}

impl TriggerType for FileChangeTrigger {
    fn is_match(&self, event_path: &str, payload: &serde_json::Value) -> AnythingResult<bool> {
        let from = parse_from_value_to_string("filepath", &self.settings)?;
        let to = parse_from_value_to_string("filepath", &payload)?;
        Ok(event_path.starts_with("file") && from == to)
    }
}

#[cfg(test)]
mod tests {}

#![allow(unused)]
use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Tags;

/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: i64,
    pub event_name: String,
    pub tags: Tags,
    pub payload: Value,
    pub metadata: Value,
    pub timestamp: DateTime<Utc>,
}

impl Event {
    /// Create a new Event with a `payload` and a set of `tags`
    ///
    /// # Example
    ///
    /// ```rust
    /// use anything_events::Event;
    /// use std::collections::HashMap;
    ///
    /// let mut evt = Event::new(
    ///     "Ginger".to_string(),
    ///     serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
    ///     vec!["corgi".to_string()]
    /// );
    /// ```
    pub fn new(event_name: String, payload: Value, tags: Vec<String>) -> Self {
        Self {
            id: i64::default(),
            event_name,
            payload,
            metadata: Value::Null,
            tags: Tags(tags),
            timestamp: DateTime::default(),
        }
    }

    pub fn tags(&self) -> Tags {
        self.tags.clone()
    }

    /// Modify an existing event with a new id
    ///
    /// # Example
    ///
    /// ```rust
    /// use anything_events::Event;
    /// use std::collections::HashMap;
    ///
    /// let mut evt = Event::new(
    ///     "Ginger".to_string(),
    ///     serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
    ///     vec!["corgi".to_string()]
    /// );
    /// let evt = evt.with_id(i64::default());
    /// ```
    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, event_name: String) -> Self {
        self.event_name = event_name;
        self
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_current_timestamp(mut self) -> Self {
        self.timestamp = Utc::now();
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        let mut tag_set: HashSet<String> = HashSet::new();

        let curr_tags = self.tags().into_iter();
        let new_tags = tags.into_iter();

        curr_tags.chain(new_tags).for_each(|tag| {
            tag_set.insert(tag.clone());
        });
        let mut tags = tag_set.into_iter().collect::<Vec<String>>();
        tags.sort();

        self.tags = Tags(tags);
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        match serde_json::to_value(metadata) {
            Ok(v) => {
                self.metadata = v;
            }
            Err(_e) => {}
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event_can_be_created() {
        Event::new(
            "wee".to_string(),
            serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
            vec![],
        );
    }

    #[test]
    fn test_cannot_have_duplicate_tags() {
        let evt = Event::new(
            "wee".to_string(),
            serde_json::json!(HashMap::from([("name".to_string(), "Ari".to_string())])),
            vec!["joe".to_string(), "bob".to_string(), "jerry".to_string()],
        );
        let evt = evt.with_tags(vec!["joe".to_string(), "ken".to_string()]);
        assert_eq!(
            evt.tags,
            Tags(vec![
                "bob".to_string(),
                "jerry".to_string(),
                "joe".to_string(),
                "ken".to_string(),
            ])
        )
    }
}

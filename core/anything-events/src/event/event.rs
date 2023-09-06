use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use tonic::Request;

use crate::pb::{AppendToStreamRequest, AppendToStreamResponse};

/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Event<D> {
    pub id: u64,
    pub event_name: String,
    pub payload: D,
    pub tags: Vec<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub timestamp: DateTime<Utc>,
}

type DBResult<T> = Result<T, Box<dyn std::error::Error>>;

impl<D> Event<D> {
    /// Create a new Event with a `payload` and a set of `tags`
    ///
    /// # Example
    ///
    /// ```rust
    /// use anything_events::Event;
    ///
    /// let event = Event::new("Ginger", vec!["corgi".to_string(), "dogs".to_string()]);
    /// ```
    pub fn new(event_name: String, payload: D, tags: Vec<String>) -> Self {
        Self {
            id: u64::default(),
            event_name,
            payload,
            metadata: None,
            tags,
            timestamp: DateTime::default(),
        }
    }

    /// Modify an existing event with a new id
    ///
    /// # Example
    ///
    /// ```rust
    /// use anything_events::Event;
    ///
    /// let mut evt = Event::new("Ginger", vec!["corgi".to_string()]);
    /// let evt = evt.with_id(u64::default());
    /// ```
    pub fn with_id(mut self, id: u64) -> Self {
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

        let curr_tags = self.tags.into_iter();
        let new_tags = tags.into_iter();

        curr_tags.chain(new_tags).for_each(|tag| {
            tag_set.insert(tag.clone());
        });
        let mut tags = tag_set.into_iter().collect::<Vec<String>>();
        tags.sort();

        self.tags = tags;
        self
    }

    pub async fn append_to_stream(
        pool: &AnyPool,
        request: Request<AppendToStreamRequest>,
    ) -> DBResult<AppendToStreamResponse> {
        let request = request.get_ref();

        let log = sqlx::query("INSERT INTO events").fetch_one(pool).await?;
        let response = AppendToStreamResponse {
            response: "ok".to_string(),
        };
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event_can_be_created() {
        Event::new("wee".to_string(), "payload", vec![]);
    }

    #[test]
    fn test_cannot_have_duplicate_tags() {
        let evt = Event::new(
            "wee".to_string(),
            "payload",
            vec!["joe".to_string(), "bob".to_string(), "jerry".to_string()],
        );
        let evt = evt.with_tags(vec!["joe".to_string(), "ken".to_string()]);
        assert_eq!(
            evt.tags,
            vec![
                "bob".to_string(),
                "jerry".to_string(),
                "joe".to_string(),
                "ken".to_string(),
            ]
        )
    }
}

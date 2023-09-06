use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Encode};
/// Event object that is stored in the database
///
/// # Keys
/// - `id` u64
/// - `event_name` String
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub event_name: String,
    pub tags: Tags,
    pub payload: Value,
    pub metadata: Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Tags(pub Vec<String>);

impl Tags {}

struct IterWrapper<'a> {
    // this needs to own the iterator, not a reference to it
    // in order to avoid returning a borrowed value
    inner: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for IterWrapper<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        // no semicolon here so the result is implicitly returned
        // your old error happened because the semicolon causes the value to not be returned
        self.inner.next()
    }
}

impl IntoIterator for Tags {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<DB: sqlx::Database> sqlx::Type<DB> for Tags
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for Tags
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_str(value).unwrap_or_default())
    }
}

impl<'q, DB: sqlx::Database> Encode<'q, DB> for Tags
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let val = serde_json::to_string(self).unwrap_or_default();
        <String as Encode<DB>>::encode(val, buf)
    }
}

impl Event {
    /// Create a new Event with a `payload` and a set of `tags`
    ///
    /// # Example
    ///
    /// ```rust
    /// use anything_events::Event;
    ///
    /// let event = Event::new("Ginger", vec!["corgi".to_string(), "dogs".to_string()]);
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
    ///
    /// let mut evt = Event::new("Ginger", vec!["corgi".to_string()]);
    /// let evt = evt.with_id(u64::default());
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

    // pub async fn append_to_stream(
    //     pool: &AnyPool,
    //     request: Request<AppendToStreamRequest>,
    // ) -> DBResult<AppendToStreamResponse> {
    //     let request = request.get_ref();

    //     let log = sqlx::query("INSERT INTO events").fetch_one(pool).await?;
    //     let response = AppendToStreamResponse {
    //         response: "ok".to_string(),
    //     };
    //     Ok(response)
    // }
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

use std::collections::HashMap;

use serde_json::Value;

pub struct Event {
    pub event_id: String,
    pub payload: Value,
}

#[derive(Debug)]
pub struct EventEnvelope<E>
where
    E: Event + Send + Sync,
{
    pub aggregate_id: String,
    pub sequence: usize,
    pub payload: E,
    pub metadata: HashMap<String, String>,
}

impl<A: Aggregate> Clone for EventEnvelope<A> {
    fn clone(&self) -> Self {
        Self {
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

use std::collections::HashMap;

use anything_core::error::AnythingResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{EventsError, EventsResult};

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkerHeartbeat {
    pub uuid: Uuid,
    pub version: String,
    pub last_seen_datetime: chrono::DateTime<chrono::Utc>,
}

// TODO: define a message for events here
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventNotification {
    pub uuid: Uuid,
    pub name: String,
    pub tags: Vec<String>,
    pub payload: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

impl EventNotification {
    pub fn from_zmq(msg: zmq::Message) -> EventsResult<Self> {
        match msg.as_str() {
            Some(msg) => {
                println!("Message: {:?}", msg);
                serde_json::from_str(msg).map_err(|e| EventsError::DecodingError(e))
            }
            None => Err(crate::errors::EventsError::EncodingError),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ControlMessage {
    pub uuid: Uuid,
    pub name: String,
    pub tags: Vec<String>,
}

impl From<ControlMessage> for zmq::Message {
    fn from(msg: ControlMessage) -> Self {
        let msg = serde_json::to_string(&msg).unwrap();
        zmq::Message::from(msg.as_bytes())
    }
}

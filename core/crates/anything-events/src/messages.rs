use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
}

impl From<zmq::Message> for EventNotification {
    fn from(msg: zmq::Message) -> Self {
        let msg = msg.as_str().unwrap();
        println!("Message: {:?}", msg);
        serde_json::from_str(msg).unwrap()
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

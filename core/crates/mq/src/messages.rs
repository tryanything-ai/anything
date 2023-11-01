use std::{
    fmt::{self, Display},
    io::{Read, Write},
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{error::MqError, MessageId, PAYLOAD_COMPRESSION_THRESHOLD};

pub trait PublishProtocol: Serialize + DeserializeOwned + Send + 'static + std::fmt::Debug {
    fn prefix() -> &'static str;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ClientProtocol {
    Sub(MessageSub),
    Unsub(MessageUnsub),
    Pub(MessagePub),
    Stop,
}

impl Display for ClientProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientProtocol::Pub(msg) => write!(f, "Pub({})", msg.topic),
            ClientProtocol::Sub(msg) => write!(f, "Sub({})", msg.topic),
            ClientProtocol::Unsub(msg) => write!(f, "Unsub({})", msg.topic),
            ClientProtocol::Stop => write!(f, "Stop"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum ServerProtocol {
    Pub(MessagePub),
    Ack(MessageAck),
}

impl Display for ServerProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerProtocol::Pub(msg) => write!(f, "Pub({})", msg.topic),
            ServerProtocol::Ack(msg) => write!(f, "Ack({})", msg.msg_id),
        }
    }
}

// ----------------------------------------------------------
// Messages
// ----------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Message<T> {
    pub id: MessageId,
    pub content: T,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MessageSub {
    pub topic: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MessageUnsub {
    pub topic: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MessagePub {
    pub topic: String,
    pub payload: Payload,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct MessageAck {
    pub msg_id: MessageId,
    pub err: Option<MqError>,
    pub num_recipients: Option<usize>,
}

// ----------------------------------------------------------
// Payload
// ----------------------------------------------------------

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Payload {
    data: Vec<u8>,
    compressed: bool,
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Payload {{ data: {:?}", self.data)?;
        if self.compressed {
            write!(f, " compressed")?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}

impl From<Vec<u8>> for Payload {
    fn from(data: Vec<u8>) -> Self {
        if data.len() > PAYLOAD_COMPRESSION_THRESHOLD {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&data[..]).unwrap();
            Payload {
                data: encoder.finish().unwrap(),
                compressed: true,
            }
        } else {
            Payload {
                data,
                compressed: false,
            }
        }
    }
}

impl From<Payload> for Vec<u8> {
    fn from(payload: Payload) -> Self {
        match payload.compressed {
            true => {
                let mut decoder = GzDecoder::new(&payload.data[..]);
                let mut data = Vec::new();
                decoder.read_to_end(&mut data).unwrap();
                data
            }
            false => payload.data,
        }
    }
}

impl From<serde_json::Value> for Payload {
    fn from(value: serde_json::Value) -> Self {
        let data = serde_json::to_vec(&value).unwrap();
        Payload::from(data)
    }
}

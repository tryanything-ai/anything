use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::{
    messages::{ControlMessage, EventNotification},
    server::server::Server,
};

pub async fn process_incoming_updates(_server: Arc<Server>) -> Result<()> {
    let (backend_tx, mut backend_rx) = mpsc::channel::<EventNotification>(32);

    let ctx = zmq::Context::new();
    let requester = ctx.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5557").is_ok());

    // let mut msg = zmq::Message::new();

    println!("Backend is sending a message");
    let ready_msg: zmq::Message = ControlMessage {
        uuid: uuid::Uuid::new_v4(),
        name: "ready".to_string(),
        tags: vec!["ready".to_string()],
    }
    .into();

    requester.send(ready_msg, 0).unwrap();

    loop {
        while let Some(msg) = backend_rx.recv().await {
            println!("Got a message back: {:?}", msg);
        }
    }
}

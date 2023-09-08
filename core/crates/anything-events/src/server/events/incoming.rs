use std::{sync::Arc, thread, time::Duration};

use anyhow::Result;
use tracing::{error, subscriber};

use crate::{
    messages::{ControlMessage, EventNotification},
    server::server::Server,
};

/// Process messages coming from the
pub async fn process_incoming_event_updates(server: Arc<Server>) -> Result<()> {
    let ctx = zmq::Context::new();
    // let requester = ctx.socket(zmq::REQ).unwrap();

    // let control_tx = server.post_office.post_mail::<ControlMessage>().await?;
    let frontend_tx = server.post_office.post_mail::<EventNotification>().await?;

    let event_receiver = ctx.socket(zmq::PULL).unwrap();
    assert!(event_receiver.bind("tcp://*:5557").is_ok());

    // event_receiver.recv_bytes(0).unwrap();
    // assert!(requester.bind("tcp://localhost:5557").is_ok());

    // let ready_msg = ControlMessage {
    //     uuid: uuid::Uuid::new_v4(),
    //     name: "ready".to_string(),
    //     tags: vec!["ready".to_string()],
    // };

    // let _ = control_tx.send(ready_msg);
    // requester.send(ready_msg, 0).unwrap();

    let mut msg = zmq::Message::new();
    loop {
        event_receiver.recv(&mut msg, 0).unwrap();
        // println!("Received: {:?}", msg.as_str().unwrap());
        let mut recved = zmq::Message::new();
        let notification = EventNotification::from(&msg.clone_into(recved));
        thread::sleep(Duration::from_millis(1000));
        // let resp = event_receiver.send("Thanks!", 0);
        // println!("resp: {:?}", resp);
    }
}

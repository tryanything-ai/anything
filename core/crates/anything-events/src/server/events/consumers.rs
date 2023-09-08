use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{messages::EventNotification, server::server::Server};

pub async fn process_consumers(server: Arc<Server>) -> anyhow::Result<()> {
    let (backend_tx, mut backend_rx) = mpsc::channel::<EventNotification>(32);
    let consumer_events_tx = server
        .comm_channels
        .lock()
        .unwrap()
        .consumer_events_tx
        .clone();

    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5557").is_ok());

    loop {
        let msg = responder.recv_msg(0);
        match msg {
            Ok(msg) => {
                println!("Frontend received a message. Sending it to the manager");
                consumer_events_tx.send(msg.into()).unwrap()
            }
            Err(_) => break,
        }
        // responder.send("World", 0).unwrap();
        // tx.send(msg).await.unwrap();
    }
    Ok(())
}

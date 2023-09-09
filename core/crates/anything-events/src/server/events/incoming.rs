use std::{sync::Arc, thread, time::Duration};

use anyhow::Result;
use crossbeam::channel::Sender;
use tracing::{error, info};
use zmq::Message;

use crate::{errors::EventsError, messages::EventNotification, server::server::Server};

/// Process messages coming from the
pub async fn process_incoming_event_updates(server: Arc<Server>) -> Result<()> {
    let ctx = zmq::Context::new();
    // let requester = ctx.socket(zmq::REQ).unwrap();

    // let control_tx = server.post_office.post_mail::<ControlMessage>().await?;
    let frontend_tx = server.post_office.post_mail::<EventNotification>().await?;

    let event_receiver = ctx.socket(zmq::PULL).unwrap();
    assert!(event_receiver.bind("tcp://*:5557").is_ok());

    loop {
        let msg = event_receiver.recv_msg(0);
        handle_notification(frontend_tx.clone(), msg).await;

        thread::sleep(Duration::from_millis(1000));
        // let resp = event_receiver.send("Thanks!", 0);
    }
}

async fn handle_notification(sender: Sender<EventNotification>, msg: Result<Message, zmq::Error>) {
    match msg {
        Ok(msg) => {
            info!("Received message: {:?}", msg.as_str());
            match EventNotification::from_zmq(msg) {
                Ok(notification) => {
                    let _ = sender.send(notification);
                }
                Err(e) => error!("Error occurred while sending notification: {:?}", e),
            }
        }
        Err(e) => {
            error!("Unable to handle message: {:?}", e);
        }
    }
}

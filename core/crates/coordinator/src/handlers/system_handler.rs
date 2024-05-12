use crate::{events::InternalEventsPublisher, manager::Manager};
use anything_mq::new_client;
use std::sync::Arc;

pub async fn process_system_events(manager: Arc<Manager>) -> anyhow::Result<()> {
    let client = new_client::<InternalEventsPublisher>().await.unwrap();
    let sub = client.subscribe("system-events").await.unwrap();

    while let Ok(msg) = sub.recv().await {
        println!("got a system message: {:?}", msg);
        match msg {
            InternalEventsPublisher::Ping => {}
            InternalEventsPublisher::Shutdown => {}
        }
    }

    Ok(())
}

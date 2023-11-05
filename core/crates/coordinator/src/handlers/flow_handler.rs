use std::sync::Arc;

use anything_mq::new_client;

use crate::{events::FlowPublisher, manager::Manager};

pub async fn process_flows(_manager: Arc<Manager>) -> anyhow::Result<()> {
    let client = new_client::<FlowPublisher>().await.unwrap();

    let sub = client.subscribe("**").await.unwrap();

    while let Ok(msg) = sub.recv().await {
        println!("Got a flow message: {:?}", msg);
    }
    Ok(())
}

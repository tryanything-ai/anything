use std::sync::Arc;

use crate::{models::event::Event, server::server::Server};

pub async fn process_on_events(server: Arc<Server>) -> anyhow::Result<()> {
    let events_rx = server.post_office.receive_mail::<Event>().await?;

    while let Ok(evt) = events_rx.recv() {
        // Do something with this new event
        println!("HERE: {:?}", evt);
    }

    Ok(())
}

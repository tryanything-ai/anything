use std::sync::Arc;

use crate::Server;

pub async fn process_engine_events(_server: Arc<Server>) -> anyhow::Result<()> {
    // let mut events_rx = server
    //     .post_office
    //     .receive_mail::<NodeExecutionUpdate>()
    //     .await?;
    Ok(())
}

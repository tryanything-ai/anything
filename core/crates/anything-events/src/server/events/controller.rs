use std::sync::Arc;

use crate::server::server::Server;

pub async fn handle_controller_plane(server: Arc<Server>) -> anyhow::Result<()> {
    // // let manager_rx = server.comm_channels.lock().unwrap().controller_rx.clone();

    // while let Ok(msg) = manager_rx.recv() {
    //     println!("Got a message back: {:?}", msg);
    // }
    Ok(())
}

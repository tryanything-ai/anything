use std::sync::Arc;

use postage::{
    mpsc::{self, Sender},
    sink::Sink,
    stream::Stream,
};
use tonic::transport::Channel;

use crate::{
    generated::events_client::EventsClient, internal::test_helper::get_test_context, Server,
};

async fn start_server(mut tx: Sender<Arc<Server>>) -> anyhow::Result<()> {
    let mut context = get_test_context().await;
    context.config.server.port = 10001;
    let server = Server::new(context).await?;
    let cloned_server = server.clone();
    let _ = tx.send(cloned_server.clone()).await;
    server.run_server().await.expect("should never return");
    Ok(())
}

pub async fn get_client() -> EventsClient<Channel> {
    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async { start_server(tx).await });

    let server = rx.recv().await.unwrap();
    loop {
        match EventsClient::connect(format!("http://localhost:{}", server.port)).await {
            Ok(client) => {
                return client;
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}

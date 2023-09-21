use std::{net::TcpListener, sync::Arc};

use postage::{
    mpsc::{self, Sender},
    sink::Sink,
    stream::Stream,
};
use tonic::transport::Channel;

use crate::{
    generated::events_client::EventsClient, internal::test_helper::get_test_context, Server,
};

static SERVER: tokio::sync::OnceCell<Arc<Server>> = tokio::sync::OnceCell::const_new();

fn get_unused_port() -> u16 {
    // Get unused port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

async fn start_server(mut tx: Sender<Arc<Server>>) -> anyhow::Result<()> {
    let mut context = get_test_context().await;

    context.config.server.port = get_unused_port();

    let server = Server::new(context).await?;
    let cloned_server = server.clone();
    let _ = tx.send(cloned_server.clone()).await;
    server.run_server().await.expect("should never return");
    Ok(())
}

pub async fn get_client() -> anyhow::Result<(EventsClient<Channel>, Arc<Server>)> {
    let (tx, mut rx) = mpsc::channel(1);
    SERVER
        .get_or_init(|| async {
            println!("Starting server in ONCECELL");
            tokio::spawn(async { start_server(tx).await });
            rx.recv().await.unwrap()
        })
        .await;

    let server = SERVER.get().unwrap();
    println!("Got server from ONCECELL: {:?}", server.port);

    loop {
        match EventsClient::connect(format!("http://[::]:{}", server.port)).await {
            Ok(client) => {
                return Ok((client, server.clone()));
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        }
    }
}

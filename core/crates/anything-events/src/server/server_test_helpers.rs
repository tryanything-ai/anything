use std::sync::Arc;

use postage::{
    mpsc::{self, Sender},
    sink::Sink,
    stream::Stream,
};
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::{
    generated::events_service_client::EventsServiceClient, internal::test_helper::get_test_context,
    system_handler::SystemHandler, utils::net::get_unused_port, Server,
};

static SERVER: tokio::sync::OnceCell<Arc<Server>> = tokio::sync::OnceCell::const_new();
static INIT: tokio::sync::OnceCell<(EventsServiceClient<Channel>, Arc<Server>)> =
    OnceCell::const_new();

async fn start_server(port: Arc<u16>, mut tx: Sender<Arc<Server>>) -> anyhow::Result<()> {
    let mut context = get_test_context().await;
    context.config.server.port = *port;
    SystemHandler::setup(context.clone()).await?;
    let server = Server::new(context).await?;
    let cloned_server = server.clone();
    let _ = tx.send(cloned_server.clone()).await;
    server.run_server().await.expect("should never return");
    Ok(())
}

pub async fn get_client() -> anyhow::Result<&'static (EventsServiceClient<Channel>, Arc<Server>)> {
    let resp = INIT
        .get_or_init(|| async {
            let unused_port = get_unused_port().await.unwrap();
            let unused_port_arc = Arc::new(unused_port);
            let (tx, mut rx) = mpsc::channel(1);
            SERVER
                .get_or_init(|| async {
                    tokio::spawn(async { start_server(unused_port_arc, tx).await });
                    let thing = rx.recv().await;
                    thing.unwrap()
                })
                .await;

            let server = SERVER.get().unwrap();
            let port = server.socket.port();
            let host = server.socket.ip().to_string();
            let addr = format!("http://[{}]:{}", host, port);

            let (client, server) = loop {
                match EventsServiceClient::connect(addr.clone()).await {
                    Ok(client) => {
                        break (client, server.clone());
                    }
                    Err(_e) => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    }
                }
            };

            (client, server)
        })
        .await;
    Ok(resp)
}

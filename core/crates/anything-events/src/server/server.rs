use std::net::SocketAddr;
use std::sync::Arc;

use anything_core::spawning::spawn_or_crash;
use postage::sink::Sink;
use tracing::debug;

use crate::callbacks::{self};
use crate::errors::{EventsError, EventsResult};
use crate::generated::events_service_server::EventsServiceServer;
use crate::generated::flows_service_server::FlowsServiceServer;
use crate::generated::triggers_service_server::TriggersServiceServer;
use crate::internal_notification::ShutdownNotification;
use crate::models::event::Event;
use crate::models::system_handler::SystemHandler;
use crate::server::events_server::EventManager;
use crate::server::flows_server::FlowManager;
use crate::server::triggers_server::TriggersManager;
use crate::{context::Context, post_office::PostOffice};
use crate::{workers, Flow, Trigger};

pub(crate) const EVENTS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("events_descriptor");

pub(crate) const FLOWS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("flows_descriptor");

pub(crate) const TRIGGER_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("triggers_descriptor");

pub struct Server {
    pub socket: SocketAddr,
    // pub port: u16,
    pub post_office: PostOffice,
    // pub store: Box<dyn StoreAdapter + Send + Sync>,
    pub context: Context,
    pub on_flow_handler_change: tokio::sync::watch::Sender<SystemHandler>,
}

impl Server {
    pub async fn new(context: Context) -> EventsResult<Arc<Self>> {
        let context_clone = context.clone();
        let (tx, _rx) =
            tokio::sync::watch::channel(SystemHandler::new(context_clone.config.clone()));

        let socket = get_configured_api_socket(&context)?;

        let server = Self {
            socket,
            post_office: PostOffice::open(),
            context,
            on_flow_handler_change: tx,
        };

        Ok(Arc::new(server))
    }

    pub async fn shutdown(&self) -> EventsResult<()> {
        let mut shutdown = self.post_office.post_mail::<ShutdownNotification>().await?;
        shutdown.send(ShutdownNotification {}).await?;

        Ok(())
    }

    pub async fn run_server(self: Arc<Self>) -> EventsResult<()> {
        spawn_or_crash(
            "on_event",
            self.clone(),
            callbacks::on_trigger::process_triggers,
        );
        spawn_or_crash(
            "system_watcher",
            self.clone(),
            workers::system_change::file_watcher,
        );
        spawn_or_crash(
            "handle_system_change",
            self.clone(),
            workers::system_change::handle_system_change,
        );

        // let addr = get_configured_api_socket(&self.context)?;
        debug!("Starting server...");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(EVENTS_FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(FLOWS_FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(TRIGGER_FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        debug!("Loading Event post mailbox");
        let sender = self.post_office.post_mail::<Event>().await.unwrap();

        debug!("Building event manager");
        let event_manager = EventManager::new(&self.context, sender.clone());
        let event_server = EventsServiceServer::new(event_manager);

        debug!("Loading Flow post mailbox");
        let sender = self.post_office.post_mail::<Flow>().await.unwrap();
        let flow_manager = FlowManager::new(&self.context, sender.clone());
        let flow_server = FlowsServiceServer::new(flow_manager);

        debug!("Loading Trigger post mailbox");
        let sender = self.post_office.post_mail::<Trigger>().await.unwrap();
        let triggers_manager = TriggersManager::new(&self.context, sender.clone());
        let triggers_server = TriggersServiceServer::new(triggers_manager);

        debug!("Getting listener");
        let (stream, local_addr) = self.get_configured_listening_stream().await?;

        debug!("Starting on local addr: {}", local_addr);
        tonic::transport::Server::builder()
            .add_service(triggers_server)
            .add_service(event_server)
            .add_service(flow_server)
            .add_service(reflection_service)
            .serve_with_incoming(stream)
            .await?;

        Ok(())
    }

    async fn get_configured_listening_stream(
        &self,
    ) -> EventsResult<(tokio_stream::wrappers::TcpListenerStream, SocketAddr)> {
        let socket = self.socket.clone();

        match tokio::net::TcpListener::bind(socket).await {
            Ok(listener) => {
                let local_addr = listener.local_addr().unwrap();
                Ok((
                    tokio_stream::wrappers::TcpListenerStream::new(listener),
                    local_addr,
                ))
            }
            Err(e) => {
                tracing::error!("Failed to bind to address: {:?}", e);
                return Err(EventsError::ConfigError(e.to_string()));
            }
        }
    }
}

fn get_configured_api_socket(context: &Context) -> EventsResult<SocketAddr> {
    let server_config = context.config.server.clone();

    let host = &server_config.host.unwrap_or("[::]".to_string());
    let port = &server_config.port;
    let url_str = &format!("{}:{}", host, port);

    debug!("Trying to parse {url_str}");
    let sock_url = &url_str.parse();
    match sock_url {
        Ok(v) => Ok(*v),
        Err(e) => {
            tracing::error!("Parsing address error: {:?}", e);
            return Err(EventsError::ConfigError(e.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {

    #![allow(unused)]
    use std::{any::Any, future, sync::Once};

    use anything_core::tracing::setup_tracing;
    use futures::Future;
    use postage::{
        mpsc::{self, Sender},
        sink::Sink,
        stream::Stream,
    };

    use tonic::{transport::Channel, Request};

    use crate::{internal::test_helper::get_test_context, server::server_test_helpers::get_client};

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_starts_up() -> anyhow::Result<()> {
        // TODO: do this in a better way
        let (_client, server) = get_client().await?;
        assert!(true); // The server started up!

        // Test a trigger update is sent when trigger is received
        // let mut trigger_tx = server.post_office.receive_mail::<Trigger>().await?;

        // let trigger = trigger_tx.recv().await;
        // assert!(trigger.is_some());

        server.shutdown().await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        Ok(())
    }
}

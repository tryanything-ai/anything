use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anything_core::spawning::spawn_or_crash;
use anything_graph::flow::flow::Flow;
use tracing::debug;

use crate::callbacks::{self};
use crate::errors::{EventsError, EventsResult};
use crate::events::events_server::EventsServer;
use crate::models::event::Event;
use crate::models::flow_handler::FlowHandler;
use crate::server::events_server::EventManager;
use crate::workers;
// use crate::utils::executor::spawn_or_crash;
use crate::{context::Context, post_office::PostOffice};

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("events_descriptor");

pub struct Server {
    pub port: u16,
    pub post_office: PostOffice,
    // pub store: Box<dyn StoreAdapter + Send + Sync>,
    pub context: Context,
    pub on_flow_handler_change: tokio::sync::watch::Sender<FlowHandler>,
}

impl Server {
    pub async fn new(context: Context) -> EventsResult<Arc<Self>> {
        let (tx, _rx) = tokio::sync::watch::channel(FlowHandler::new());
        let server = Self {
            port: context.config().server.port,
            post_office: PostOffice::open(),
            context,
            on_flow_handler_change: tx,
        };

        Ok(Arc::new(server))
    }

    pub async fn run_server(self: Arc<Self>) -> EventsResult<()> {
        spawn_or_crash(
            "on_event",
            self.clone(),
            callbacks::on_event::process_on_events,
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

        let addr = get_configured_api_socket(&self.context)?;
        debug!("Starting server...");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        debug!("Loading Event post mailbox");
        let sender = self.post_office.post_mail::<Event>().await.unwrap();

        debug!("Building event manager");
        let event_manager = EventManager::new(&self.context, sender);
        debug!("Building event server");
        let event_server = EventsServer::new(event_manager);

        debug!("Starting");
        tonic::transport::Server::builder()
            .add_service(event_server)
            .add_service(reflection_service)
            .serve(addr)
            .await?;

        Ok(())
    }
}

fn get_configured_api_socket(context: &Context) -> EventsResult<SocketAddr> {
    let server_config = context.config.server.clone();

    let host = &server_config.host.unwrap_or("0.0.0.0".to_string());
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
    use anyhow::Ok;

    use crate::internal::test_helper::get_test_context;

    use super::*;

    async fn start_server() -> anyhow::Result<Arc<Server>> {
        // let config = get_test_config();
        // let context = Context::new(config).await.unwrap();
        let mut context = get_test_context().await;
        context.config.server.port = 0;

        let server = Server::new(context).await.unwrap();
        let cloned_server = server.clone();
        tokio::spawn(async move {
            println!("Starting server...");
            cloned_server.run_server().await.unwrap();
        });
        Ok(server)
    }

    #[tokio::test]
    async fn test_starts_up() -> anyhow::Result<()> {
        // let server = start_server().await?;
        Ok(())
    }
}

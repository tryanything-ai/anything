use std::net::SocketAddr;
use std::sync::Arc;

use tracing::debug;

use crate::callbacks;
use crate::errors::{EventsError, EventsResult};
use crate::events::events_server::EventsServer;
use crate::server::events_server::EventManager;
use crate::utils::executor::spawn_or_crash;
// use crate::utils::executor::spawn_or_crash;
use crate::{context::Context, post_office::PostOffice};

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("events_descriptor");

pub struct Server {
    pub port: u16,
    pub post_office: PostOffice,
    // pub store: Box<dyn StoreAdapter + Send + Sync>,
    pub context: Context,
}

impl Server {
    pub async fn new(context: Context) -> EventsResult<Arc<Self>> {
        let server = Self {
            port: context.config().server.port,
            post_office: PostOffice::open(),
            context,
        };

        Ok(Arc::new(server))
    }

    pub async fn run_server(self: Arc<Self>) -> EventsResult<()> {
        spawn_or_crash(
            "on_event",
            self.clone(),
            callbacks::on_event::process_on_events,
        );

        let addr = get_configured_api_socket(&self.context)?;
        debug!("Starting server...");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        let sender = self.post_office.post_mail().await?;

        let event_manager = EventManager::new(&self.context, sender);
        let event_server = EventsServer::new(event_manager);

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

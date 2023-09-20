use std::net::SocketAddr;
use std::sync::Arc;

use anything_core::spawning::spawn_or_crash;
use tracing::debug;

use crate::callbacks::{self};
use crate::errors::{EventsError, EventsResult};
use crate::generated::events_server::EventsServer;
use crate::models::event::Event;
use crate::models::system_handler::SystemHandler;
use crate::server::events_server::EventManager;
use crate::workers;
// use crate::utils::executor::spawn_or_crash;
use crate::{context::Context, post_office::PostOffice};

pub(crate) const EVENTS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("events_descriptor");

pub(crate) const FLOWS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("flows_descriptor");

pub struct Server {
    pub port: u16,
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

        let port = match context.config().server.port {
            0 => {
                let socket = get_configured_api_socket(&context)?;
                let listener = std::net::TcpListener::bind(socket)?;
                listener.local_addr().unwrap().port()
            }
            p => p,
        };

        let server = Self {
            port,
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

        let addr = get_configured_api_socket(&self.context)?;
        debug!("Starting server...");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(EVENTS_FILE_DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(FLOWS_FILE_DESCRIPTOR_SET)
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

    #![allow(unused)]
    use tokio::sync::OnceCell;
    use tonic::transport::Channel;

    use crate::{
        generated::events::events_client::EventsClient, internal::test_helper::get_test_context,
    };
    static SERVER: OnceCell<Arc<Server>> = OnceCell::const_new();
    static CLIENT: OnceCell<EventsClient<Channel>> = OnceCell::const_new();

    async fn get_client() -> EventsClient<Channel> {
        SERVER
            .get_or_init(|| async {
                let mut context = get_test_context().await;
                context.config.server.port = 10001;
                let server = Server::new(context).await.unwrap();
                let cloned_server = server.clone();
                tokio::spawn(async move {
                    println!("Starting server...");
                    cloned_server.run_server().await.unwrap();
                })
                .await
                .unwrap();
                server
            })
            .await;

        let server = SERVER.get().unwrap();
        CLIENT
            .get_or_init(|| async {
                let addr_string = format!("http://[::1]:{}", server.port);
                println!("Connecting to {}", addr_string);
                let client = EventsClient::connect(addr_string).await.unwrap();
                client
            })
            .await
            .to_owned()
    }

    use super::*;

    #[tokio::test]
    async fn test_starts_up() -> anyhow::Result<()> {
        todo!("Finish test");
        // let _client = get_client().await;

        // let client = EventManager::models::event::{CreateEvent, Event},
        // Ok(())
    }
}

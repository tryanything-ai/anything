use std::net::SocketAddr;

use anything_core::error::AnythingError;
use axum::{
    routing::{get, post},
    Router,
};
use tracing::{debug, info};

use crate::{
    context::Context,
    errors::{EventsError, EventsResult},
};

mod heartbeat;

#[derive(Clone)]
pub struct AppState {
    context: Context,
}

async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn make_app(context: &Context) -> EventsResult<Router> {
    let state = AppState {
        context: context.clone(),
    };
    let api_routes = Router::new().route("/heartbeat", post(heartbeat::post_heartbeat));

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .nest("/api", api_routes)
        .with_state(state);

    Ok(app)
}

pub async fn serve(context: Context) -> EventsResult<()> {
    let app = make_app(&context).await?;

    let server_config = context.config.server.clone();

    let host = &server_config.host.unwrap_or("0.0.0.0".to_string());
    let port = &server_config.port;
    let url_str = &format!("{}:{}", host, port);

    debug!("Trying to parse {url_str}");
    let sock_url = &url_str.parse();
    let url: &SocketAddr = match &sock_url {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Parsing address error: {:?}", e);
            return Err(EventsError::ConfigError(e.to_string()));
        }
    };

    info!("Running server on {}", &url_str);

    axum::Server::bind(url)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

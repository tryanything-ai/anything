use axum::{
    routing::{get, post},
    Router,
};

use crate::{context::Context, errors::EventsResult};

mod heartbeat;

#[derive(Clone)]
pub struct AppState {
    context: Context,
}

async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn make_app(context: Context) -> EventsResult<Router> {
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
    let app = make_app(context).await?;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

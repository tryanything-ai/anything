use axum::{Router, routing::{get, post, delete, put}, middleware::{self, Next}, http::{HeaderValue, Method}};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;
// use extism::*;
// use tokio::sync::{Mutex, Semaphore};

mod api;
mod engine;
mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");

    let client = Arc::new(
        Postgrest::new(supabase_url.clone())
        .schema("anything")
        .insert_header("apikey", supabase_api_key.clone())
    );

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::OPTIONS])
        .allow_headers([hyper::header::AUTHORIZATION, hyper::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(api::root))
        .route("/workflows", get(api::get_workflows))
        .route("/workflow/:id", get(api::get_workflow))
        .route("/workflow/:id/versions", get(api::get_flow_versions))
        .route("/workflow/:id", post(api::create_workflow))
        .route("/workflow/:id", delete(api::delete_workflow))
        .route("/workflow/:id", put(api::update_workflow))
        .layer(middleware::from_fn(auth::middleware))
        .layer(cors)
        .with_state(client.clone());

    // let url = Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");
    // let manifest = Manifest::new([url]);
    // let plugin = Arc::new(Mutex::new(
    //     Plugin::new(&manifest, [], true).unwrap()
    // ));

    // Create a semaphore to limit the number of concurrent tasks
    // let semaphore = Arc::new(Semaphore::new(5));

    // // Spawn task processing loop
    // tokio::spawn(engine::task_processing_loop(client.clone(), plugin.clone(), semaphore.clone()));

    // // Spawn cron job loop
    // tokio::spawn(engine::cron_job_loop(client.clone()));

    // Run the API server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

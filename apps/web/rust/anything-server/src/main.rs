use axum::{Router, routing::{get, post, delete, put}, middleware::{self, Next}, http::{HeaderValue, Method}};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;
// use extism::*;
use tokio::sync::{Mutex, Semaphore, watch};

mod api;
mod task_engine;
mod trigger_engine;
mod auth;
mod secrets;
mod workflow_types;
mod execution_planner;
mod bundler;

#[macro_use] extern crate slugify;

pub struct AppState {
    client: Arc<Postgrest>,
    semaphore: Arc<Semaphore>,
    task_signal: watch::Sender<()>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");

    println!("supabase url {}", supabase_url); 
    println!("supabase api key {}", supabase_api_key);

    let client = Arc::new(
        Postgrest::new(supabase_url.clone())
        .schema("anything")
        // .insert_header("apikey", supabase_api_key.clone())
    );

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::OPTIONS])
        .allow_headers([hyper::header::AUTHORIZATION, hyper::header::CONTENT_TYPE]);

    let (task_signal, _) = watch::channel(());

    let state = Arc::new(AppState {
        client: client.clone(),
        semaphore: Arc::new(Semaphore::new(5)),
        task_signal,
    });

    let app = Router::new()
        .route("/", get(api::root))
        .route("/workflows", get(api::get_workflows))
        .route("/workflow/:id", get(api::get_workflow))
        .route("/workflow/:id/versions", get(api::get_flow_versions))
        .route("/workflow/:workflow_id/version/:workflow_version_id", put(api::update_workflow_version))
        .route("/workflow", post(api::create_workflow))
        .route("/workflow/:id", delete(api::delete_workflow))
        .route("/workflow/:id", put(api::update_workflow))
        .route("/actions", get(api::get_actions))
        // Secrets
        .route("/secrets", get(secrets::get_decrypted_secrets))
        .route("/secret", post(secrets::create_secret))
        .route("/secret", put(secrets::update_secret))
        .route("/secret/:id", delete(secrets::delete_secret))
        // Users Testing Workflows
        .route("/testing/workflow/:workflow_id/version/:workflow_version_id", get(api::test_workflow))
        .route("/testing/workflow/:workflow_id/version/:workflow_version_id/action/:action_id", get(api::test_action))
        .layer(middleware::from_fn(auth::middleware))
        .layer(cors)
        .with_state(state.clone()); 

    // let url = Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");
    // let manifest = Manifest::new([url]);
    // let plugin = Arc::new(Mutex::new(
    //     Plugin::new(&manifest, [], true).unwrap()
    // ));

    // Create a semaphore to limit the number of concurrent tasks
    // let semaphore = Arc::new(Semaphore::new(5));

    // Spawn task processing loop
    // Keeps making progress on work that is meant to be down now. 
    // tokio::spawn(task_engine::task_processing_loop(state.clone()));

    // // Spawn cron job loop
    // // Initiates work to be done on schedule tasks
    // tokio::spawn(trigger_engine::cron_job_loop(state.clone()));

    // Run the API server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{
    http::{
        header::ACCESS_CONTROL_ALLOW_ORIGIN, request::Parts as RequestParts, HeaderValue, Method,
    },
    middleware::{self},
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use postgrest::Postgrest;
use std::env;
use std::sync::Arc;
use tokio::sync::{watch, Semaphore};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
// use tower_http::set_header::response::SetResponseHeaderLayer;

mod api;
mod auth;
mod bundler;
mod execution_planner;
mod marketplace;
mod secrets;
mod supabase_auth_middleware;
mod task_engine;
mod task_types;
mod trigger_engine;
mod workflow_types;
use regex::Regex;

#[macro_use]
extern crate slugify;

pub struct AppState {
    anything_client: Arc<Postgrest>,
    marketplace_client: Arc<Postgrest>,
    semaphore: Arc<Semaphore>,
    task_engine_signal: watch::Sender<()>,
    trigger_engine_signal: watch::Sender<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");
    let cors_origin = env::var("ANYTHING_BASE_URL").expect("ANYTHING_BASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3001".to_string());

    //Anything Schema for Application
    let anything_client = Arc::new(
        Postgrest::new(supabase_url.clone())
            .schema("anything")
            .insert_header("apikey", supabase_api_key.clone()),
    );

    //Marketplace Schema for Managing Templates etc
    let marketplace_client = Arc::new(
        Postgrest::new(supabase_url.clone())
            .schema("marketplace")
            .insert_header("apikey", supabase_api_key.clone()),
    );

    let cors_origin = Arc::new(cors_origin);

    // Create a regex to match subdomains and localhost
    let protocol = if cors_origin.starts_with("https") {
        "https"
    } else {
        "http"
    };
    let cors_origin_regex = if cors_origin.contains("localhost") {
        Regex::new(&format!(r"^{}://localhost(:\d+)?$", protocol))
    } else {
        Regex::new(&format!(
            r"^{}://(?:[a-zA-Z0-9-]+\.)?{}$",
            protocol,
            regex::escape(&cors_origin)
        ))
    }
    .unwrap();

    let preflightlayer = SetResponseHeaderLayer::if_not_present(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _request_parts: &RequestParts| {
                let origin_str = origin.to_str().unwrap_or("");
                cors_origin_regex.is_match(origin_str)
            },
        ))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_headers([hyper::header::AUTHORIZATION, hyper::header::CONTENT_TYPE]);

    let (task_engine_signal, _) = watch::channel(());
    let (trigger_engine_signal, _) = watch::channel("".to_string());

    let state = Arc::new(AppState {
        anything_client: anything_client.clone(),
        marketplace_client: marketplace_client.clone(),
        semaphore: Arc::new(Semaphore::new(5)),
        task_engine_signal,
        trigger_engine_signal,
    });

    let app = Router::new()
        .route("/", get(api::root))
        .route("/workflows", get(api::get_workflows))
        .route("/workflow/:id", get(api::get_workflow))
        .route("/workflow/:id/versions", get(api::get_flow_versions))
        .route(
            "/workflow/:workflow_id/version/:workflow_version_id",
            put(api::update_workflow_version),
        )
        .route(
            "/workflow/:workflow_id/version/:workflow_version_id/publish",
            put(api::publish_workflow_version),
        )
        .route("/workflow", post(api::create_workflow))
        .route("/workflow/:id", delete(api::delete_workflow))
        .route("/workflow/:id", put(api::update_workflow))
        .route("/actions", get(api::get_actions))
        //Marketplace
        .route(
            "/marketplace/:workflow_id/publish",
            post(marketplace::publish_workflow_to_marketplace),
        )
        //Tasks
        .route("/tasks", get(api::get_tasks))
        .route("/tasks/:workflow_id", get(api::get_task_by_workflow_id))
        //Charts
        .route(
            "/charts/:workflow_id/tasks/:start_date/:end_date/:time_unit",
            get(api::get_task_status_counts_by_workflow_id),
        )
        // Secrets
        .route("/secrets", get(secrets::get_decrypted_secrets))
        .route("/secret", post(secrets::create_secret))
        .route("/secret", put(secrets::update_secret))
        .route("/secret/:id", delete(secrets::delete_secret))
        //Auth Providrs
        .route(
            "/auth/providers/:provider_name",
            get(api::get_auth_provider_by_name),
        )
        .route("/auth/accounts", get(api::get_auth_accounts))
        .route(
            "/auth/accounts/:provider_name",
            get(api::get_auth_accounts_for_provider_name),
        )
        .route("/auth/providers", get(api::get_auth_providers))
        .route(
            "/auth/:provider_name/callback",
            post(auth::handle_provider_callback),
        )
        // .route("/auth/initiate/:provider_name", post(auth::initiate_auth_flow))
        // Users Testing Workflows
        //Test Workflows
        .route(
            "/testing/workflow/:workflow_id/version/:workflow_version_id",
            get(api::test_workflow),
        )
        .route(
            "/testing/workflow/:workflow_id/version/:workflow_version_id/session/:session_id",
            get(api::get_test_session_results),
        )
        //Test Actions
        .route(
            "/testing/workflow/:workflow_id/version/:workflow_version_id/action/:action_id",
            get(api::test_action),
        )
        .layer(middleware::from_fn(supabase_auth_middleware::middleware))
        .layer(cors)
        .layer(preflightlayer)
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
    tokio::spawn(task_engine::task_processing_loop(state.clone()));

    // // Spawn cron job loop
    // // Initiates work to be done on schedule tasks
    tokio::spawn(trigger_engine::cron_job_loop(state.clone()));

    // Run the API server
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

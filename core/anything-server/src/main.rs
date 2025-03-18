use axum::{
    http::{
        header::ACCESS_CONTROL_ALLOW_ORIGIN, request::Parts as RequestParts, HeaderValue, Method,
    }, middleware::{self},
    response::{Html, IntoResponse},
      routing::{any, delete, get, post, put}, Router
};
 
use bundler::{accounts::accounts_cache::AccountsCache, secrets::secrets_cache::SecretsCache};
use dotenv::dotenv;
use processor::processor::ProcessorMessage;
use postgrest::Postgrest;
use reqwest::Client;
use status_updater::StatusUpdateMessage;
use serde_json::Value;
use std::{collections::HashMap, time::Duration, time::Instant};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{watch, Semaphore};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tokio::sync::mpsc; 
use aws_sdk_s3::Client as S3Client;
use files::r2_client::get_r2_client;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;

use regex::Regex;

#[macro_use]
extern crate slugify;

use auth::init::AuthState;

mod system_plugins; 
mod system_workflows;
mod processor;
mod system_variables;
mod workflows; 
mod actions; 
mod tasks; 
mod auth;
mod vault;
mod billing;
mod email;
mod bundler;
mod status_updater;
mod files;
mod variables; 
mod charts;
mod marketplace;
mod secrets;
mod supabase_jwt_middleware;
mod api_key_middleware;
mod account_auth_middleware;
mod types;
mod templater;
mod testing; 
mod trigger_engine;
mod agents; 

use tokio::sync::oneshot;
use tokio::sync::Mutex;
use std::sync::atomic::AtomicBool;
use sys_info;

// Add this struct to store completion channels
pub struct FlowCompletion {
    pub sender: oneshot::Sender<Value>,
    pub needs_response: bool,
}

pub struct CachedApiKey {
    pub account_id: String,
    pub secret_id: uuid::Uuid,
    pub secret_name: String,
}

pub struct AppState {
    anything_client: Arc<Postgrest>,
    marketplace_client: Arc<Postgrest>,
    public_client: Arc<Postgrest>,
    r2_client: Arc<S3Client>,
    http_client: Arc<Client>,
    workflow_processor_semaphore: Arc<Semaphore>,
    auth_states: RwLock<HashMap<String, AuthState>>,
    trigger_engine_signal: watch::Sender<String>,
    processor_sender: mpsc::Sender<ProcessorMessage>,
    processor_receiver: Mutex<mpsc::Receiver<ProcessorMessage>>, 
    flow_completions: Arc<Mutex<HashMap<String, FlowCompletion>>>,
    api_key_cache: Arc<RwLock<HashMap<String, CachedApiKey>>>,
    account_access_cache: Arc<RwLock<account_auth_middleware::AccountAccessCache>>,
    bundler_secrets_cache: RwLock<SecretsCache>,
    bundler_accounts_cache: RwLock<AccountsCache>,
    flow_session_cache: Arc<RwLock<processor::flow_session_cache::FlowSessionCache>>,
    shutdown_signal: Arc<AtomicBool>,
    task_updater_sender: mpsc::Sender<StatusUpdateMessage>,
}

#[tokio::main(
    flavor = "multi_thread",
    worker_threads = 48
)]
// #[tokio::main]
async fn main() {
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");
    let cors_origin = env::var("ANYTHING_BASE_URL").expect("ANYTHING_BASE_URL must be set");
    let bind_address = "0.0.0.0:3001".to_string();

    //Anything Schema for Application
    let anything_client = Arc::new(
        Postgrest::new(supabase_url.clone())
            .schema("anything")
            .insert_header("apikey", supabase_api_key.clone()),
    );

    let r2_client = Arc::new(get_r2_client().await);    

    //Marketplace Schema for Managing Templates etc
    let marketplace_client = Arc::new(
        Postgrest::new(supabase_url.clone())
            .schema("marketplace")
            .insert_header("apikey", supabase_api_key.clone()),
    );
    
    //Marketplace Schema for Managing Templates etc
    let public_client = Arc::new(
        Postgrest::new(supabase_url.clone())
            .schema("public")
            .insert_header("apikey", supabase_api_key.clone()),
    );

    let cors_origin = Arc::new(cors_origin);
    println!("[CORS] CORS origin: {:?}", cors_origin);

    // Create a regex to match subdomains and localhost
    let protocol = if cors_origin.starts_with("https") {
        "https"
    } else {
        "http"
    };
    println!("[CORS] Protocol: {}", protocol);

    let cors_origin_regex = if cors_origin.contains("localhost") {
        let regex = Regex::new(&format!(r"^{}://localhost(:\d+)?$", protocol)).unwrap();
        println!("[CORS] Localhost regex: {:?}", regex);
        regex
    } else {
        let regex = Regex::new(&format!(
            r"^{}://(?:[a-zA-Z0-9-]+\.)?{}$",
            protocol,
            regex::escape(&cors_origin) //TODO: maybe this is breaking? app acts normal but the logs say is not woring
        ))
        .unwrap();
        println!("[CORS] Domain regex: {:?}", regex);
        regex
    };

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _request_parts: &RequestParts| {
                let origin_str = origin.to_str().unwrap_or("");
                let is_match = cors_origin_regex.is_match(origin_str);
                println!(
                    "[CORS] Checking origin: {} - Match: {}",
                    origin_str, is_match
                );
                is_match
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

    println!("[CORS] CORS layer configured");

    let preflightlayer = SetResponseHeaderLayer::if_not_present(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );

    let (trigger_engine_signal, _) = watch::channel("".to_string());
    let (processor_tx, processor_rx) = mpsc::channel::<ProcessorMessage>(1000); // Create both sender and receiver

    // Create the task updater channel  
   let (task_updater_tx, task_updater_rx) = mpsc::channel::<StatusUpdateMessage>(1000); // Buffer size of 100

    let state = Arc::new(AppState {
        anything_client: anything_client.clone(),
        marketplace_client: marketplace_client.clone(),
        public_client: public_client.clone(),
        r2_client: r2_client.clone(),
        http_client: Arc::new(Client::new()),
        auth_states: RwLock::new(HashMap::new()),
        workflow_processor_semaphore: Arc::new(Semaphore::new(100)), //How many workflows we can run at once
        trigger_engine_signal,
        processor_sender: processor_tx,
        processor_receiver: Mutex::new(processor_rx),
        flow_completions: Arc::new(Mutex::new(HashMap::new())),
        api_key_cache: Arc::new(RwLock::new(HashMap::new())),
        account_access_cache: Arc::new(RwLock::new(
            account_auth_middleware::AccountAccessCache::new(Duration::from_secs(86400))
        )),
        bundler_secrets_cache: RwLock::new(SecretsCache::new(Duration::from_secs(86400))), // 1 day TTL
        bundler_accounts_cache: RwLock::new(AccountsCache::new(Duration::from_secs(86400))), // 1 day TTL
        flow_session_cache: Arc::new(RwLock::new(processor::flow_session_cache::FlowSessionCache::new(Duration::from_secs(3600)))),
        shutdown_signal: Arc::new(AtomicBool::new(false)),
        task_updater_sender: task_updater_tx.clone(), // Store the sender in AppState
    });

pub async fn root() -> impl IntoResponse {
    Html(r#"Check out <a href="https://www.tryanything.xyz">tryanything.xyz</a> to start"#)
}

    // Define routes that are public
    let public_routes = Router::new()
    .route("/", get(root))
    .route(
        "/auth/:provider_name/callback",
        get(auth::init::handle_provider_callback),
    )
    .route(
        "/billing/webhooks/new_account_webhook",
        post(billing::accounts::handle_new_account_webhook),
    )
    .route("/webhooks/create_user_in_external_email_system", post(email::handle_new_account_webhook))
    .route("/billing/webhooks/stripe", post(billing::stripe_webhooks::handle_webhook))
    .route("/auth/providers/:provider_name/client_id/set",
        post(auth::providers::set_auth_provider_client_id),
    )
    .route("/auth/providers/:provider_name/client_id/update",
    post(auth::providers::update_auth_provider_client_id),
    )
        .route("/auth/providers/:provider_name/client_secret_id/set",
        post(auth::providers::set_auth_provider_client_secret_id),
    )
    //marketplace
    .route("/marketplace/actions", get(marketplace::actions::get_actions_from_marketplace))
    .route("/marketplace/workflows", get(marketplace::workflows::get_marketplace_workflows))
    .route("/marketplace/workflow/:slug", get(marketplace::workflows::get_marketplace_workflow_by_slug))
    .route("/marketplace/profiles", get(marketplace::profiles::get_profiles_from_marketplace))
    .route("/marketplace/profile/:username", get(marketplace::profiles::get_marketplace_profile_by_username))

    // API Routes for running workflows - some protection done at api.rs vs route level
    .route("/api/v1/workflow/:workflow_id/start", any(system_plugins::webhook_trigger::run_workflow))
    .route("/api/v1/workflow/:workflow_id/start/respond", any(system_plugins::webhook_trigger::run_workflow_and_respond))
    .route("/api/v1/workflow/:workflow_id/version/:workflow_version_id/start", any(system_plugins::webhook_trigger::run_workflow_version))
    .route("/api/v1/workflow/:workflow_id/version/:workflow_version_id/start/respond", any(system_plugins::webhook_trigger::run_workflow_version_and_respond))

    // API routes for running agent tools - very simliar to webhooks just shapped differnt to capture relationshipe between agent and workflow
    .route("/api/v1/agent/:agent_id/tool/:tool_id/start/respond", post(system_plugins::agent_tool_trigger::run_workflow_as_tool_call_and_respond));

    let protected_routes = Router::new()
        .route("/account/:account_id/workflows", get(workflows::get_workflows))
        .route("/account/:account_id/workflow/:id", get(workflows::get_workflow))
        .route("/account/:account_id/workflow/:id/versions", get(workflows::get_flow_versions))
        .route(
            "/account/:account_id/workflow/:workflow_id/version/:workflow_version_id",
            get(workflows::get_flow_version),
        )
        .route(
            "/account/:account_id/workflow/:workflow_id/version/:workflow_version_id",
            put(workflows::update_workflow_version),
        )
        .route(
            "/account/:account_id/workflow/:workflow_id/version/:workflow_version_id/publish",
            put(workflows::publish_workflow_version),
        )
        .route("/account/:account_id/workflow", post(workflows::create_workflow))
        .route("/account/:account_id/workflow/json", post(workflows::create_workflow_from_json))
        .route("/account/:account_id/workflow/:id", delete(workflows::delete_workflow))
        .route("/account/:account_id/workflow/:id", put(workflows::update_workflow))
        .route("/account/:account_id/actions", get(actions::get_actions))
        .route("/account/:account_id/triggers", get(actions::get_triggers))
        .route("/account/:account_id/other", get(actions::get_other_actions))
        .route("/account/:account_id/responses", get(actions::get_responses))

        //Marketplace && Templates
        .route(
            "/account/:account_id/marketplace/workflow/:workflow_id/version/:workflow_version_id/publish",
            post(marketplace::workflows::publish_workflow_to_marketplace), 
        )
        .route("/account/:account_id/marketplace/action/publish", post(marketplace::actions::publish_action_template))
        .route("/account/:account_id/marketplace/workflow/:template_id/clone", get(marketplace::workflows::clone_marketplace_workflow_template))

        //Account Management
        .route("/account/:account_id/slug/:slug", get(auth::accounts::get_account_by_slug))

        //Billing
        .route("/account/:account_id/billing/status", get(billing::usage::get_account_billing_status))
        .route("/account/:account_id/billing/checkout", post(billing::create_links::get_checkout_link))
        .route("/account/:account_id/billing/portal", post(billing::create_links::get_billing_portal_link))
        
        //Tasks
        .route("/account/:account_id/tasks", get(tasks::get_tasks))
        .route("/account/:account_id/tasks/:workflow_id", get(tasks::get_task_by_workflow_id))

        //Charts
        .route(
            "/account/:account_id/charts/:workflow_id/tasks/:start_date/:end_date/:time_unit/:timezone",
            get(charts::get_workflow_tasks_chart),
        )
        .route("/account/:account_id/charts/tasks/:start_date/:end_date/:time_unit/:timezone", get(charts::get_account_tasks_chart))

        // Secrets
        .route("/account/:account_id/secrets", get(secrets::get_decrypted_secrets))
        .route("/account/:account_id/secret", post(secrets::create_secret))
        .route("/account/:account_id/secret/:id", delete(secrets::delete_secret))
        
        // User Facing API
        .route("/account/:account_id/keys", get(secrets::get_decrypted_anything_api_keys)) //read
        .route("/account/:account_id/key", post(secrets::create_anything_api_key)) //create
        .route("/account/:account_id/key/:id", delete(secrets::delete_api_key)) //delete from db, vault, and cache
      
        //Auth Providrs
        .route(
            "/account/:account_id/auth/providers/:provider_name",
            get(auth::providers::get_auth_provider_by_name),
        )
        .route("/account/:account_id/auth/accounts", get(auth::accounts::get_auth_accounts))
        .route(
            "/account/:account_id/auth/accounts/:provider_name",
            get(auth::accounts::get_auth_accounts_for_provider_name),
        )
        .route("/account/:account_id/auth/providers", get(auth::providers::get_auth_providers)) //No reason to really havea account_id here but maybe in future we have account specific auth providers so leaving it
        .route(
            "/account/:account_id/auth/:provider_name/initiate",
            get(auth::init::generate_oauth_init_url_for_client),
        )
        //Test Workflows
        .route(
            "/account/:account_id/testing/workflow/:workflow_id/version/:workflow_version_id",
            post(testing::test_workflow),
        )
        .route(
            "/account/:account_id/testing/workflow/:workflow_id/version/:workflow_version_id/session/:session_id",
            get(testing::get_test_session_results),
        )
        //Variables Explorer for Testing
        //TODO: we need to protect this for parallel running. You should not be able to select a result that isnt guranteed to be there
        .route(
            "/account/:account_id/testing/workflow/:workflow_id/version/:workflow_version_id/action/:action_id/results",
            get(variables::get_flow_version_results)
        )
        .route( "/account/:account_id/testing/workflow/:workflow_id/version/:workflow_version_id/action/:action_id/variables",
        get(variables::get_flow_version_inputs))
        .route(
            "/account/:account_id/testing/system_variables",
            get(system_variables::get_system_variables_handler))
        
        //Test Actions
        // .route(
        //     "/account/:account_id/testing/workflow/:workflow_id/version/:workflow_version_id/action/:action_id",
        //     get(testing::test_action),
        // )

        //Agents
        .route("/account/:account_id/agent", post(agents::create::create_agent))
        .route("/account/:account_id/agents", get(agents::get::get_agents))
        .route("/account/:account_id/agent/:agent_id", get(agents::get::get_agent))
        .route("/account/:account_id/agent/:agent_id", put(agents::update::update_agent))
        .route("/account/:account_id/agent/:agent_id", delete(agents::delete::delete_agent))

        //Agent Tools
        .route("/account/:account_id/agent/:agent_id/tool", post(agents::tools::add_tool))
        .route("/account/:account_id/agent/:agent_id/tool/:tool_id", delete(agents::tools::remove_tool))
        .route("/account/:account_id/agent/:agent_id/tools", get(agents::tools::get_agent_tools))
        
        //Fetch Workflows that are tools
        .route("/account/:account_id/tools", get(workflows::get_agent_tool_workflows))

        //Phone Numbers
        .route("/account/:account_id/phone_numbers/:country/:area_code", get(agents::twilio::search_available_phone_numbers_on_twilio))
        .route("/account/:account_id/phone_numbers", get(agents::twilio::get_account_phone_numbers))
        .route("/account/:account_id/phone_number", post(agents::twilio::purchase_phone_number))

        //Agent Communication Channels
        .route("/account/:account_id/agent/:agent_id/phone_number", post(agents::channels::connect_phone_number_to_agent))
        .route("/account/:account_id/agent/:agent_id/phone_number/:phone_number_id", delete(agents::channels::remove_phone_number_from_agent))

        //Calls
        .route("/account/:account_id/calls", get(agents::vapi::get_vapi_calls))

        // Invitations
        .route("/account/:account_id/invitations", get(auth::accounts::get_account_invitations))

        // Members
        .route("/account/:account_id/members", get(auth::accounts::get_account_members))

        // File Management
        .route("/account/:account_id/files", get(files::routes::get_files))
        .route("/account/:account_id/file/upload/:access", post(files::routes::upload_file))
        .route("/account/:account_id/file/:file_id", delete(files::routes::delete_file))
        .route("/account/:account_id/file/:file_id/download", get(files::routes::get_file_download_url))

        .layer(middleware::from_fn_with_state(
            state.clone(),
            account_auth_middleware::account_access_middleware,
        ))
        .layer(middleware::from_fn(supabase_jwt_middleware::middleware));
   

    let app = Router::new()
        .merge(public_routes) // Public routes
        .merge(protected_routes) // Protected routes
        .layer(cors)
        .layer(preflightlayer)
        .with_state(state.clone()); 
    
    // Spawn processor
    tokio::spawn(processor::processor(state.clone()));

   // Spawn Update Processor
   tokio::spawn(status_updater::task_database_status_processor(state.clone(), task_updater_rx));

    // // Spawn cron job loop
    // // Initiates work to be done on schedule tasks
    tokio::spawn(trigger_engine::cron_job_loop(state.clone()));

    //Spawn task billing processing loop
    tokio::spawn(billing::billing_usage_engine::billing_processing_loop(
        state.clone(),
    ));

    // Add the cache cleanup task here
    tokio::spawn(account_auth_middleware::cleanup_account_access_cache(state.clone()));
    tokio::spawn(bundler::cleanup_bundler_caches(state.clone()));

    // Spawn a channel monitoring task
    tokio::spawn({
        let state = state.clone();
        async move {
            loop {
                // Check channels every 30 seconds
                tokio::time::sleep(Duration::from_secs(30)).await;

                // Get processor channel capacity
                let processor_capacity = state.processor_sender.capacity();
                let processor_max = 1000; // This matches your channel size
                println!(
                    "[CHANNEL MONITOR] Processor channel: {}/{} slots available ({:.1}% full)",
                    processor_capacity,
                    processor_max,
                    ((processor_max - processor_capacity) as f64 / processor_max as f64) * 100.0
                );

                // Get task updater channel capacity
                let task_updater_capacity = state.task_updater_sender.capacity();
                let task_updater_max = 1000; // Adjust this to match your channel size
                println!(
                    "[CHANNEL MONITOR] Task updater channel: {}/{} slots available ({:.1}% full)",
                    task_updater_capacity,
                    task_updater_max,
                    ((task_updater_max - task_updater_capacity) as f64 / task_updater_max as f64) * 100.0
                );

                // Check workflow processor semaphore
                let workflow_permits = state.workflow_processor_semaphore.available_permits();
                let max_workflows = 100; // This matches your semaphore size
                println!(
                    "[CHANNEL MONITOR] Workflow processors: {}/{} available ({:.1}% in use)",
                    workflow_permits,
                    max_workflows,
                    ((max_workflows - workflow_permits) as f64 / max_workflows as f64) * 100.0
                );

                // Check flow completions size
                let completion_count = state.flow_completions.lock().await.len();
                println!(
                    "[CHANNEL MONITOR] Active flow completions: {}",
                    completion_count
                );

                // Check cache sizes - Fixed to access inner data structures
                let api_key_cache_size = state.api_key_cache.read().await.keys().len();
                // let account_access_cache_size = state.account_access_cache.read().await.get_size();  // Assuming there's a get_size() method
                // let flow_session_cache_size = state.flow_session_cache.read().await.get_size();  // Assuming there's a get_size() method
                
                println!("[CHANNEL MONITOR] Cache sizes:");
                println!("  - API Key cache: {} entries", api_key_cache_size);
                // println!("  - Account access cache: {} entries", account_access_cache_size);
                // println!("  - Flow session cache: {} entries", flow_session_cache_size);

                // Check if shutdown signal is active
                if state.shutdown_signal.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("[CHANNEL MONITOR] Shutdown signal detected, stopping monitoring");
                    break;
                }
            }
        }
    });

    // Add monitoring task with both memory, CPU and runtime stats
    tokio::spawn({
        let state = state.clone();
        async move {
            let mut last_cpu_measure = Instant::now();
            let mut last_cpu_usage = 0_f64;
            let runtime = tokio::runtime::Handle::current();

            loop {
                // Memory monitoring
                if let Ok(mem_info) = sys_info::mem_info() {
                    let used_mem_gb = (mem_info.total - mem_info.free) as f64 / 1024.0 / 1024.0;
                    let total_mem_gb = mem_info.total as f64 / 1024.0 / 1024.0;
                    let mem_usage_pct = (used_mem_gb / total_mem_gb) * 100.0;
                    
                    println!(
                        "[SYSTEM MONITOR] Memory: {:.2}GB/{:.2}GB ({:.1}%)",
                        used_mem_gb,
                        total_mem_gb,
                        mem_usage_pct
                    );

                    // Memory threshold check
                    if mem_usage_pct > 85.0 {
                        println!("[SYSTEM MONITOR] ⚠️ High memory usage detected");
                        // Clear caches
                        // let flow_cache = state.flow_session_cache.write().await;
                        // let mut api_cache = state.api_key_cache.write().await;
                        // api_cache.clear();
                        // drop(flow_cache);
                        // drop(api_cache);
                    }
                }

                // CPU monitoring
                if let Ok(cpu_load) = sys_info::loadavg() {
                    let elapsed = last_cpu_measure.elapsed().as_secs_f64();
                    let cpu_usage = cpu_load.one; // 1 minute load average
                    let cpu_change = (cpu_usage - last_cpu_usage).abs();

                    println!(
                        "[SYSTEM MONITOR] CPU Load: 1min: {:.1}, 5min: {:.1}, 15min: {:.1}",
                        cpu_load.one,
                        cpu_load.five,
                        cpu_load.fifteen,
                    );

                    // Get per-core CPU info if available
                    if let Ok(cpu_num) = sys_info::cpu_num() {
                        println!("[SYSTEM MONITOR] Number of CPUs: {}", cpu_num);
                        
                        // Calculate per-core load
                        let per_core_load = cpu_load.one / cpu_num as f64;
                        println!("[SYSTEM MONITOR] Average load per core: {:.1}%", per_core_load * 100.0);

                        // Alert on high CPU usage
                        if per_core_load > 0.8 { // 80% per core
                            println!("[SYSTEM MONITOR] ⚠️ High CPU usage detected!");
                        }
                    }

                    // Log significant CPU changes
                    if cpu_change > 0.5 && elapsed > 5.0 {
                        println!(
                            "[SYSTEM MONITOR] Significant CPU change detected: {:.1}% -> {:.1}%",
                            last_cpu_usage * 100.0,
                            cpu_usage * 100.0
                        );
                        last_cpu_measure = Instant::now();
                        last_cpu_usage = cpu_usage;
                    }
                }

                // Tokio runtime stats
                let stats = runtime.metrics();
                println!(
                    "[SYSTEM MONITOR] Tokio Runtime Stats:\n  \
                     - Active Tasks: {}\n  \
                     - Workers: {}\n  \
                     - Global Queue Depth: {}\n  \
                    ",
                    stats.num_alive_tasks(),
                    stats.num_workers(),
                    stats.global_queue_depth(),
                );

                // Check if we're in shutdown
                if state.shutdown_signal.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("[SYSTEM MONITOR] Shutdown signal detected, stopping monitoring");
                    break;
                }

                // Sleep between checks
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        }
    });

    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        sigterm.recv().await;
        println!("Received SIGTERM signal");
        // Set the shutdown signal
        state_clone.shutdown_signal.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Give time for in-flight operations to complete
        sleep(Duration::from_secs(2)).await;
    });

    // Run the API server
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

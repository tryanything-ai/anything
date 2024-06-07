use axum::{Router, 
    routing::get, Json, 
    extract::{Path, State},  
    http::{HeaderValue, Method, StatusCode}, 
    response::IntoResponse
};
use hyper::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE}; 
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;
use extism::*;
use tokio::sync::{Mutex, Semaphore};
use tokio::task;
use tokio::time::{sleep, Duration};
use chrono::{Utc, DateTime, Timelike};
use std::str::FromStr;
use serde_with::{serde_as, DisplayFromStr};

//gpt wrote this 
//https://chatgpt.com/share/65c4bd30-e38f-4db0-b410-696eb0759989

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: i32,
    data: String,
    status: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
struct Trigger {
    id: i32,
    cron_expression: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    last_run: Option<DateTime<Utc>>,
}

// Function to fetch a task from the database
async fn fetch_task(client: &Postgrest) -> Option<Task> {
    let response = client
        .from("tasks")
        .select("*")
        .eq("status", "pending")
        .limit(1)
        .execute()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    let tasks: Vec<Task> = serde_json::from_str(&response).ok()?;
    tasks.into_iter().next()
}

// Function to update the status of a task
async fn update_task_status(client: &Postgrest, task: &Task, status: &str) {
    let task = Task {
        id: task.id,
        data: task.data.clone(),
        status: status.to_string(),
    };
    client
        .from("tasks")
        .eq("id", &task.id.to_string())
        .update(serde_json::to_string(&task).unwrap())
        .execute()
        .await
        .unwrap();
}

// Function to process a task with the Extism plugin
async fn process_task(plugin: &mut Plugin, task: &Task) {
    let res = plugin.call::<&str, &str>("process_task", &task.data).unwrap();
    println!("Processed task {}: {}", task.id, res);
}

// Function to fetch triggers from the database
async fn fetch_triggers(client: &Postgrest) -> Vec<Trigger> {
    let response = client
        .from("triggers")
        .select("*")
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&response).unwrap()
}

// Function to update the last run time of a trigger
async fn update_trigger_last_run(client: &Postgrest, trigger: &Trigger) {
    let updated_trigger = Trigger {
        id: trigger.id,
        cron_expression: trigger.cron_expression.clone(),
        last_run: Some(Utc::now()),
    };
    client
        .from("triggers")
        .eq("id", &updated_trigger.id.to_string())
        .update(serde_json::to_string(&updated_trigger).unwrap())
        .execute()
        .await
        .unwrap();
}

// Function to check if the trigger should run
fn should_trigger_run(trigger: &Trigger) -> bool {
    let now = Utc::now();
    let next_run_time = cron::Schedule::from_str(&trigger.cron_expression)
        .unwrap()
        .upcoming(Utc)
        .next()
        .unwrap();

    if let Some(last_run) = trigger.last_run {
        now > next_run_time && now.minute() != last_run.minute()
    } else {
        now > next_run_time
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");

    let client = Arc::new(Postgrest::new(supabase_url.clone()).insert_header("apikey", supabase_api_key.clone()));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/workflows", get(get_workflows))
        .route("/workflow/:id", get(get_workflow))
        .route("/workflow/:id/versions", get(get_flow_versions))
        .layer(cors)
        .with_state(client.clone());

    let url = Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");
    let manifest = Manifest::new([url]);
    let plugin = Arc::new(Mutex::new(
        Plugin::new(&manifest, [], true).unwrap()
    ));

    // Create a semaphore to limit the number of concurrent tasks
    let semaphore = Arc::new(Semaphore::new(5));

    // Spawn task processing loop
    let client_clone = client.clone();
    let plugin_clone = plugin.clone();
    let semaphore_clone = semaphore.clone();

    // tokio::spawn(async move {
    //     let mut backoff = Duration::from_millis(200);

    //     loop {
    //         let task = fetch_task(&client_clone).await;
            
    //         if let Some(task) = task {
    //             backoff = Duration::from_millis(200); // Reset backoff when a task is found
    //             update_task_status(&client_clone, &task, "in_progress").await;

    //             let plugin = plugin_clone.clone();
    //             let client = client_clone.clone();
    //             let permit = semaphore_clone.clone().acquire_owned().await.unwrap();

    //             task::spawn(async move {
    //                 let mut plugin = plugin.lock().await;
    //                 process_task(&mut plugin, &task).await;
    //                 update_task_status(&client, &task, "completed").await;
    //                 drop(permit);
    //             });
    //         } else {
    //             // Increase the backoff duration, up to a maximum
    //             sleep(backoff).await;
    //             backoff = (backoff * 2).min(Duration::from_secs(60));
    //         }
    //     }
    // });

    // Spawn cron job loop
    let client_clone = client.clone();

    // tokio::spawn(async move {
    //     loop {
    //         let triggers = fetch_triggers(&client_clone).await;
    //         for trigger in triggers {
    //             if should_trigger_run(&trigger) {
    //                 // Execute the task associated with the trigger
    //                 println!("Triggering task for cron expression: {}", trigger.cron_expression);
    //                 update_trigger_last_run(&client_clone, &trigger).await;
    //             }
    //         }
    //         // Sleep for a minute before checking again
    //         sleep(Duration::from_secs(60)).await;
    //     }
    // });

    // Run the API server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}


async fn get_workflows(State(client): State<Arc<Postgrest>>, headers: HeaderMap) -> impl IntoResponse {
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flows")
        .auth(jwt)
        .select("*,flow_versions(*)")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    println!("response: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}


// New function to get a specific workflow by flow_id
async fn get_workflow(
    Path(flow_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flows")
        .auth(jwt)
        .eq("id", &flow_id)
        .select("*,flow_versions(*)")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    println!("response: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(item).into_response()
}

// New function to get flow_versions for a specific flow_id
async fn get_flow_versions(
    Path(flow_id): Path<String>,
    State(client): State<Arc<Postgrest>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flow_versions")
        .auth(jwt)
        .eq("flow_id", &flow_id)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    println!("response: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}
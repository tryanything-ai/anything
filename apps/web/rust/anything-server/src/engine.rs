use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::task;
use tokio::time::{sleep, Duration};
// use chrono::{Utc, DateTime};
use chrono::{Utc, DateTime, Timelike};
use postgrest::Postgrest;
use extism::*;
use std::str::FromStr;

use dotenv::dotenv;
use std::env;

use reqwest::Client;
// use serde_json::Value;
// use hyper::{Body, Client, Method, Request, Response};
// use hyper::client::HttpConnector;
// use hyper::header::{HeaderMap, HeaderName, HeaderValue};
// use hyper::{Body, Client, Method, Request};
// use hyper::header::{HeaderMap, HeaderName, HeaderValue};

// use serde_json::Value;

use crate::AppState; 

// #[derive(Debug, Deserialize, Serialize)]
// use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task_id: Uuid,
    pub account_id: Uuid,
    pub task_status: String,
    pub flow_id: Uuid,
    pub flow_version_id: Uuid,
    pub flow_version_name: Option<String>,
    pub trigger_id: String,
    pub trigger_session_id: String,
    pub trigger_session_status: String,
    pub flow_session_id: String,
    pub flow_session_status: String,
    pub node_id: String,
    pub is_trigger: bool,
    pub plugin_id: Option<String>,
    pub stage: String,
    pub test_config: Option<Value>,
    pub config: Value,
    pub context: Option<Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub debug_result: Option<Value>,
    pub result: Option<Value>,
    pub archived: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Trigger {
    id: i32,
    cron_expression: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    last_run: Option<DateTime<Utc>>,
}

pub async fn fetch_task(client: &Postgrest) -> Option<Task> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("task_status", "pending")
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("Error executing request: {:?}", e);
            return None;
        },
    };

    if !response.status().is_success() {
        println!("Request failed with status: {}", response.status());
        return None;
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("Error reading response body: {:?}", e);
            return None;
        },
    };

    println!("Response body: {}", body);

    let tasks: Vec<Task> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("Error parsing JSON: {:?}", e);
            return None;
        },
    };

    tasks.into_iter().next()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskInput {
    task_status: String
}

pub async fn update_task_status(client: &Postgrest, task: &Task, status: &str) {

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let input = UpdateTaskInput {
        task_status: status.to_string()
    };

    client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .eq("task_id", &task.task_id.to_string())
        .update(serde_json::to_string(&input).unwrap())
        .execute()
        .await
        .unwrap();
}

// pub async fn process_task(task: &Task) {
//     // let res = plugin.call::<&str, &str>("process_task", &task.data).unwrap();
//     println!("Processed task {}", task.task_id);
// }

pub async fn process_task(task: &Task) {
    if let Some(plugin_id) = &task.plugin_id {
        if plugin_id == "http" {
            if let (Some(method), Some(url)) = (
                task.config.get("method").and_then(Value::as_str),
                task.config.get("url").and_then(Value::as_str),
            ) {
                let client = Client::new();
                let method = match method.to_uppercase().as_str() {
                    "GET" => reqwest::Method::GET,
                    "POST" => reqwest::Method::POST,
                    "PUT" => reqwest::Method::PUT,
                    "DELETE" => reqwest::Method::DELETE,
                    _ => {
                        println!("Unsupported HTTP method: {}", method);
                        return;
                    }
                };

                let mut request_builder = client.request(method, url);

                if let Some(headers) = task.config.get("headers").and_then(Value::as_object) {
                    for (key, value) in headers {
                        if let Some(value_str) = value.as_str() {
                            request_builder = request_builder.header(key.as_str(), value_str);
                        }
                    }
                }

                if let Some(body) = task.config.get("body").and_then(Value::as_str) {
                    request_builder = request_builder.body(body.to_string());
                }

                match request_builder.send().await {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                println!("HTTP request successful. Response: {}", text);
                            }
                            Err(err) => {
                                println!("Failed to read response text: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("HTTP request failed: {}", err);
                    }
                }
            } else {
                println!("Missing required fields (method, url) in task config.");
            }
        } else {
            println!("Processed task {} with plugin_id {}", task.task_id, plugin_id);
        }
    } else {
        println!("No plugin_id found for task {}", task.task_id);
    }
}

// pub async fn process_task(task: &Task) {
//     if let Some(plugin_id) = &task.plugin_id {
//         if plugin_id == "http" {
//             if let (Some(method), Some(url)) = (
//                 task.config.get("method").and_then(Value::as_str),
//                 task.config.get("url").and_then(Value::as_str),
//             ) {
//                 let client = Client::new();
//                 let method = match method.to_uppercase().as_str() {
//                     "GET" => Method::GET,
//                     "POST" => Method::POST,
//                     "PUT" => Method::PUT,
//                     "DELETE" => Method::DELETE,
//                     _ => {
//                         println!("Unsupported HTTP method: {}", method);
//                         return;
//                     }
//                 };

//                 let mut request_builder = Request::builder()
//                     .method(method)
//                     .uri(url);

//                 if let Some(headers) = task.config.get("headers").and_then(Value::as_object) {
//                     let mut header_map = HeaderMap::new();
//                     for (key, value) in headers {
//                         if let (Ok(header_name), Ok(header_value)) = (
//                             HeaderName::from_str(key),
//                             HeaderValue::from_str(value.as_str().unwrap_or("")),
//                         ) {
//                             header_map.insert(header_name, header_value);
//                         }
//                     }
//                     request_builder = request_builder.headers(header_map);
//                 }

//                 let body = task.config.get("body").and_then(Value::as_str).unwrap_or("").to_string();
//                 let request = request_builder.body(Body::from(body)).unwrap();

//                 match client.request(request).await {
//                     Ok(response) => {
//                         let bytes = Body::to_bytes(response.into_body()).await.unwrap();
//                         let response_text = String::from_utf8(bytes.to_vec()).unwrap();
//                         println!("HTTP request successful. Response: {}", response_text);
//                     }
//                     Err(err) => {
//                         println!("HTTP request failed: {}", err);
//                     }
//                 }
//             } else {
//                 println!("Missing required fields (method, url) in task config.");
//             }
//         } else {
//             println!("Processed task {} with plugin_id {}", task.task_id, plugin_id);
//         }
//     } else {
//         println!("No plugin_id found for task {}", task.task_id);
//     }
// }

// pub async fn process_task(task: &Task) {
//     if let Some(plugin_id) = &task.plugin_id {
//         if plugin_id == "http" {
//             if let (Some(method), Some(url)) = (
//                 task.config.get("method").and_then(Value::as_str),
//                 task.config.get("url").and_then(Value::as_str),
//             ) {
//                 let client = Client::new();
//                 let method = match method.to_uppercase().as_str() {
//                     "GET" => Method::GET,
//                     "POST" => Method::POST,
//                     "PUT" => Method::PUT,
//                     "DELETE" => Method::DELETE,
//                     _ => {
//                         println!("Unsupported HTTP method: {}", method);
//                         return;
//                     }
//                 };

//                 let mut request = Request::builder()
//                     .method(method)
//                     .uri(url);

//                 if let Some(headers) = task.config.get("headers").and_then(Value::as_str) {
//                     let headers_map: Result<HeaderMap, _> = serde_json::from_str(headers);
//                     if let Ok(headers_map) = headers_map {
//                         for (key, value) in headers_map.iter() {
//                             request = request.header(key, value);
//                         }
//                     }
//                 }

//                 let body = task.config.get("body").and_then(Value::as_str).unwrap_or("").to_string();
//                 let request = request.body(Body::from(body)).unwrap();

//                 match client.request(request).await {
//                     Ok(response) => {
//                         let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
//                         let response_text = String::from_utf8(bytes.to_vec()).unwrap();
//                         println!("HTTP request successful. Response: {}", response_text);
//                     }
//                     Err(err) => {
//                         println!("HTTP request failed: {}", err);
//                     }
//                 }
//             } else {
//                 println!("Missing required fields (method, url) in task config.");
//             }
//         } else {
//             println!("Processed task {} with plugin_id {}", task.task_id, plugin_id);
//         }
//     } else {
//         println!("No plugin_id found for task {}", task.task_id);
//     }
// }

pub async fn fetch_triggers(client: &Postgrest) -> Vec<Trigger> {

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = client
        .from("triggers")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&response).unwrap()
}

pub async fn update_trigger_last_run(client: &Postgrest, trigger: &Trigger) {
    let updated_trigger = Trigger {
        id: trigger.id,
        cron_expression: trigger.cron_expression.clone(),
        last_run: Some(Utc::now()),
    };

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");


    client
        .from("triggers")
        .auth(supabase_service_role_api_key.clone())
        .eq("id", &updated_trigger.id.to_string())
        .update(serde_json::to_string(&updated_trigger).unwrap())
        .execute()
        .await
        .unwrap();
}

pub fn should_trigger_run(trigger: &Trigger) -> bool {
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

// The task processing loop function
pub async fn task_processing_loop(state: Arc<AppState>) {
    // Receive info from other systems
    let mut task_signal_rx = state.task_signal.subscribe();
    let client = state.client.clone();
    let semaphore = state.semaphore.clone();
    // To not hit db like crazy if no work to do
    let mut backoff = Duration::from_millis(200);

    loop {
        tokio::select! {
            _ = task_signal_rx.changed() => {
                println!("Received start signal, checking for tasks.");
            }
            _ = sleep(backoff) => {
                // Periodic task checking
            }
        }

        let task = fetch_task(&client).await;

        println!("Task in Loop: {:?}", task);

        if let Some(task) = task {
            backoff = Duration::from_millis(200); // Reset backoff when a task is found
            update_task_status(&client, &task, "in_progress").await;

            let client = client.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            task::spawn(async move {
                process_task(&task).await;
                update_task_status(&client, &task, "completed").await;
                drop(permit);
            });
        } else {
            // Increase the backoff duration, up to a maximum
            backoff = (backoff * 2).min(Duration::from_secs(60));
        }
    }
}

pub async fn cron_job_loop(client: Arc<Postgrest>) {
    loop {
        let triggers = fetch_triggers(&client).await;
        for trigger in triggers {
            if should_trigger_run(&trigger) {
                // Execute the task associated with the trigger
                println!("Triggering task for cron expression: {}", trigger.cron_expression);
                update_trigger_last_run(&client, &trigger).await;
            }
        }
        // Sleep for a minute before checking again
        sleep(Duration::from_secs(60)).await;
    }
}

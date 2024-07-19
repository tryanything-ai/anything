use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::task;
use tokio::time::{sleep, Duration};
// use chrono::{Utc, DateTime};
use chrono::{Utc, DateTime, Timelike};
use postgrest::Postgrest;
// use extism::*;
use std::str::FromStr;

use dotenv::dotenv;
use std::env;

use reqwest::Client;

use crate::AppState; 
use crate::workflow_types::{Task, Trigger};
use crate::execution_planner::process_trigger_task;
use crate::bundler::bundle_context;

use uuid::Uuid;
use serde_json::Value;

pub async fn fetch_task(client: &Postgrest) -> Option<Task> {

    println!("Looking for pending task");

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

    // println!("Response body: {}", body);
    // println!("Fetched Task");

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

pub async fn process_task(client: &Postgrest, task: &Task) {
    println!("Processing task");

    match bundle_context(client, task).await {
        Ok(bundled_context) => {
            if task.is_trigger {
                println!("Processed trigger task {}", task.task_id);
                if let Err(e) = process_trigger_task(client, task).await {
                    println!("Failed to process trigger task: {}", e);
                }
            } else {
                println!("Processing task {}", task.task_id);
                if let Some(plugin_id) = &task.plugin_id {
                    if plugin_id == "http" {
                        if let (Some(method), Some(url)) = (
                            bundled_context.get("method").and_then(Value::as_str),
                            bundled_context.get("url").and_then(Value::as_str),
                        ) {
                            println!("Processing HTTP task");
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

                            if let Some(headers) = bundled_context.get("headers").and_then(Value::as_object) {
                                for (key, value) in headers {
                                    if let Some(value_str) = value.as_str() {
                                        request_builder = request_builder.header(key.as_str(), value_str);
                                    }
                                }
                            }

                            if let Some(body) = bundled_context.get("body").and_then(Value::as_str) {
                                request_builder = request_builder.body(body.to_string());
                            }

                            match request_builder.send().await {
                                Ok(response) => {
                                    println!("HTTP request response! {:?}", response);
                                    match response.text().await {
                                        Ok(text) => {
                                            println!("HTTP request successful. Response: {}", text);
                                        }
                                        Err(err) => {
                                            println!("HTTP Failed to read response text: {}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    println!("HTTP request failed: {}", err);
                                }
                            }
                        } else {
                            println!("HTTP Missing required fields (method, url) in task context.");
                        }
                    } else {
                        println!("Processed task {} with plugin_id {}", task.task_id, plugin_id);
                    }
                } else {
                    println!("No plugin_id found for task {}", task.task_id);
                }
            }
        },
        Err(e) => {
            println!("Failed to bundle context: {}", e);
        }
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

        // println!("Task in Loop: {:?}", task);

        if let Some(task) = task {
            backoff = Duration::from_millis(200); // Reset backoff when a task is found
            update_task_status(&client, &task, "in_progress").await;

            let client = client.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            task::spawn(async move {
                process_task(&client, &task).await;
                update_task_status(&client, &task, "completed").await;
                drop(permit);
            });
        } else {
            // Increase the backoff duration, up to a maximum
            backoff = (backoff * 2).min(Duration::from_secs(60));
        }
    }
}

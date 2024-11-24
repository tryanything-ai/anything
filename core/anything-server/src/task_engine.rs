use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;
use tokio::time::{sleep, Duration, Instant};

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use postgrest::Postgrest;
use std::collections::HashMap;
use std::env;

use reqwest::Client;

use crate::bundler::bundle_task_context;
use crate::execution_planner::process_trigger_task;
use crate::system_actions::formatter_actions::{
    date_formatter::process_date_task, text_formatter::process_text_task,
};
use crate::system_actions::output_action::process_response_task;
use crate::workflow_types::Task;
use crate::AppState;

use std::collections::HashSet;
use tokio::sync::Mutex;

use serde_json::{json, Value};

use crate::task_types::{ActionType, FlowSessionStatus, Stage, TaskStatus, TriggerSessionStatus};

pub async fn fetch_task(client: &Postgrest, stage: &Stage) -> Option<Task> {
    let start = Instant::now();
    println!(
        "[TASK_ENGINE] Looking for oldest pending task in stage {}",
        stage.as_str()
    );

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("task_status", TaskStatus::Pending.as_str())
        .eq("flow_session_status", FlowSessionStatus::Pending.as_str())
        .eq("stage", stage.as_str())
        .order("created_at.asc")
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TASK_ENGINE] Error executing request: {:?}", e);
            return None;
        }
    };

    if !response.status().is_success() {
        println!(
            "[TASK_ENGINE] Request failed with status: {}",
            response.status()
        );
        return None;
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[TASK_ENGINE] Error reading response body: {:?}", e);
            return None;
        }
    };

    let tasks: Vec<Task> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("[TASK_ENGINE] Error parsing JSON: {:?}", e);
            return None;
        }
    };

    println!("[SPEED] fetch_task took {:?}", start.elapsed());
    tasks.into_iter().next()
}

pub async fn fetch_flow_tasks(client: &Postgrest, flow_session_id: &String) -> Option<Vec<Task>> {
    let start = Instant::now();
    println!(
        "[TASK_ENGINE] Fetching tasks for flow_session_id {}",
        flow_session_id
    );

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("flow_session_id", flow_session_id.to_string())
        .order("processing_order.asc")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TASK_ENGINE] Error executing request: {:?}", e);
            return None;
        }
    };

    if !response.status().is_success() {
        println!(
            "[TASK_ENGINE] Request failed with status: {}",
            response.status()
        );
        return None;
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[TASK_ENGINE] Error reading response body: {:?}", e);
            return None;
        }
    };

    let tasks: Vec<Task> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("[TASK_ENGINE] Error parsing JSON: {:?}", e);
            return None;
        }
    };

    if tasks.is_empty() {
        return None;
    }

    println!("[SPEED] fetch_flow_tasks took {:?}", start.elapsed());
    Some(tasks)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskInput {
    task_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ended_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
}

pub async fn update_task_status(
    client: &Postgrest,
    task: &Task,
    status: &TaskStatus,
    result: Option<Value>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let started_at = if status.as_str() == TaskStatus::Running.as_str() {
        Some(Utc::now())
    } else {
        None
    };

    let ended_at = if status.as_str() != TaskStatus::Running.as_str() {
        Some(Utc::now())
    } else {
        None
    };

    let input = UpdateTaskInput {
        task_status: status.as_str().to_string(),
        started_at,
        ended_at,
        result,
    };

    client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .eq("task_id", &task.task_id.to_string())
        .update(serde_json::to_string(&input)?)
        .execute()
        .await?;

    println!("[SPEED] update_task_status took {:?}", start.elapsed());
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFlowSesssionInput {
    flow_session_status: String,
    trigger_session_status: String,
}

pub async fn update_flow_session_status(
    client: &Postgrest,
    flow_session_id: &String,
    flow_session_status: &FlowSessionStatus,
    trigger_session_status: &TriggerSessionStatus,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let input = UpdateFlowSesssionInput {
        flow_session_status: flow_session_status.as_str().to_string(),
        trigger_session_status: trigger_session_status.as_str().to_string(),
    };

    client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .eq("flow_session_id", &flow_session_id.to_string())
        .update(serde_json::to_string(&input)?)
        .execute()
        .await?;

    println!(
        "[SPEED] update_flow_session_status took {:?}",
        start.elapsed()
    );
    Ok(())
}


pub async fn process_flow_tasks(
    client: &Postgrest,
    flow_session_id: &String,
    state: Arc<AppState>,
) {
    let start = Instant::now();
    println!("[PROCESS FLOW TASKS] Starting to process new flow");
    println!("[PROCESS FLOW TASKS] Flow session ID: {}", flow_session_id);

    if let Some(tasks) = fetch_flow_tasks(client, flow_session_id).await {
        println!(
            "[PROCESS FLOW TASKS] Found {} tasks to process",
            tasks.len()
        );
        let mut final_result = None;
        let mut all_tasks_completed = true;

        for task in &tasks {
            let task_start = Instant::now();
            if task.task_status == TaskStatus::Completed.as_str().to_string() {
                println!("[PROCESS FLOW TASKS] Found completed task {}", task.task_id);
                final_result = task.result.clone();
            } else if task.task_status == TaskStatus::Pending.as_str().to_string() {
                println!(
                    "[PROCESS FLOW TASKS] Processing pending task {}",
                    task.task_id
                );
                match process_task(client, task, &state.http_client).await {
                    Ok(result) => {
                        println!(
                            "[PROCESS FLOW TASKS] Successfully processed task {}",
                            task.task_id
                        );
                        final_result = Some(result.clone());
                    }
                    Err(error_result) => {
                        println!(
                            "[PROCESS FLOW TASKS] Error processing task {}: {}",
                            task.task_id, error_result
                        );
                        all_tasks_completed = false;

                        final_result = Some(error_result.clone());

                        // Update tasks with process_order > current task to cancelled state
                        let current_process_order = task.processing_order;
                        println!(
                            "[PROCESS FLOW TASKS] Cancelling subsequent tasks after failed task {}",
                            task.task_id
                        );
                        for remaining_task in &tasks {
                            if remaining_task.processing_order > current_process_order
                                && remaining_task.task_status
                                    != TaskStatus::Completed.as_str().to_string()
                            {
                                println!(
                                    "[PROCESS FLOW TASKS] Cancelling task {}",
                                    remaining_task.task_id
                                );
                                if let Err(update_err) = update_task_status(
                                    client,
                                    remaining_task,
                                    &TaskStatus::Canceled,
                                    Some(serde_json::json!({
                                        "error": format!("Cancelled due to error in a previous task named {}", task.action_id)
                                    })),
                                )
                                .await
                                {
                                    println!(
                                        "[PROCESS FLOW TASKS] Error updating task status for {}: {}",
                                        remaining_task.task_id, update_err
                                    );
                                }
                            }
                        }
                        break;
                    }
                }
            }
            println!("[SPEED] Task {} took {:?}", task.task_id, task_start.elapsed());
        }

        // Update flow session status
        let flow_session_status = if all_tasks_completed {
            println!("[PROCESS FLOW TASKS] All tasks completed successfully");
            FlowSessionStatus::Completed
        } else {
            println!("[PROCESS FLOW TASKS] Some tasks failed");
            FlowSessionStatus::Failed
        };

        let trigger_session_status = if all_tasks_completed {
            TriggerSessionStatus::Completed
        } else {
            TriggerSessionStatus::Failed
        };

        println!(
            "[PROCESS FLOW TASKS] Updating flow session status to {:?}",
            flow_session_status
        );
        if let Err(e) = update_flow_session_status(
            client,
            flow_session_id,
            &flow_session_status,
            &trigger_session_status,
        )
        .await
        {
            println!(
                "[PROCESS FLOW TASKS] Error updating flow session status: {}",
                e
            );
        }

        // Send result through completion channel if it exists
        // This is where we handle the webhook response
        println!("[PROCESS FLOW TASKS] Checking for completion channel");
        println!("[PROCESS FLOW TASKS] Final result: {:?}", final_result);

        let mut completions = state.flow_completions.lock().await;
        if let Some(completion) = completions.remove(flow_session_id) {
            if completion.needs_response {
                println!("[PROCESS FLOW TASKS] Sending result through completion channel");
                let _ = completion.sender.send(final_result.unwrap_or(json!({
                    "status": "completed",
                    "message": "Workflow completed successfully"
                })));
            }
        }
    } else {
        println!(
            "[PROCESS FLOW TASKS] No tasks found for flow session {}",
            flow_session_id
        );
    }
    println!("[SPEED] Total flow processing took {:?}", start.elapsed());
}

pub async fn process_task(
    client: &Postgrest,
    task: &Task,
    http_client: &Client,
) -> Result<Value, Value> {
    let start = Instant::now();
    println!("[PROCESS TASK] Processing task {}", task.task_id);

    // Run status update and context bundling in parallel
    let (status_result, context_result) = tokio::join!(
        update_task_status(client, task, &TaskStatus::Running, None),
        bundle_task_context(client, task, true)
    );

    // Check status update result
    if let Err(e) = status_result {
        let error_value = json!({
            "error": format!("Failed to update task status to running: {}", e)
        });
        return Err(error_value);
    }

    let result = match context_result {
        Ok(bundled_context) => {
            let task_result = if task.r#type == ActionType::Trigger.as_str().to_string() {
                println!("[PROCESS TASK] Processing trigger task {}", task.task_id);
                process_trigger_task(client, task).await
            } else {
                println!("[PROCESS TASK] Processing regular task {}", task.task_id);
                match &task.plugin_id {
                    Some(plugin_id) => match plugin_id.as_str() {
                        "http" => process_http_task(&bundled_context, http_client).await,
                        "response" => process_response_task(&bundled_context).await,
                        "format_text" => process_text_task(&bundled_context).await,
                        "format_date" => process_date_task(&bundled_context).await,
                        _ => Ok(json!({
                            "message": format!("Processed task {} with plugin_id {}", task.task_id, plugin_id)
                        })),
                    },
                    None => Ok(json!({
                        "message": format!("No plugin_id found for task {}", task.task_id)
                    })),
                }
            };

            match task_result {
                Ok(result) => Ok(result),
                Err(e) => Err(json!({
                    "error": e.to_string()
                })),
            }
        }
        Err(e) => Err(json!({
            "error": format!("Failed to bundle task context: {}", e)
        })),
    };

    match result {
        Ok(success_result) => {
            // Update task status to "completed" with the result
            if let Err(e) = update_task_status(
                client,
                task,
                &TaskStatus::Completed,
                Some(success_result.clone()),
            )
            .await
            {
                let error_value = json!({
                    "error": format!("Task succeeded but failed to update status: {}", e)
                });
                return Err(error_value);
            }
            println!(
                "[PROCESS TASK] Task {} completed successfully",
                task.task_id
            );
            println!(
                "[SPEED] Task {} processing took {:?}",
                task.task_id,
                start.elapsed()
            );
            Ok(success_result)
        }
        Err(error_result) => {
            // Update task status to "failed" with the error message
            if let Err(e) = update_task_status(
                client,
                task,
                &TaskStatus::Failed,
                Some(error_result.clone()),
            )
            .await
            {
                let error_value = json!({
                    "error": format!("Task failed and failed to update status: {}. Original error: {}", e, error_result)
                });
                return Err(error_value);
            }

            println!(
                "[PROCESS TASK] Task {} failed: {}",
                task.task_id, error_result
            );
            println!(
                "[SPEED] Failed task {} processing took {:?}",
                task.task_id,
                start.elapsed()
            );
            Err(error_result)
        }
    }
}

async fn process_http_task(
    bundled_context: &Value,
    http_client: &Client,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[TASK_ENGINE] Entering process_http_task");
    println!("[TASK_ENGINE] Bundled context: {:?}", bundled_context);

    if let (Some(method), Some(url)) = (
        bundled_context.get("method").and_then(Value::as_str),
        bundled_context.get("url").and_then(Value::as_str),
    ) {
        println!(
            "[TASK_ENGINE] Processing HTTP task with method: {}, url: {}",
            method, url
        );
        let method = match method.to_uppercase().as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            "HEAD" => reqwest::Method::HEAD,
            "OPTIONS" => reqwest::Method::OPTIONS,
            "PATCH" => reqwest::Method::PATCH,
            _ => {
                println!("[TASK_ENGINE] Unsupported HTTP method: {}", method);
                return Err(format!("Unsupported HTTP method: {}", method).into());
            }
        };

        let mut request_builder = http_client.request(method, url);

        println!("[TASK_ENGINE] Processing headers");
        if let Some(headers) = bundled_context.get("headers") {
            match headers {
                Value::Object(headers_obj) => {
                    println!("[TASK_ENGINE] Headers are an object: {:?}", headers_obj);
                    for (key, value) in headers_obj {
                        if let Some(value_str) = value.as_str() {
                            println!("[TASK_ENGINE] Adding header: {} = {}", key, value_str);
                            request_builder = request_builder.header(key.as_str(), value_str);
                        }
                    }
                }
                Value::String(headers_str) => {
                    println!("[TASK_ENGINE] Headers are a string: {}", headers_str);
                    match serde_json::from_str::<Value>(headers_str) {
                        Ok(Value::Object(parsed_headers)) => {
                            println!("[TASK_ENGINE] Parsed headers: {:?}", parsed_headers);
                            for (key, value) in parsed_headers {
                                if let Some(value_str) = value.as_str() {
                                    println!(
                                        "[TASK_ENGINE] Adding header: {} = {}",
                                        key, value_str
                                    );
                                    request_builder =
                                        request_builder.header(key.as_str(), value_str);
                                }
                            }
                        }
                        _ => {
                            println!("[TASK_ENGINE] Failed to parse headers string as JSON object")
                        }
                    }
                }
                _ => println!("[TASK_ENGINE] Headers are neither an object nor a string"),
            }
        } else {
            println!("[TASK_ENGINE] No headers found in bundled context");
        }

        if let Some(body) = bundled_context.get("body") {
            if let Some(body_str) = body.as_str() {
                if !body_str.is_empty() {
                    println!("[TASK_ENGINE] Adding body: {}", body_str);
                    request_builder = request_builder.body(body_str.to_string());
                } else {
                    println!("[TASK_ENGINE] Body is an empty string, sending request without body");
                }
            } else if let Some(body_object) = body.as_object() {
                let body_json = serde_json::to_string(body_object)?;
                println!("[TASK_ENGINE] Adding body: {}", body_json);
                request_builder = request_builder.body(body_json);
            } else {
                println!("[TASK_ENGINE] Body is not a string or an object");
                return Err("HTTP task body must be a string or an object".into());
            }
        } else {
            println!("[TASK_ENGINE] No body found in task context");
        }

        println!("[TASK_ENGINE] Sending HTTP request");
        let request_start = Instant::now();
        let response = request_builder.send().await?;
        println!("[SPEED] HTTP request took {:?}", request_start.elapsed());
        println!(
            "[TASK_ENGINE] HTTP request response received: {:?}",
            response
        );
        let status = response.status();
        let headers = response.headers().clone();
        let _content_type = response
            .headers()
            .get("content-type")
            .map(|v| v.to_str().unwrap_or(""));

        // Try to parse the response as JSON, if it fails, return the raw text
        let body = match response.text().await {
            Ok(text) => {
                println!("[TASK_ENGINE] Response text: {}", text);
                match serde_json::from_str::<Value>(&text) {
                    Ok(json_value) => {
                        println!(
                            "[TASK_ENGINE] HTTP request successful. JSON Response: {:?}",
                            json_value
                        );
                        json_value
                    }
                    Err(_) => {
                        println!(
                            "[TASK_ENGINE] HTTP request successful. Text Response: {}",
                            text
                        );
                        Value::String(text)
                    }
                }
            }
            Err(e) => {
                println!("[TASK_ENGINE] Error reading response body: {:?}", e);
                return Err(e.into());
            }
        };

        let result = serde_json::json!({
            "status_code": status.as_u16(),
            "headers": headers
                .iter()
                .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>(),
            "body": body
        });

        println!("[TASK_ENGINE] Returning result: {:?}", result);
        println!(
            "[SPEED] Total HTTP task processing took {:?}",
            start.elapsed()
        );
        Ok(result)
    } else {
        println!("[TASK_ENGINE] Missing required fields (method, url) in task context");
        Err("HTTP Missing required fields (method, url) in task context.".into())
    }
}

// The task processing loop function
pub async fn task_processing_loop(state: Arc<AppState>) {
    // Receive info from other systems
    let mut task_signal_rx = state.task_engine_signal.subscribe();
    let client = state.anything_client.clone();
    let semaphore = state.semaphore.clone();
    // To not hit db like crazy if no work to do
    let mut backoff = Duration::from_millis(200);

    let active_flow_sessions = Arc::new(Mutex::new(HashSet::new()));

    loop {
        tokio::select! {
            _ = task_signal_rx.changed() => {
                println!("[TASK_ENGINE] Received start signal, checking for tasks.");
            }
            _ = sleep(backoff) => {
                // Periodic task checking
            }
        }

        let (task_testing, task_production) = tokio::join!(
            fetch_task(&client, &Stage::Testing),
            fetch_task(&client, &Stage::Production)
        );

        if let Some(task) = task_testing.or(task_production) {
            backoff = Duration::from_millis(200); // Reset backoff when tasks are found
            let client = client.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            // let active_flow_sessions = active_flow_sessions.clone();
            let active_flow_sessions = Arc::clone(&active_flow_sessions);

            // Clone state for the spawned task
            let state = Arc::clone(&state);

            task::spawn(async move {
                let flow_session_id = task.flow_session_id.clone();

                // Try to acquire a lock for this flow_session_id
                let mut active_sessions = active_flow_sessions.lock().await;
                if !active_sessions.insert(flow_session_id.clone()) {
                    println!(
                        "[TASK_ENGINE] Flow session {} is already being processed",
                        flow_session_id
                    );
                    drop(permit);
                    return;
                }
                drop(active_sessions);
                // Update flow session status to "running"
                // Prevents other workers from picking up the same flow session ( maybe not perfect )
                if let Err(e) = update_flow_session_status(
                    &client,
                    &task.flow_session_id,
                    &FlowSessionStatus::Running,
                    &TriggerSessionStatus::Running,
                )
                .await
                {
                    println!(
                        "[TASK_ENGINE] Error updating flow session status to processing: {}",
                        e
                    );
                    active_flow_sessions.lock().await.remove(&flow_session_id);
                    drop(permit);
                    return;
                }

                // Process the trigger task if it is a trigger
                if task.r#type == ActionType::Trigger.as_str().to_string() {
                    if let Err(e) = process_task(&client, &task, &state.http_client).await {
                        println!("[TASK_ENGINE] Failed to process trigger task: {}", e);
                    }
                }

                // Process rest of flow
                process_flow_tasks(&client, &flow_session_id, state).await;

                // Remove the flow_session_id from active sessions
                active_flow_sessions.lock().await.remove(&flow_session_id);
                drop(permit);
            });
        } else {
            // Increase the backoff duration, up to a maximum
            backoff = (backoff * 2).min(Duration::from_secs(60));
        }
    }
}

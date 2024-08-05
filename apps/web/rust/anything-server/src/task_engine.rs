use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;
use tokio::time::{sleep, Duration};

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use postgrest::Postgrest;
use std::collections::HashMap;
use std::env;

use reqwest::Client;

use crate::bundler::bundle_context;
use crate::execution_planner::process_trigger_task;
use crate::workflow_types::Task;
use crate::AppState;

use serde_json::Value;

use crate::task_types::{ActionType, FlowSessionStatus, Stage, TaskStatus, TriggerSessionStatus};

pub async fn fetch_task(client: &Postgrest, stage: &Stage) -> Option<Task> {
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

    tasks.into_iter().next()
}

pub async fn fetch_flow_tasks(client: &Postgrest, flow_session_id: &String) -> Option<Vec<Task>> {
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

    Ok(())
}

pub async fn process_flow_tasks(client: &Postgrest, flow_session_id: &String) {
    println!("[TASK_ENGINE] [PROCESSING_NEW_FLOW]");

    if let Some(tasks) = fetch_flow_tasks(client, flow_session_id).await {
        let mut all_tasks_completed = true;

        for task in &tasks {
            if task.task_status == TaskStatus::Pending.as_str().to_string() {
                match process_task(client, task).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "[TASK_ENGINE] Error processing task {}: {}",
                            task.task_id, e
                        );
                        all_tasks_completed = false;

                        // Update tasks with process_order > current task to cancelled state
                        let current_process_order = task.processing_order;
                        for remaining_task in &tasks {
                            if remaining_task.processing_order > current_process_order
                                && remaining_task.task_status
                                    != TaskStatus::Completed.as_str().to_string()
                            {
                                if let Err(update_err) = update_task_status(
                                    client,
                                    remaining_task,
                                    &TaskStatus::Canceled,
                                    Some(serde_json::json!({
                                        "error": format!("Cancelled due to error in previous task: {}", e)
                                    })),
                                )
                                .await
                                {
                                    println!(
                                        "[TASK_ENGINE] Error updating task status: {}",
                                        update_err
                                    );
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Update flow session status
        let flow_session_status = if all_tasks_completed {
            FlowSessionStatus::Completed
        } else {
            FlowSessionStatus::Failed
        };

        let trigger_session_status = if all_tasks_completed {
            TriggerSessionStatus::Completed
        } else {
            TriggerSessionStatus::Failed
        };

        if let Err(e) = update_flow_session_status(
            client,
            flow_session_id,
            &flow_session_status,
            &trigger_session_status,
        )
        .await
        {
            println!("[TASK_ENGINE] Error updating flow session status: {}", e);
        }
    }
}

// pub async fn process_flow_tasks(client: &Postgrest, flow_session_id: &String) {
//     println!("[TASK_ENGINE] [PROCESSING_NEW_FLOW]");

//     if let Some(tasks) = fetch_flow_tasks(client, flow_session_id).await {
//         let mut all_tasks_completed = true;

//         for task in &tasks {
//             if task.task_status == TaskStatus::Pending.as_str().to_string() {
//                 match process_task(client, task).await {
//                     Ok(_) => {}
//                     Err(e) => {
//                         println!(
//                             "[TASK_ENGINE] Error processing task {}: {}",
//                             task.task_id, e
//                         );
//                         all_tasks_completed = false;

//                         // Update remaining tasks to cancelled state
//                         for remaining_task in &tasks {
//                             if remaining_task.task_status
//                                 != TaskStatus::Completed.as_str().to_string()
//                             {
//                                 if let Err(update_err) = update_task_status(
//                                     client,
//                                     remaining_task,
//                                     &TaskStatus::Canceled,
//                                     None,
//                                 )
//                                 .await
//                                 {
//                                     println!(
//                                         "[TASK_ENGINE] Error updating task status: {}",
//                                         update_err
//                                     );
//                                 }
//                             }
//                         }
//                         break;
//                     }
//                 }
//             }
//         }

//         // Update flow session status
//         let flow_session_status = if all_tasks_completed {
//             FlowSessionStatus::Completed
//         } else {
//             FlowSessionStatus::Failed
//         };

//         let trigger_session_status = if all_tasks_completed {
//             TriggerSessionStatus::Completed
//         } else {
//             TriggerSessionStatus::Failed
//         };

//         if let Err(e) = update_flow_session_status(
//             client,
//             flow_session_id,
//             &flow_session_status,
//             &trigger_session_status,
//         )
//         .await
//         {
//             println!("[TASK_ENGINE] Error updating flow session status: {}", e);
//         }
//     }
// }
// pub async fn process_flow_tasks(client: &Postgrest, flow_session_id: &String) {

//     println!("[TASK_ENGINE] [PROCESSING_NEW_FLOW]");

//     if let Some(tasks) = fetch_flow_tasks(client, flow_session_id).await {
//         let mut all_tasks_completed = true;

//         for task in &tasks {
//             if task.task_status == TaskStatus::Pending.as_str().to_string() {
//                 if let Err(e) = process_task(client, task).await {
//                     println!("[TASK_ENGINE] Error processing task {}: {}", task.task_id, e);
//                     all_tasks_completed = false;

//                     // Update remaining tasks to cancelled state
//                     for remaining_task in &tasks {
//                         if remaining_task.task_status != TaskStatus::Completed.as_str().to_string() {
//                             if let Err(update_err) = update_task_status(client, remaining_task, &TaskStatus::Canceled).await {
//                                 println!("[TASK_ENGINE] Error updating task status: {}", update_err);
//                             }
//                         }
//                     }
//                     break;
//                 }
//             }
//         }

//         // Update flow session status
//         let flow_session_status = if all_tasks_completed {
//             FlowSessionStatus::Completed
//         } else {
//             FlowSessionStatus::Failed
//         };

//         let trigger_session_status = if all_tasks_completed {
//             TriggerSessionStatus::Completed
//         } else {
//             TriggerSessionStatus::Failed
//         };

//         if let Err(e) = update_flow_session_status(client, flow_session_id, &flow_session_status, &trigger_session_status).await {
//             println!("[TASK_ENGINE] Error updating flow session status: {}", e);
//         }
//     }
// }

pub async fn process_task(
    client: &Postgrest,
    task: &Task,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    println!("[TASK_ENGINE] Processing task {}", task.task_id);

    // Update task status to "running"
    update_task_status(client, task, &TaskStatus::Running, None).await?;

    let result: Result<Value, Box<dyn std::error::Error + Send + Sync>> = async {
        let bundled_context = bundle_context(client, task).await?;

        let task_result = if task.action_type == ActionType::Trigger.as_str().to_string() {
            println!("[TASK_ENGINE] Processing trigger task {}", task.task_id);
            process_trigger_task(client, task).await?
        } else {
            println!("[TASK_ENGINE] Processing regular task {}", task.task_id);
            if let Some(plugin_id) = &task.plugin_id {
                if plugin_id == "http" {
                    process_http_task(&bundled_context).await?
                } else {
                    serde_json::json!({
                        "message": format!("Processed task {} with plugin_id {}", task.task_id, plugin_id)
                    })
                }
            } else {
                serde_json::json!({
                    "message": format!("No plugin_id found for task {}", task.task_id)
                })
            }
        };

        Ok(task_result)
    }.await;

    match result {
        Ok(task_result) => {
            // Update task status to "completed" with the result
            update_task_status(
                client,
                task,
                &TaskStatus::Completed,
                Some(task_result.clone()),
            )
            .await?;
            println!("[TASK_ENGINE] Task {} completed successfully", task.task_id);
            Ok(task_result)
        }
        Err(e) => {
            // Update task status to "error" with the error message
            let error_result = serde_json::json!({
                "error": e.to_string()
            });
            update_task_status(
                client,
                task,
                &TaskStatus::Failed,
                Some(error_result.clone()),
            )
            .await?;
            println!("[TASK_ENGINE] Task {} failed: {}", task.task_id, e);
            Err(e)
        }
    }
}

async fn process_http_task(
    bundled_context: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    if let (Some(method), Some(url)) = (
        bundled_context.get("method").and_then(Value::as_str),
        bundled_context.get("url").and_then(Value::as_str),
    ) {
        println!("[TASK_ENGINE] Processing HTTP task");
        let client = Client::new();
        let method = match method.to_uppercase().as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            _ => return Err(format!("Unsupported HTTP method: {}", method).into()),
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

        let response = request_builder.send().await?;
        println!("[TASK_ENGINE] HTTP request response! {:?}", response);
        let status = response.status();
        let headers = response.headers().clone();

        // Try to parse the response as JSON, if it fails, return the raw text
        let body = match response.text().await {
            Ok(text) => match serde_json::from_str::<Value>(&text) {
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
            },
            Err(e) => {
                println!("[TASK_ENGINE] Error reading response body: {:?}", e);
                return Err(e.into());
            }
        };

        Ok(serde_json::json!({
            "status": status.as_u16(),
            "headers": headers
                .iter()
                .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>(),
            "body": body
        }))
    } else {
        Err("HTTP Missing required fields (method, url) in task context.".into())
    }
}
// async fn process_http_task(
//     bundled_context: &Value,
// ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
//     if let (Some(method), Some(url)) = (
//         bundled_context.get("method").and_then(Value::as_str),
//         bundled_context.get("url").and_then(Value::as_str),
//     ) {
//         println!("[TASK_ENGINE] Processing HTTP task");
//         let client = Client::new();
//         let method = match method.to_uppercase().as_str() {
//             "GET" => reqwest::Method::GET,
//             "POST" => reqwest::Method::POST,
//             "PUT" => reqwest::Method::PUT,
//             "DELETE" => reqwest::Method::DELETE,
//             _ => return Err(format!("Unsupported HTTP method: {}", method).into()),
//         };

//         let mut request_builder = client.request(method, url);

//         if let Some(headers) = bundled_context.get("headers").and_then(Value::as_object) {
//             for (key, value) in headers {
//                 if let Some(value_str) = value.as_str() {
//                     request_builder = request_builder.header(key.as_str(), value_str);
//                 }
//             }
//         }

//         if let Some(body) = bundled_context.get("body").and_then(Value::as_str) {
//             request_builder = request_builder.body(body.to_string());
//         }

//         let response = request_builder.send().await?;
//         println!("[TASK_ENGINE] HTTP request response! {:?}", response);
//         let status = response.status();
//         let headers = response.headers().clone();
//         let body: Value = response.json().await?;
//         println!("[TASK_ENGINE] HTTP request successful. Response: {}", body);

//         Ok(serde_json::json!({
//             "status": status.as_u16(),
//             "headers": headers
//                 .iter()
//                 .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
//                 .collect::<HashMap<String, String>>(),
//             "body": body
//         }))
//     } else {
//         Err("HTTP Missing required fields (method, url) in task context.".into())
//     }
// }

// The task processing loop function
pub async fn task_processing_loop(state: Arc<AppState>) {
    // Receive info from other systems
    let mut task_signal_rx = state.task_engine_signal.subscribe();
    let client = state.anything_client.clone();
    let semaphore = state.semaphore.clone();
    // To not hit db like crazy if no work to do
    let mut backoff = Duration::from_millis(200);

    loop {
        tokio::select! {
            _ = task_signal_rx.changed() => {
                println!("[TASK_ENGINE] Received start signal, checking for tasks.");
            }
            _ = sleep(backoff) => {
                // Periodic task checking
            }
        }

        let task_testing = fetch_task(&client, &Stage::Testing).await;
        let task_production = fetch_task(&client, &Stage::Production).await;

        if let Some(task) = task_testing.or(task_production) {
            backoff = Duration::from_millis(200); // Reset backoff when tasks are found
            let client = client.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            task::spawn(async move {
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
                    return;
                }

                // Process the trigger task if it is a trigger
                if task.action_type == ActionType::Trigger.as_str().to_string() {
                    if let Err(e) = process_task(&client, &task).await {
                        println!("[TASK_ENGINE] Failed to process trigger task: {}", e);
                    }
                }

                //Process rest of flwo
                process_flow_tasks(&client, &task.flow_session_id).await;

                drop(permit);
            });
        } else {
            // Increase the backoff duration, up to a maximum
            backoff = (backoff * 2).min(Duration::from_secs(60));
        }
    }
}

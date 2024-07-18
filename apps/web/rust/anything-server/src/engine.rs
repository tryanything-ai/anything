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

pub async fn process_task(task: &Task) {
    // let res = plugin.call::<&str, &str>("process_task", &task.data).unwrap();
    println!("Processed task {}", task.task_id);
}

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

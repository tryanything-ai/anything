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

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: i32,
    data: String,
    status: String,
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

pub async fn update_task_status(client: &Postgrest, task: &Task, status: &str) {
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

pub async fn process_task(task: &Task) {
    // let res = plugin.call::<&str, &str>("process_task", &task.data).unwrap();
    println!("Processed task {}", task.id);
}

pub async fn fetch_triggers(client: &Postgrest) -> Vec<Trigger> {
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

pub async fn update_trigger_last_run(client: &Postgrest, trigger: &Trigger) {
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

pub async fn task_processing_loop(client: Arc<Postgrest>, semaphore: Arc<Semaphore>) {
    //To not hit db like crazy if no work to do. 
    let mut backoff = Duration::from_millis(200);

    loop {
        let task = fetch_task(&client).await;

        println!("Task in Loop: {:?}", task);
        
        if let Some(task) = task {
            backoff = Duration::from_millis(200); // Reset backoff when a task is found
            update_task_status(&client, &task, "in_progress").await;

            // let plugin = plugin.clone();
            let client = client.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            task::spawn(async move {
                // let mut plugin = plugin.lock().await;
                process_task(&task).await;
                update_task_status(&client, &task, "completed").await;
                drop(permit);
            });
        } else {
            // Increase the backoff duration, up to a maximum
            sleep(backoff).await;
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

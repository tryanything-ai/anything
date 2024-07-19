use tokio::time::{sleep, Duration};
use chrono::{Utc, DateTime, Timelike};
use postgrest::Postgrest;

use dotenv::dotenv;
use std::env;

// use serde_json::Value;
use std::sync::Arc;

use crate::AppState;
// use crate::workflow_types::{Task};
use std::collections::HashMap;
use tokio::sync::RwLock;

use cron::Schedule;
use serde_json::Value;
use std::str::FromStr;

pub struct TriggerEngineState {
    pub triggers: Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
}

#[derive(Debug, Clone)]
pub struct InMemoryTrigger {
    pub trigger_id: String,                //type of trigger essentially
    pub flow_id: String,                   //flow id
    pub flow_version_id: String,           // Some identifier for the task related to the trigger
    pub config: Value,                     // Store the trigger configuration
    pub last_fired: Option<DateTime<Utc>>, //data so we know when it was last fired
    pub next_fire: Option<DateTime<Utc>>,  //data so we know when it will fire next
}

pub async fn cron_job_loop(client: Arc<Postgrest>) {
    let trigger_state = Arc::new(RwLock::new(HashMap::new()));

    // Initial hydration of known cron triggers
    hydrate_triggers(&client, &trigger_state).await;

    // Refresh interval for checking the database
    let refresh_interval = Duration::from_secs(60);

    loop {
        // Check if any triggers should run
        {
            let triggers = trigger_state.read().await;
            for (id, trigger) in triggers.iter() {
                println!("Checking trigger for flow_version_id: {}", id);
                if should_trigger_run(trigger) {
                    // Execute the task associated with the trigger
                    println!("Running triggering task for trigger: {}", trigger.trigger_id);
                    // TODO: Execute the task by creating a trigger task
                    update_trigger_last_run(trigger, &trigger_state).await;
                } else {
                    println!("Trigger {} should not run", id);
                }
            }
        }

        // Sleep for a short duration before the next check
        sleep(refresh_interval).await;

        // Periodically refresh triggers from the database
        hydrate_triggers(&client, &trigger_state).await;
    }
}
// pub async fn cron_job_loop(client: Arc<Postgrest>) {
//     let trigger_state = Arc::new(RwLock::new(HashMap::new()));

//     // Initial hydration of known cron triggers
//     hydrate_triggers(&client, &trigger_state).await;

//     // Refresh interval for checking the database
//     let refresh_interval = Duration::from_secs(60);

//     loop {
//         // Check if any triggers should run
//         {
//             let triggers = trigger_state.read().await;
//             for (id, trigger) in triggers.iter() {
//                 println!("Checking trigger: {}", id);
//                 if should_trigger_run(trigger) {
//                     // Execute the task associated with the trigger
//                     println!("Running triggering task for trigger: {}", trigger.trigger_id);
//                        // TODO: Execute the task by creating a trigger task
//                        update_trigger_last_run(trigger, &trigger_state).await;
//                 } else {
//                     println!("Trigger {} should not run", id); 
//                 }
//             }
//         }

//         // Sleep for a short duration before the next check
//         sleep(refresh_interval).await;

//         // Periodically refresh triggers from the database
//         hydrate_triggers(&client, &trigger_state).await;
//     }
// }

pub async fn hydrate_triggers(client: &Postgrest, triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>) {
    println!("Hydrating triggers from the database");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .select("flow_id, flow_version_id, flow_definition") // TODO: only fetch active flows
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("Error fetching flow versions: {:?}", e);
            return;
        },
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("Error reading response body: {:?}", e);
            return;
        },
    };

    let flow_versions: Vec<Value> = match serde_json::from_str(&body) {
        Ok(flow_versions) => flow_versions,
        Err(e) => {
            println!("Error parsing JSON: {:?}", e);
            return;
        },
    };

    println!("Found flow_versions vector: {}", flow_versions.len());

    let mut new_triggers = HashMap::new();

    for flow_version in flow_versions {
        if let (Some(flow_id), Some(flow_version_id), Some(flow_definition)) = (
            flow_version.get("flow_id").and_then(|v| v.as_str()),
            flow_version.get("flow_version_id").and_then(|v| v.as_str()),
            flow_version.get("flow_definition"),
        ) {
            if let Some(actions) = flow_definition.get("actions").and_then(|v| v.as_array()) {
                for action in actions {
                    if let (Some(trigger_id), Some(trigger_type)) = (
                        action.get("node_id").and_then(|v| v.as_str()),
                        action.get("type").and_then(|v| v.as_str()),
                    ) {
                        if trigger_type == "trigger" {
                            println!("Found trigger action of type trigger");

                            let input = action.get("input").cloned().unwrap_or_default();
                            let variables = action.get("variables").cloned().unwrap_or_default();

                            let config = serde_json::json!({
                                "input": input,
                                "variables": variables,
                            });

                            println!("Creating trigger with config: {:?}", config);

                            // Parse the cron expression and calculate the next fire time
                            let cron_expression = config["input"]["cron_expression"]
                                .as_str()
                                .unwrap_or("* * * * *");

                            let next_fire = match Schedule::from_str(cron_expression) {
                                Ok(schedule) => schedule.upcoming(Utc).next(),
                                Err(e) => {
                                    println!("Error parsing cron expression: {}", e);
                                    None
                                }
                            };

                            let new_trigger = InMemoryTrigger {
                                trigger_id: trigger_id.to_string(),
                                flow_id: flow_id.to_string(),
                                flow_version_id: flow_version_id.to_string(),
                                config,
                                last_fired: None,
                                next_fire,
                            };

                            // Check if the trigger already exists in memory
                            if let Some(existing_trigger) = triggers.read().await.get(flow_version_id) {
                                println!("Trigger already exists, preserving last_fired and next_fire values");
                                new_triggers.insert(flow_version_id.to_string(), InMemoryTrigger {
                                    last_fired: existing_trigger.last_fired,
                                    next_fire: existing_trigger.next_fire,
                                    ..new_trigger
                                });
                            } else {
                                println!("Adding new trigger to in-memory store: {:?}", new_trigger);
                                new_triggers.insert(flow_version_id.to_string(), new_trigger);
                            }
                        } else {
                            println!("Found an action that's not a trigger.");
                        }
                    }
                }
            }
        }
    }

    let mut triggers = triggers.write().await;
    for (id, trigger) in new_triggers.into_iter() {
        triggers.insert(id, trigger);
    }
}


pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
    let now = Utc::now();

    println!("Current time: {}", now);
    if let Some(next_fire) = trigger.next_fire {
        println!("Next fire time: {}", next_fire);

        if now >= next_fire {
            println!("Trigger should run (now >= next_fire)");
            return true;
        }
    } else {
        println!("No next_fire time set, trigger should not run.");
    }

    println!("Trigger should not run");
    false
}

async fn update_trigger_last_run(trigger: &InMemoryTrigger, triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>) {
    println!("Updating trigger last run and next_run time");

    let new_next_fire = match Schedule::from_str(trigger.config["input"]["cron_expression"].as_str().unwrap()) {
        Ok(schedule) => schedule.upcoming(Utc).next(),
        Err(e) => {
            println!("Error parsing cron expression: {}", e);
            None
        }
    };

    println!("New next fire time: {:?}", new_next_fire);

    let updated_trigger = InMemoryTrigger {
        last_fired: Some(Utc::now()),
        next_fire: new_next_fire,
        ..trigger.clone()
    };

    // Update the trigger in memory
    let mut triggers = triggers.write().await;
    triggers.insert(trigger.flow_version_id.clone(), updated_trigger);
}

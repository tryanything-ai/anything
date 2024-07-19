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

// use cron::Schedule;
// use std::str::FromStr;

// use chrono::{DateTime, Utc};
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
    let triggers = Arc::new(RwLock::new(HashMap::new()));

    // Initial hydration of known cron triggers
    hydrate_triggers(&client, &triggers).await;

    // Refresh interval for checking the database
    let refresh_interval = Duration::from_secs(60);

    loop {
        // Check if any triggers should run
        {
            let triggers = triggers.read().await;
            for (id, trigger) in triggers.iter() {
                if should_trigger_run(trigger) {
                    // Execute the task associated with the trigger
                    println!("Triggering task for cron expression: {}", trigger.config);
                    // TODO: Execute the task by creating a trigger task
                    update_trigger_last_run(&client, trigger).await;
                }
            }
        }

        // Sleep for a short duration before the next check
        sleep(refresh_interval).await;

        // Periodically refresh triggers from the database
        hydrate_triggers(&client, &triggers).await;
    }
}

// Function to fetch and update the triggers in memory
pub async fn hydrate_triggers(client: &Postgrest, triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>) {
    println!("Hydrating triggers from the database");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .select("flow_id, flow_version_id, flow_definition") //TODO: only fetch active flows
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

    println!("Found of flow_versions vector: {}", flow_versions.len());

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
                            println!("Found trigger a action of type trigger");

                            let input = action.get("input").cloned().unwrap_or_default();
                            let variables = action.get("variables").cloned().unwrap_or_default();

                            let config = serde_json::json!({
                                "input": input,
                                "variables": variables,
                            });

                            println!("Creating trigger with config: {:?}", config);

                            let trigger = InMemoryTrigger {
                                trigger_id: trigger_id.to_string(),
                                flow_id: flow_id.to_string(),
                                flow_version_id: flow_version_id.to_string(),
                                config,
                                last_fired: None,
                                next_fire: None,
                            };
                            println!("Adding trigger to in-memory store: {:?}", trigger);
                            new_triggers.insert(trigger_id.to_string(), trigger);
                        } else {
                             println!("Found an action thats not a trigger.");
                        }
                    }
                }
            }
        }
    }

    let mut triggers = triggers.write().await;
    *triggers = new_triggers;
}

pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
    let now = Utc::now();
    
    let cron_expression = trigger.config["input"]["cron_expression"]
        .as_str()
        .unwrap_or("* * * * *");

    println!("Cron expression from config: {}", cron_expression);

    match Schedule::from_str(cron_expression) {
        Ok(schedule) => {
            let next_run_time = schedule.upcoming(Utc).next().unwrap();
            println!("Next run time: {}", next_run_time);

            if let Some(last_fired) = trigger.last_fired {
                now >= next_run_time && now.minute() != last_fired.minute()
            } else {
                now >= next_run_time
            }
        },
        Err(e) => {
            println!("Error parsing cron expression: {}", e);
            false // Do not run the trigger if cron expression is invalid
        }
    }
}

// pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
//     let now = Utc::now();

//     println!("Trigger config: {:?}", trigger.config);
//     println!("Input config: {:?}", trigger.config.get("input"));
//     println!("Cron expression from config: {:?}", trigger.config.get("input").and_then(|input| input.get("cron_expression")));

//     // let cron_expression = trigger.config.get("input")
//     //     .and_then(|input| input.get("cron_expression"))
//     //     .and_then(|v| v.as_str())
//     //     .unwrap_or("* * * * *"); 
//     let cron_expression = trigger.config["input"]["cron_expression"]
//         .as_str()
//         .unwrap_or("* * * * *");

//     println!("Cron expression from config: {:?}", cron_expression);
//     println!("Cron expression type: {}", std::any::type_name_of_val(&cron_expression));
//     println!("Cron expression length: {}", cron_expression.len());
//     println!("Cron expression bytes: {:?}", cron_expression.as_bytes());

//     // println!("Cron expression in should_trigger_run: {}", cron_expression);

//     let schedule = match Schedule::from_str(cron_expression) {
//         Ok(schedule) => schedule,
//         Err(e) => {
//             println!("Error parsing cron expression: {}", e);
//             return false; // Do not run the trigger if cron expression is invalid
//         }
//     };

//     let next_run_time = match schedule.upcoming(Utc).next() {
//         Some(time) => time,
//         None => {
//             println!("No upcoming run times found for cron expression: {}", cron_expression);
//             return false; // Do not run the trigger if there are no upcoming times
//         }
//     };

//     println!("Next run time: {}", next_run_time);

//     if let Some(last_fired) = trigger.last_fired {
//         now > next_run_time && now.minute() != last_fired.minute()
//     } else {
//         now > next_run_time
//     }
// }
// pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
//     let now = Utc::now();
//     let cron_expression = trigger.config.get("input")
//         .and_then(|input| input.get("cron_expression"))
//         .and_then(|v| v.as_str())
//         .unwrap_or("* * * * *");

//     println!("Cron expression in should_trigger_run: {}", cron_expression);

//     let next_run_time = Schedule::from_str(cron_expression)
//         .unwrap()
//         .upcoming(Utc)
//         .next()
//         .unwrap();

//     println!("Next run time: {}", next_run_time);

//     if let Some(last_fired) = trigger.last_fired {
//         now > next_run_time && now.minute() != last_fired.minute()
//     } else {
//         now > next_run_time
//     }
// }

// pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
//     let now = Utc::now();
//     let cron_expression = trigger.config.get("cron_expression").and_then(|v| v.as_str()).unwrap_or("* * * * *");
//     let next_run_time = Schedule::from_str(cron_expression)
//         .unwrap()
//         .upcoming(Utc)
//         .next()
//         .unwrap();

//     if let Some(last_fired) = trigger.last_fired {
//         now > next_run_time && now.minute() != last_fired.minute()
//     } else {
//         now > next_run_time
//     }
// }

pub async fn update_trigger_last_run(client: &Postgrest, trigger: &InMemoryTrigger) {
    let updated_trigger = InMemoryTrigger {
        last_fired: Some(Utc::now()),
        ..trigger.clone()
    };

    // let response = client
    //     .from("flow_versions")
    //     .eq("flow_version_id", updated_trigger.flow_version_id.clone())
    //     .update(serde_json::json!({ "last_fired": updated_trigger.last_fired }))
    //     .execute()
    //     .await;

    // if let Err(e) = response {
    //     println!("Failed to update trigger last fired time: {:?}", e);
    // }
}
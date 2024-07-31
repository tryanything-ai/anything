use tokio::time::{sleep, Duration};
use chrono::{Utc, DateTime};
use postgrest::Postgrest;

use dotenv::dotenv;
use std::env;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{task_types::{ActionType, FlowSessionStatus, Stage, TaskStatus, TriggerSessionStatus}, AppState};
use std::collections::HashMap;

use cron::Schedule;
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

use crate::workflow_types::{CreateTaskInput, TaskConfig};

pub struct TriggerEngineState {
    pub triggers: Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
}

#[derive(Debug, Clone)]
pub struct InMemoryTrigger {
    pub account_id: String,
    pub node_id: String,
    pub trigger_id: String,
    pub flow_id: String,
    pub action_label: String,
    pub flow_version_id: String,
    pub config: Value,
    pub last_fired: Option<DateTime<Utc>>,
    pub next_fire: Option<DateTime<Utc>>,
}

pub async fn cron_job_loop(state: Arc<AppState>) {
    let trigger_state = Arc::new(RwLock::new(HashMap::new()));

    let client = state.anything_client.clone();
    hydrate_triggers(&client, &trigger_state).await;

    let refresh_interval = Duration::from_secs(60);

    loop {
        println!("[TRIGGER_ENGINE] Starting trigger check loop");

        //find triggers to run
        let triggers_to_run = {
            let triggers = trigger_state.read().await;
            triggers
                .iter()
                .filter(|(_, trigger)| should_trigger_run(trigger))
                .map(|(id, trigger)| (id.clone(), trigger.clone()))
                .collect::<Vec<_>>()
        };

        //Create tasks for triggers that should run
        //Then udpate trigger to get next time to run in memory
        for (id, trigger) in triggers_to_run {
            println!("[TRIGGER_ENGINE] Trigger should run for trigger_id: {}", trigger.trigger_id);
            if let Err(e) = create_trigger_task(&state, &trigger).await {
                println!("[TRIGGER_ENGINE] Error creating trigger task: {:?}", e);
            } else {
                if let Err(e) = update_trigger_last_run(&id, &trigger, &trigger_state).await {
                    println!("[TRIGGER_ENGINE] Error updating trigger last run: {:?}", e);
                }
            }
            println!("[TRIGGER_ENGINE] Trigger Loop Successfully LOOPED");
        }

        println!("[TRIGGER_ENGINE] Sleeping for refresh interval");
        sleep(refresh_interval).await;

        println!("[TRIGGER_ENGINE] Hydrating triggers from the database");
        hydrate_triggers(&client, &trigger_state).await;
    }
}

pub async fn hydrate_triggers(client: &Postgrest, triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>) {
    println!("[TRIGGER_ENGINE] Hydrating triggers from the database");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY").expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .select("flow_id, flow_version_id, flow_definition, account_id") // TODO: only fetch active flows
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error fetching flow versions: {:?}", e);
            return;
        },
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error reading response body: {:?}", e);
            return;
        },
    };

    let flow_versions: Vec<Value> = match serde_json::from_str(&body) {
        Ok(flow_versions) => flow_versions,
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error parsing JSON: {:?}", e);
            return;
        },
    };

    println!("[TRIGGER_ENGINE] Found flow_versions vector: {}", flow_versions.len());

    let mut new_triggers = HashMap::new();

    for flow_version in flow_versions {
        if let (Some(flow_id), Some(flow_version_id), Some(flow_definition), Some(account_id)) = (
            flow_version.get("flow_id").and_then(|v| v.as_str()),
            flow_version.get("flow_version_id").and_then(|v| v.as_str()),
            flow_version.get("flow_definition"),
            flow_version.get("account_id").and_then(|v| v.as_str())
        ) {
            if let Some(actions) = flow_definition.get("actions").and_then(|v| v.as_array()) {
                for action in actions {
                    if let (Some(trigger_id), Some(action_type), Some(node_id)) = (
                        action.get("plugin_id").and_then(|v| v.as_str()),
                        action.get("type").and_then(|v| v.as_str()),
                        action.get("node_id").and_then(|v| v.as_str())
                    ) {
                        if action_type == "trigger" {
                            println!("[TRIGGER_ENGINE] Found trigger action of type trigger");

                            let input = action.get("input").cloned().unwrap_or_default();
                            let variables = action.get("variables").cloned().unwrap_or_default();

                            let config = serde_json::json!({
                                "input": input,
                                "variables": variables,
                            });

                            println!("[TRIGGER_ENGINE] Creating trigger with config: {:?}", config);

                            // Parse the cron expression and calculate the next fire time
                            let cron_expression = config["input"]["cron_expression"]
                                .as_str()
                                .unwrap_or("* * * * *");

                            let next_fire = match Schedule::from_str(cron_expression) {
                                Ok(schedule) => schedule.upcoming(Utc).next(),
                                Err(e) => {
                                    println!("[TRIGGER_ENGINE] Error parsing cron expression: {}", e);
                                    None
                                }
                            };

                            let new_trigger = InMemoryTrigger {
                                node_id: node_id.to_string(),
                                account_id: account_id.to_string(),
                                trigger_id: trigger_id.to_string(),
                                flow_id: flow_id.to_string(),
                                action_label: action.get("label").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                flow_version_id: flow_version_id.to_string(),
                                config,
                                last_fired: None,
                                next_fire,
                            };

                            // Check if the trigger already exists in memory
                            let existing_triggers = triggers.read().await;
                            if let Some(existing_trigger) = existing_triggers.get(flow_version_id) {
                                println!("[TRIGGER_ENGINE] Trigger already exists, preserving last_fired and next_fire values");
                                new_triggers.insert(flow_version_id.to_string(), InMemoryTrigger {
                                    last_fired: existing_trigger.last_fired,
                                    next_fire: existing_trigger.next_fire,
                                    ..new_trigger
                                });
                            } else {
                                println!("[TRIGGER_ENGINE] Adding new trigger to in-memory store: {:?}", new_trigger);
                                new_triggers.insert(flow_version_id.to_string(), new_trigger);
                            }
                        } else {
                            println!("[TRIGGER_ENGINE] Found an action that's not a trigger.");
                        }
                    }
                }
            }
        } else {
            println!("[TRIGGER_ENGINE] Missing required fields in flow_version: {:?}", flow_version);
        }
    }

    let mut triggers = triggers.write().await;
    for (id, trigger) in new_triggers.into_iter() {
        triggers.insert(id, trigger);
    }
}

pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
    let now = Utc::now();

    println!("[TRIGGER_ENGINE] Current time: {}", now);
    if let Some(next_fire) = trigger.next_fire {
        println!("[TRIGGER_ENGINE] Next fire time: {}", next_fire);

        if now >= next_fire {
            println!("[TRIGGER_ENGINE] Trigger should run (now >= next_fire)");
            return true;
        }
    } else {
        println!("[TRIGGER_ENGINE] No next_fire time set, trigger should not run.");
    }

    println!("[TRIGGER_ENGINE] Trigger should not run");
    false
}

async fn update_trigger_last_run(
    id: &str,
    trigger: &InMemoryTrigger,
    triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[TRIGGER_ENGINE] Updating trigger last run and next_run time");

    let new_next_fire = match Schedule::from_str(trigger.config["input"]["cron_expression"].as_str().unwrap()) {
        Ok(schedule) => schedule.upcoming(Utc).next(),
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error parsing cron expression: {}", e);
            None
        }
    };

    println!("[TRIGGER_ENGINE] New next fire time: {:?}", new_next_fire);

    let updated_trigger = InMemoryTrigger {
        last_fired: Some(Utc::now()),
        next_fire: new_next_fire,
        ..trigger.clone()
    };

    println!("[TRIGGER_ENGINE] Updated trigger: {:?}", updated_trigger);

    // Use a write lock to update the trigger
    let mut triggers = triggers.write().await;
    println!("[TRIGGER_ENGINE] Acquired write lock on triggers map");
    triggers.insert(id.to_string(), updated_trigger);
    println!("[TRIGGER_ENGINE] Successfully updated trigger last run and next_run time");

    Ok(())
}


async fn create_trigger_task(state: &AppState, trigger: &InMemoryTrigger) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;
    let client = &state.anything_client;

    println!("Handling create task from cron trigger");

    let task_config = TaskConfig {
        variables: trigger.config.get("variables").cloned().unwrap_or_default(),
        inputs: trigger.config.get("input").cloned().unwrap_or_default(),
    };

    let input = CreateTaskInput {
        account_id: trigger.account_id.clone(),
        task_status: TaskStatus::Pending.as_str().to_string(),
        flow_id: trigger.flow_id.clone(),
        flow_version_id: trigger.flow_version_id.clone(),
        action_label: trigger.action_label.clone(),
        trigger_id: trigger.trigger_id.clone(),
        trigger_session_id: Uuid::new_v4().to_string(),
        trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
        flow_session_id: Uuid::new_v4().to_string(),
        flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
        node_id: trigger.node_id.clone(),
        action_type: ActionType::Trigger, 
        plugin_id: trigger.trigger_id.clone(),
        stage: Stage::Production.as_str().to_string(),
        config: serde_json::json!(task_config),
        test_config: None,
        processing_order: 0
    };

    let response = client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(serde_json::to_string(&input)?)
        .execute()
        .await?;

    let body = response.text().await?;
    let _items: Value = serde_json::from_str(&body)?;

    if let Err(err) = state.task_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
    }

    println!("Successfully created trigger task");

    Ok(())
}
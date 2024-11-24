use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use tokio::time::{sleep, Duration};

use dotenv::dotenv;
use std::env;

use crate::{
    bundler::bundle_task_context,
    task_types::{ActionType, FlowSessionStatus, Stage, TaskStatus, TriggerSessionStatus},
    workflow_types::Task,
    AppState,
};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use cron::Schedule;
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

use crate::workflow_types::{CreateTaskInput, TaskConfig};

#[derive(Debug, Clone)]
pub struct InMemoryTrigger {
    pub account_id: String,
    pub action_id: String,
    pub trigger_id: String,
    pub flow_id: String,
    pub action_label: String,
    pub flow_version_id: String,
    pub config: Value,
    pub last_fired: Option<DateTime<Utc>>,
    pub next_fire: Option<DateTime<Utc>>,
}

pub async fn cron_job_loop(state: Arc<AppState>) {
    //worfklow_id => trigger
    let trigger_state: Arc<RwLock<HashMap<String, InMemoryTrigger>>> =
        Arc::new(RwLock::new(HashMap::new()));

    // Receive info from other systems
    let mut trigger_engine_signal_rx = state.trigger_engine_signal.subscribe();
    let client = state.anything_client.clone();
    hydrate_triggers(state.clone(), &client, &trigger_state).await;

    let refresh_interval = Duration::from_secs(60);

     // Clone state once here for use in the loop
     let state = Arc::new(state);

    loop {
        tokio::select! {
            _ = sleep(refresh_interval) => {
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
                //Then update trigger to get next time to run in memory
                for (id, trigger) in triggers_to_run {
                    println!(
                        "[TRIGGER_ENGINE] Trigger should run for trigger_id ie workflow_id: {}",
                        trigger.trigger_id
                    );
                    if let Err(e) = create_trigger_task(&state, &trigger).await {
                        println!("[TRIGGER_ENGINE] Error creating trigger task: {:?}", e);
                    } else {
                        if let Err(e) = update_trigger_last_run(&id, &trigger, &trigger_state).await {
                            println!("[TRIGGER_ENGINE] Error updating trigger last run: {:?}", e);
                        }
                    }
                    println!("[TRIGGER_ENGINE] Trigger Loop Successfully LOOPED");
                }

                println!("[TRIGGER_ENGINE] Finished trigger check loop");
            }
            _ = trigger_engine_signal_rx.changed() => {
                let workflow_id = trigger_engine_signal_rx.borrow().clone();
                println!("[TRIGGER_ENGINE] Received workflow_id: {}", workflow_id);
                if let Err(e) = update_triggers_for_workflow(&state, &client, &trigger_state, &workflow_id).await {
                    println!("[TRIGGER_ENGINE] Error updating triggers for workflow: {:?}", e);
                }
            }
        }
    }
}

//From Claude and very untested so far
//Ment to lightly update triggers so we don't need to refresh the entire memory each time we update something
async fn update_triggers_for_workflow(
    state: &Arc<AppState>,
    client: &Postgrest,
    triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
    workflow_id: &String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[TRIGGER_ENGINE] Updating triggers for workflow: {}",
        workflow_id
    );

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    //Get current published workflow version
    let response = client
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .select("flow_id, flow_version_id, flow_definition, account_id, flows!inner(active)") // TODO: only fetch active flows
        .eq("published", "true")
        .eq("flows.active", "true")
        .execute()
        .await?;

    let body = response.text().await?;
    let flow_versions: Vec<Value> = serde_json::from_str(&body)?;

    let mut new_triggers = HashMap::new();

    //Add new triggers to new_triggers
    for flow_version in flow_versions {
        let triggers_from_flow =
            create_in_memory_triggers_from_flow_definition(state.clone(), &flow_version, client)
                .await;
        new_triggers.extend(triggers_from_flow);
    }

    //Delete Existing trigger for workflow_id from hashmap
    //Can't just overwrite because the workflow update may have removed the trigger completely.
    let mut triggers = triggers.write().await;
    let old_value = triggers.remove(workflow_id);
    if let Some(old_trigger) = old_value {
        println!("[TRIGGER_ENGINE] Removing old trigger: {:?}", old_trigger);
    }
    //Write new riggers in memory for workflow_id
    // let mut triggers = triggers.write().await;
    for (id, trigger) in new_triggers.into_iter() {
        //Write New Trigger
        let old_value = triggers.insert(id, trigger);
        if let Some(old_trigger) = old_value {
            println!("[TRIGGER_ENGINE] Replaced old trigger: {:?}", old_trigger);
        } else {
            println!("[TRIGGER_ENGINE] Added new trigger");
        }
    }

    println!(
        "[TRIGGER_ENGINE] Successfully updated triggers for workflow: {}",
        workflow_id
    );
    Ok(())
}

pub async fn hydrate_triggers(
    state: Arc<AppState>,
    client: &Postgrest,
    triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
) {
    println!("[TRIGGER_ENGINE] Hydrating triggers from the database");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client //TODO: pagination for large number of triggers
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .select("flow_id, flow_version_id, flow_definition, account_id, flows!inner(active)") // TODO: only fetch active flows
        .eq("published", "true")
        .eq("flows.active", "true")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error fetching flow versions: {:?}", e);
            return;
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            // println!(
            //     "[TRIGGER_ENGINE] Response body for active and published triggers: {}",
            //     body
            // );
            body
        }
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error reading response body: {:?}", e);
            return;
        }
    };

    let flow_versions: Vec<Value> = match serde_json::from_str(&body) {
        Ok(flow_versions) => flow_versions,
        Err(e) => {
            println!("[TRIGGER_ENGINE] Error parsing JSON: {:?}", e);
            return;
        }
    };

    println!(
        "[TRIGGER_ENGINE] Found flow_versions vector: {}",
        flow_versions.len()
    );

    let mut new_triggers = HashMap::new();

    //Add new triggers to new_triggers
    for flow_version in flow_versions {
        let triggers_from_flow =
            create_in_memory_triggers_from_flow_definition(state.clone(), &flow_version, client)
                .await;

        for (workflow_id, new_trigger) in triggers_from_flow {
            // Check if the trigger already exists in memory
            let existing_triggers = triggers.read().await;
            if let Some(existing_trigger) = existing_triggers.get(&workflow_id) {
                println!("[TRIGGER_ENGINE] Trigger already exists, preserving last_fired and next_fire values");
                new_triggers.insert(
                    workflow_id.to_string(),
                    InMemoryTrigger {
                        last_fired: existing_trigger.last_fired,
                        next_fire: existing_trigger.next_fire,
                        ..new_trigger
                    },
                );
            } else {
                println!(
                    "[TRIGGER_ENGINE] Adding new trigger to in-memory store: {:?}",
                    new_trigger
                );
                new_triggers.insert(workflow_id.to_string(), new_trigger);
            }
        }
    }

    for (id, trigger) in new_triggers.iter() {
        println!(
            "[TRIGGER_ENGINE] New Trigger - ID: {}, Flow ID: {}, Next Fire: {:?}, Last Fired: {:?}",
            id, trigger.flow_id, trigger.next_fire, trigger.last_fired
        );
    }

    let mut triggers = triggers.write().await;
    for (id, trigger) in new_triggers.into_iter() {
        triggers.insert(id, trigger);
    }

    println!("[TRIGGER_ENGINE] Current triggers in memory:");
    for (id, trigger) in triggers.iter() {
        println!(
            "Trigger ID: {}, Flow ID: {}, Next Fire: {:?}, Last Fired: {:?}",
            id, trigger.flow_id, trigger.next_fire, trigger.last_fired
        );
    }

    println!("[TRIGGER_ENGINE] Successfully hydrated triggers from the database");
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

    let new_next_fire =
        match Schedule::from_str(trigger.config["input"]["cron_expression"].as_str().unwrap()) {
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

async fn create_trigger_task(
    state: &Arc<AppState>,
    trigger: &InMemoryTrigger,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;
    let client = state.anything_client.clone();

    println!("Handling create task from cron trigger");

    let task_config = TaskConfig {
        variables: trigger.config.get("variables").cloned().unwrap_or_default(),
        input: trigger.config.get("input").cloned().unwrap_or_default(),
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
        action_id: trigger.action_id.clone(),
        r#type: ActionType::Trigger,
        plugin_id: trigger.trigger_id.clone(),
        stage: Stage::Production.as_str().to_string(),
        config: serde_json::json!(task_config),
        result: None,
        test_config: None,
        processing_order: 0,
    };

    let response = client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(serde_json::to_string(&input)?)
        .execute()
        .await?;

    let body = response.text().await?;
    let _items: Value = serde_json::from_str(&body)?;

    // Send signal to task engine to process the new task
    if let Err(err) = state.task_engine_signal.send(()) {
        println!("Failed to send task signal: {:?}", err);
    }

    println!("Successfully created trigger task");

    Ok(())
}

pub async fn create_in_memory_triggers_from_flow_definition(
    state: Arc<AppState>,
    flow_version: &Value,
    client: &Postgrest,
) -> HashMap<String, InMemoryTrigger> {
    let mut triggers = HashMap::new();

    println!("[TRIGGER_ENGINE] Processing flow_version: {}", flow_version);
    if let (Some(flow_id), Some(flow_version_id), Some(flow_definition), Some(account_id)) = (
        flow_version.get("flow_id").and_then(|v| v.as_str()),
        flow_version.get("flow_version_id").and_then(|v| v.as_str()),
        flow_version.get("flow_definition"),
        flow_version.get("account_id").and_then(|v| v.as_str()),
    ) {
        if let Some(actions) = flow_definition.get("actions").and_then(|v| v.as_array()) {
            for action in actions {
                if let (Some(plugin_id), Some(r#type), Some(action_id)) = (
                    action.get("plugin_id").and_then(|v| v.as_str()),
                    action.get("type").and_then(|v| v.as_str()),
                    action.get("action_id").and_then(|v| v.as_str()),
                ) {
                    if r#type == "trigger" && plugin_id == "cron" {
                        println!(
                            "[TRIGGER ENGINE] Processing trigger action with ID: {}",
                            action_id
                        );
                        let input = action.get("input").cloned().unwrap_or_default();
                        let variables = action.get("variables").cloned().unwrap_or_default();

                        println!("[TRIGGER ENGINE] Trigger input: {:?}", input);
                        println!("[TRIGGER ENGINE] Trigger variables: {:?}", variables);

                        // let config = serde_json::json!({
                        //     "input": input,
                        //     "variables": variables,
                        // });

                        // println!("[TRIGGER ENGINE] Created config: {:?}", config);

                        // Parse action into Task structure
                        let mock_task = Task {
                            task_id: Uuid::new_v4(),
                            account_id: Uuid::parse_str(&account_id).unwrap_or_default(),
                            task_status: "pending".to_string(),
                            flow_id: Uuid::new_v4(),
                            flow_version_id: Uuid::new_v4(),
                            action_label: "fake".to_string(),
                            trigger_id: "".to_string(),
                            trigger_session_id: "".to_string(),
                            trigger_session_status: "".to_string(),
                            flow_session_id: "fake".to_string(),
                            flow_session_status: "fake".to_string(),
                            action_id: "fake".to_string(),
                            r#type: "fake".to_string(),
                            plugin_id: Some("fake".to_string()),
                            stage: "fake".to_string(),
                            test_config: None,
                            config: serde_json::json!({
                                "variables": variables,
                                "input": input,
                            }),
                            context: None,
                            started_at: None,
                            ended_at: None,
                            debug_result: None,
                            result: None,
                            archived: false,
                            updated_at: None,
                            created_at: None,
                            updated_by: None,
                            created_by: None,
                            processing_order: 0,
                        };

                        println!("[TRIGGER ENGINE] Created mock task: {:?}", mock_task);

                        //Run the templater over the variables and results from last session
                        //Return the templated variables and inputs
                        println!("[TRIGGER ENGINE] Attempting to bundle variables for trigger");
                        let rendered_input =
                            match bundle_task_context(state.clone(), client, &mock_task, false)
                                .await
                            {
                                Ok(vars) => {
                                    println!(
                                        "[TRIGGER ENGINE] Successfully bundled variables: {:?}",
                                        vars
                                    );
                                    vars
                                }
                                Err(e) => {
                                    println!(
                                        "[TRIGGER ENGINE] Failed to bundle variables: {:?}",
                                        e
                                    );
                                    continue;
                                }
                            };

                        let cron_expression = rendered_input["cron_expression"]
                            .as_str()
                            .unwrap_or("* * * * *");

                        println!(
                            "[TRIGGER ENGINE] Using cron expression: {}",
                            cron_expression
                        );

                        let next_fire = match Schedule::from_str(cron_expression) {
                            Ok(schedule) => {
                                let next = schedule.upcoming(Utc).next();
                                println!("[TRIGGER ENGINE] Calculated next fire time: {:?}", next);
                                next
                            }
                            Err(e) => {
                                println!("[TRIGGER_ENGINE] Error parsing cron expression: {}", e);
                                None
                            }
                        };

                        let trigger = InMemoryTrigger {
                            action_id: action_id.to_string(),
                            account_id: account_id.to_string(),
                            trigger_id: plugin_id.to_string(),
                            flow_id: flow_id.to_string(),
                            action_label: action
                                .get("label")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            flow_version_id: flow_version_id.to_string(),
                            config: serde_json::json!({
                                // "variables": rendered_variables,
                                "input": rendered_input,
                            }), //TODO: figure out how this is used and where to fix it
                            last_fired: None,
                            next_fire,
                        };

                        triggers.insert(flow_id.to_string(), trigger);
                    } else {
                        println!("[TRIGGER_ENGINE] Found an action that's not a cron trigger.");
                    }
                } else {
                    println!(
                        "[TRIGGER_ENGINE] Missing required fields in action: {:?}",
                        action
                    );
                }
            }
        } else {
            println!("[TRIGGER_ENGINE] No actions found in flow_definition");
        }
    } else {
        println!(
            "[TRIGGER_ENGINE] Missing required fields in flow_version: {:?}",
            flow_version
        );
    }

    triggers
}

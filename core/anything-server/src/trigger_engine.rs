use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use tokio::time::{sleep, Duration};

use dotenv::dotenv;
use std::env;

use node_semver::Version;
use serde_json::json;

use crate::{
    bundler::bundle_context_from_parts,
    metrics::METRICS,
    processor::processor::ProcessorMessage,
    types::{
        action_types::{ActionType, PluginName},
        task_types::{Stage, Task, TaskConfig},
        workflow_types::DatabaseFlowVersion,
    },
    AppState,
};

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use cron::Schedule;
use std::str::FromStr;
use tracing::{error, info, Span};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InMemoryTrigger {
    pub account_id: String,
    pub action_id: String,
    pub plugin_name: PluginName,
    pub plugin_version: Version,
    pub flow_id: String,
    pub action_label: String,
    pub flow_version_id: String,
    pub config: TaskConfig,
    pub last_fired: Option<DateTime<Utc>>,
    pub next_fire: Option<DateTime<Utc>>,
    pub cron_expression: String,
}

pub async fn cron_job_loop(state: Arc<AppState>) {
    //worfklow_id => trigger
    let trigger_state: Arc<RwLock<HashMap<String, InMemoryTrigger>>> =
        Arc::new(RwLock::new(HashMap::new()));

    // Receive info from other systems like CRUD over workflows that have triggers
    let mut trigger_engine_signal_rx = state.trigger_engine_signal.subscribe();
    let client = state.anything_client.clone();
    hydrate_triggers(state.clone(), &client, &trigger_state).await;

    //How often we check for triggers to run
    let refresh_interval = Duration::from_secs(60);

    // Clone state once here for use in the loop
    let state = Arc::new(state);

    loop {
        tokio::select! {
            _ = sleep(refresh_interval) => {
                // info!("[TRIGGER_ENGINE] Starting trigger check loop");

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
                let triggers_count = triggers_to_run.len();
                for (id, trigger) in triggers_to_run {
                    info!("[TRIGGER_ENGINE] Trigger should run for trigger_id ie workflow_id: {}",
                        trigger.plugin_name
                    );
                    if let Err(e) = create_trigger_task(&state, &trigger).await {
                        error!("[TRIGGER_ENGINE] Error creating trigger task: {:?}", e);
                    } else {
                        if let Err(e) = update_trigger_last_run(&id, &trigger, &trigger_state).await {
                            error!("[TRIGGER_ENGINE] Error updating trigger last run: {:?}", e);
                        }
                    }
                    info!("[TRIGGER_ENGINE] Trigger Loop Successfully LOOPED");
                }

                if triggers_count > 0 {
                    info!("[TRIGGER_ENGINE] Finished trigger check loop - executed {} triggers", triggers_count);
                } else {
                    info!("[TRIGGER_ENGINE] Finished trigger check loop - no triggers to execute");
                }
            }
            _ = trigger_engine_signal_rx.changed() => {
                let workflow_id = trigger_engine_signal_rx.borrow().clone();
                info!("[TRIGGER_ENGINE] Received workflow_id: {}", workflow_id);
                if let Err(e) = update_triggers_for_workflow(&state, &client, &trigger_state, &workflow_id).await {
                    error!("[TRIGGER_ENGINE] Error updating triggers for workflow: {:?}", e);
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
    let update_start = Instant::now();
    info!(
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
        .select("*, flows!inner(active)") // TODO: only fetch active flows
        .eq("published", "true")
        .eq("flows.active", "true")
        .execute()
        .await?;

    let body = response.text().await?;
    let flow_versions: Vec<DatabaseFlowVersion> = serde_json::from_str(&body)?;

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
        info!("[TRIGGER_ENGINE] Removing old trigger: {:?}", old_trigger);
        METRICS.triggers_active.add(-1, &[]);
    }
    //Write new riggers in memory for workflow_id
    // let mut triggers = triggers.write().await;
    for (id, trigger) in new_triggers.into_iter() {
        //Write New Trigger
        let old_value = triggers.insert(id, trigger);
        if let Some(old_trigger) = old_value {
            info!("[TRIGGER_ENGINE] Replaced old trigger: {:?}", old_trigger);
        } else {
            info!("[TRIGGER_ENGINE] Added new trigger");
            METRICS.triggers_active.add(1, &[]);
        }
    }

    // Record metrics
    let update_duration = update_start.elapsed();
    METRICS.trigger_updates_total.add(1, &[]);
    METRICS
        .trigger_update_duration
        .record(update_duration.as_secs_f64(), &[]);

    info!(
        "[TRIGGER_ENGINE] Successfully updated triggers for workflow: {} in {:?}",
        workflow_id, update_duration
    );
    Ok(())
}

pub async fn hydrate_triggers(
    state: Arc<AppState>,
    client: &Postgrest,
    triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
) {
    let hydration_start = Instant::now();
    info!("[TRIGGER_ENGINE] Hydrating triggers from the database");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = match client //TODO: pagination for large number of triggers
        .from("flow_versions")
        .auth(supabase_service_role_api_key.clone())
        .select("*, flows!inner(active)") // TODO: only fetch active flows
        .eq("published", "true")
        .eq("flows.active", "true")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("[TRIGGER_ENGINE] Error fetching flow versions: {:?}", e);
            METRICS.trigger_failures_total.add(1, &[]);
            return;
        }
    };

    let body = match response.text().await {
        Ok(body) => {
            // info!(
            //     "[TRIGGER_ENGINE] Response body for active and published triggers: {}",
            //     body
            // );
            body
        }
        Err(e) => {
            error!("[TRIGGER_ENGINE] Error reading response body: {:?}", e);
            METRICS.trigger_failures_total.add(1, &[]);
            return;
        }
    };

    let flow_versions: Vec<DatabaseFlowVersion> = match serde_json::from_str(&body) {
        Ok(flow_versions) => flow_versions,
        Err(e) => {
            error!("[TRIGGER_ENGINE] Error parsing JSON: {:?}", e);
            METRICS.trigger_failures_total.add(1, &[]);
            return;
        }
    };

    info!(
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
                info!("[TRIGGER_ENGINE] Trigger already exists, preserving last_fired and next_fire values");
                new_triggers.insert(
                    workflow_id.to_string(),
                    InMemoryTrigger {
                        last_fired: existing_trigger.last_fired,
                        next_fire: existing_trigger.next_fire,
                        ..new_trigger
                    },
                );
            } else {
                info!(
                    "[TRIGGER_ENGINE] Adding new trigger to in-memory store: {:?}",
                    new_trigger
                );
                new_triggers.insert(workflow_id.to_string(), new_trigger);
            }
        }
    }

    for (id, trigger) in new_triggers.iter() {
        info!(
            "[TRIGGER_ENGINE] New Trigger - ID: {}, Flow ID: {}, Next Fire: {:?}, Last Fired: {:?}",
            id, trigger.flow_id, trigger.next_fire, trigger.last_fired
        );
    }

    let triggers_count = new_triggers.len();
    let mut triggers = triggers.write().await;

    // Update the active triggers counter
    let old_count = triggers.len() as i64;
    let new_count = triggers_count as i64;
    METRICS.triggers_active.add(new_count - old_count, &[]);

    for (id, trigger) in new_triggers.into_iter() {
        triggers.insert(id, trigger);
    }

    info!("[TRIGGER_ENGINE] Current triggers in memory:");
    for (id, trigger) in triggers.iter() {
        info!(
            "Trigger ID: {}, Flow ID: {}, Next Fire: {:?}, Last Fired: {:?}",
            id, trigger.flow_id, trigger.next_fire, trigger.last_fired
        );
    }

    // Record metrics
    let hydration_duration = hydration_start.elapsed();
    METRICS
        .trigger_hydration_duration
        .record(hydration_duration.as_secs_f64(), &[]);
    METRICS
        .triggers_loaded_total
        .add(triggers_count as u64, &[]);

    info!(
        "[TRIGGER_ENGINE] Successfully hydrated {} triggers from the database in {:?}",
        triggers_count, hydration_duration
    );
}

pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
    let now = Utc::now();

    info!("[TRIGGER_ENGINE] Current time: {}", now);
    if let Some(next_fire) = trigger.next_fire {
        info!("[TRIGGER_ENGINE] Next fire time: {}", next_fire);

        if now >= next_fire {
            info!("[TRIGGER_ENGINE] Trigger should run (now >= next_fire)");
            return true;
        }
    } else {
        info!("[TRIGGER_ENGINE] No next_fire time set, trigger should not run.");
    }

    info!("[TRIGGER_ENGINE] Trigger should not run");
    false
}

async fn update_trigger_last_run(
    id: &str,
    trigger: &InMemoryTrigger,
    triggers: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("[TRIGGER_ENGINE] Updating trigger last run and next_run time");

    let new_next_fire = match Schedule::from_str(&trigger.cron_expression) {
        Ok(schedule) => schedule.upcoming(Utc).next(),
        Err(e) => {
            error!("[TRIGGER_ENGINE] Error parsing cron expression: {}", e);
            None
        }
    };

    info!("[TRIGGER_ENGINE] New next fire time: {:?}", new_next_fire);

    let updated_trigger = InMemoryTrigger {
        last_fired: Some(Utc::now()),
        next_fire: new_next_fire,
        ..trigger.clone()
    };

    info!("[TRIGGER_ENGINE] Updated trigger: {:?}", updated_trigger);

    // Use a write lock to update the trigger
    let mut triggers = triggers.write().await;
    info!("[TRIGGER_ENGINE] Acquired write lock on triggers map");
    triggers.insert(id.to_string(), updated_trigger);
    info!("[TRIGGER_ENGINE] Successfully updated trigger last run and next_run time");

    Ok(())
}

async fn create_trigger_task(
    state: &Arc<AppState>,
    trigger: &InMemoryTrigger,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let execution_start = Instant::now();
    let trigger_span = tracing::info_span!("create_trigger_task",
        flow_id = %trigger.flow_id,
        action_id = %trigger.action_id,
        task_id = tracing::field::Empty  // Declare but leave empty initially
    );
    let _entered = trigger_span.enter();
    info!("[CRON TRIGGER] Handling create task from cron trigger");

    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    info!("[WEBHOOK API] Fetching flow version from database");
    let response = match state
        .anything_client
        .from("flow_versions")
        .eq("flow_id", trigger.flow_id.clone())
        .eq("flow_version_id", trigger.flow_version_id.clone())
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            error!("[CRON TRIGGER] Failed to fetch flow version: {:?}", err);
            METRICS.trigger_failures_total.add(1, &[]);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to fetch flow version: {}", err),
            )));
        }
    };

    let response_body = match response.text().await {
        Ok(body) => {
            info!("[CRON TRIGGER] Response body: {}", body);
            body
        }
        Err(err) => {
            error!("[CRON TRIGGER] Failed to read response body: {:?}", err);
            METRICS.trigger_failures_total.add(1, &[]);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read response body: {}", err),
            )));
        }
    };

    let workflow_version: DatabaseFlowVersion = match serde_json::from_str(&response_body) {
        Ok(version) => version,
        Err(_) => {
            error!("[CRON TRIGGER] No published workflow found");
            METRICS.trigger_failures_total.add(1, &[]);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unpublished Workflow. To use this endpoint you must publish your workflow."
                ),
            )));
        }
    };

    let task = match Task::builder()
        .account_id(Uuid::parse_str(&trigger.account_id).unwrap())
        .flow_id(Uuid::parse_str(&trigger.flow_id).unwrap())
        .flow_version_id(Uuid::parse_str(&trigger.flow_version_id).unwrap())
        .action_label(trigger.action_label.clone())
        .trigger_id(trigger.action_id.clone())
        .action_id(trigger.action_id.clone())
        .r#type(ActionType::Trigger)
        .plugin_name(trigger.plugin_name.clone())
        .plugin_version(trigger.plugin_version.clone())
        .stage(Stage::Production)
        .config(trigger.config.clone())
        .result(json!({
            "message": format!("Successfully triggered task"),
                    "created_at": Utc::now()
        }))
        .build()
    {
        Ok(task) => task,
        Err(e) => {
            error!("[CRON TRIGGER] Failed to build task: {}", e);
            METRICS.trigger_failures_total.add(1, &[]);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to build task: {}", e),
            )));
        }
    };

    // Add task_id to the span by recording it
    Span::current().record("task_id", &tracing::field::display(&task.task_id));
    info!(
        "[CRON TRIGGER] Creating processor message for task: {}",
        task.task_id
    );
    // Send message to processor
    let processor_message = ProcessorMessage {
        workflow_id: Uuid::parse_str(&trigger.flow_id).unwrap(),
        workflow_version: workflow_version.clone(),
        workflow_definition: workflow_version.flow_definition.clone(),
        flow_session_id: task.flow_session_id.clone(),
        trigger_session_id: task.trigger_session_id.clone(),
        trigger_task: Some(task.clone()),
        task_id: Some(task.task_id),    // Include task_id for tracing
        existing_tasks: HashMap::new(), // No existing tasks for new workflows
        workflow_graph: crate::processor::utils::create_workflow_graph(
            &workflow_version.flow_definition,
        ),
    };

    if let Err(e) = state.processor_sender.send(processor_message).await {
        error!(
            "[TRIGGER_ENGINE] Failed to send message to processor: {}",
            e
        );
        METRICS.trigger_failures_total.add(1, &[]);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send message to processor: {}", e),
        )));
    }

    // Record successful execution
    let execution_duration = execution_start.elapsed();
    METRICS.trigger_executions_total.add(1, &[]);
    METRICS
        .trigger_execution_duration
        .record(execution_duration.as_secs_f64(), &[]);

    info!(
        "Successfully created trigger task in {:?}",
        execution_duration
    );

    Ok(())
}

pub async fn create_in_memory_triggers_from_flow_definition(
    state: Arc<AppState>,
    flow_version: &DatabaseFlowVersion,
    client: &Postgrest,
) -> HashMap<String, InMemoryTrigger> {
    let mut triggers = HashMap::new();

    // info!(
    //     "[TRIGGER_ENGINE] Processing flow_version: {:?}",
    //     flow_version
    // );
    let (flow_id, flow_version_id, flow_definition, account_id) = (
        flow_version.flow_id.to_string(),
        flow_version.flow_version_id.to_string(),
        flow_version.flow_definition.clone(),
        flow_version.account_id.to_string(),
    );

    let actions = flow_definition.actions;
    for action in actions {
        let (plugin_name, r#type, action_id) = (
            action.plugin_name.clone(),
            action.r#type.clone(),
            action.action_id.clone(),
        );
        if r#type == ActionType::Trigger
            && plugin_name == PluginName::new("@anything/cron".to_string()).unwrap()
        {
            info!(
                "[TRIGGER ENGINE] Processing trigger action with ID: {}",
                action_id
            );

            let inputs = action.inputs.clone();
            let inputs_schema = action.inputs_schema.clone();
            let plugin_config = action.plugin_config.clone();
            let plugin_config_schema = action.plugin_config_schema.clone();

            info!("[TRIGGER ENGINE] Trigger input: {:?}", inputs);
            info!("[TRIGGER ENGINE] Trigger input schema: {:?}", inputs_schema);

            let task_config: TaskConfig = TaskConfig {
                inputs: Some(inputs.clone().unwrap()),
                inputs_schema: Some(inputs_schema.clone().unwrap()),
                plugin_config: Some(plugin_config.clone()),
                plugin_config_schema: Some(plugin_config_schema.clone()),
            };

            //Run the templater over the variables and results from last session
            //Return the templated variables and inputs
            info!("[TRIGGER ENGINE] Attempting to bundle variables for trigger");
            let rendered_input = match bundle_context_from_parts(
                state.clone(),
                client,
                &account_id,
                &Uuid::new_v4().to_string(),
                Some(&inputs.clone().unwrap()),
                Some(&inputs_schema.clone().unwrap()),
                Some(&plugin_config.clone()),
                Some(&plugin_config_schema.clone()),
                false,
            )
            .await
            {
                Ok(vars) => {
                    info!(
                        "[TRIGGER ENGINE] Successfully bundled variables: {:?}",
                        vars
                    );
                    vars
                }
                Err(e) => {
                    error!("[TRIGGER ENGINE] Failed to bundle variables: {:?}", e);
                    continue;
                }
            };

            let cron_expression = rendered_input["cron_expression"]
                .as_str()
                .unwrap_or("* * * * *");

            info!(
                "[TRIGGER ENGINE] Using cron expression: {}",
                cron_expression
            );

            let next_fire = match Schedule::from_str(cron_expression) {
                Ok(schedule) => {
                    let next = schedule.upcoming(Utc).next();
                    info!("[TRIGGER ENGINE] Calculated next fire time: {:?}", next);
                    next
                }
                Err(e) => {
                    error!("[TRIGGER_ENGINE] Error parsing cron expression: {}", e);
                    None
                }
            };

            let trigger = InMemoryTrigger {
                action_id: action_id.to_string(),
                account_id: account_id.to_string(),
                plugin_name: plugin_name.clone(),
                plugin_version: action.plugin_version.clone(),
                flow_id: flow_id.to_string(),
                action_label: action.label.clone(),
                flow_version_id: flow_version_id.to_string(),
                config: task_config, // figure out how this is used and where to fix it
                last_fired: None,
                next_fire,
                cron_expression: cron_expression.to_string(),
            };

            triggers.insert(flow_id.to_string(), trigger);
        } else {
            info!("[TRIGGER_ENGINE] Found an action that's not a cron trigger.");
        }
    }

    triggers
}

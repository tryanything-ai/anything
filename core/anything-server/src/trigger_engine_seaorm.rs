use chrono::{DateTime, Utc};
use tokio::time::{sleep, Duration};

use dotenv::dotenv;
use std::env;

use node_semver::Version;
use serde_json::json;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Related, RelationTrait, QuerySelect, JoinType};

use crate::{
    bundler::bundle_context_from_parts,
    metrics::METRICS,
    processor::processor::ProcessorMessage,
    types::{
        action_types::{ActionType, PluginName},
        task_types::{Stage, Task, TaskConfig},
        workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
    },
    entities::{flow_versions, flows},
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
    println!("[TRIGGER ENGINE SEAORM] Starting cron job loop");
    //workflow_id => trigger
    let trigger_state: Arc<RwLock<HashMap<String, InMemoryTrigger>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let mut trigger_engine_signal_rx = state.trigger_engine_signal.subscribe();
    hydrate_triggers(state.clone(), &trigger_state).await;

    loop {
        let timeout_duration = Duration::from_secs(30);
        match tokio::time::timeout(timeout_duration, trigger_engine_signal_rx.recv()).await {
            Ok(Ok(signal)) => {
                info!("Received trigger engine signal: {}", signal);
                hydrate_triggers(state.clone(), &trigger_state).await;
            }
            Ok(Err(e)) => {
                error!("Error receiving trigger engine signal: {:?}", e);
                break;
            }
            Err(_) => {
                // Timeout occurred - this is normal, continue with trigger checking
            }
        }

        let now = Utc::now();
        let trigger_state_read = trigger_state.read().await;
        let triggers_to_fire: Vec<InMemoryTrigger> = trigger_state_read
            .values()
            .filter(|trigger| should_trigger_run(trigger))
            .cloned()
            .collect();
        drop(trigger_state_read);

        for trigger in triggers_to_fire {
            println!("[TRIGGER ENGINE SEAORM] Firing trigger: {}", trigger.action_id);
            let task_creation_start = Instant::now();
            if let Err(e) = create_trigger_task(state.clone(), &trigger).await {
                METRICS.trigger_failures_total.add(1, &[]);
                error!("Failed to create trigger task: {:?}", e);
            } else {
                METRICS
                    .trigger_executions_total
                    .add(1, &[]);
                METRICS
                    .trigger_execution_duration
                    .record(task_creation_start.elapsed().as_secs_f64(), &[]);
            }

            let update_start = Instant::now();
            if let Err(e) = update_trigger_last_run(state.clone(), &trigger, now).await {
                error!("Failed to update trigger last run: {:?}", e);
            } else {
                METRICS
                    .trigger_updates_total
                    .add(1, &[]);
                METRICS
                    .trigger_update_duration
                    .record(update_start.elapsed().as_secs_f64(), &[]);
            }

            let mut trigger_state_write = trigger_state.write().await;
            if let Some(stored_trigger) = trigger_state_write.get_mut(&trigger.flow_version_id) {
                stored_trigger.last_fired = Some(now);
                // Calculate next fire time
                if let Ok(schedule) = Schedule::from_str(&trigger.cron_expression) {
                    stored_trigger.next_fire = schedule.upcoming(Utc).take(1).next();
                }
            }
            drop(trigger_state_write);
        }

        // Sleep for 1 second before checking again
        sleep(Duration::from_secs(1)).await;
    }
}

async fn update_triggers_for_workflow(
    state: Arc<AppState>,
    flow_id: &str,
    trigger_state: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
) {
    println!("[TRIGGER ENGINE SEAORM] Updating triggers for workflow: {}", flow_id);
    
    let flow_uuid = match Uuid::parse_str(flow_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid flow ID format: {}", flow_id);
            return;
        }
    };

    let flow_versions = match flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowId.eq(flow_uuid))
        .filter(flow_versions::Column::Published.eq(true))
        .find_also_related(flows::Entity)
        .all(&*state.db)
        .await
    {
        Ok(versions) => versions,
        Err(err) => {
            error!("Database error fetching flow versions: {:?}", err);
            return;
        }
    };

    let mut trigger_state_write = trigger_state.write().await;
    
    for (version, flow_opt) in flow_versions {
        if let Some(flow) = flow_opt {
            if !flow.active {
                continue;
            }
            
            let definition: WorkflowVersionDefinition = match serde_json::from_value(version.flow_definition) {
                Ok(def) => def,
                Err(err) => {
                    error!("Failed to parse flow definition: {:?}", err);
                    continue;
                }
            };

            let triggers = create_in_memory_triggers_from_flow_definition(
                &definition,
                &version.account_id.to_string(),
                &version.flow_id.to_string(),
                &version.flow_version_id.to_string(),
            ).await;

            for trigger in triggers {
                trigger_state_write.insert(trigger.flow_version_id.clone(), trigger);
            }
        }
    }
    drop(trigger_state_write);
}

pub async fn hydrate_triggers(
    state: Arc<AppState>,
    trigger_state: &Arc<RwLock<HashMap<String, InMemoryTrigger>>>,
) {
    println!("[TRIGGER ENGINE SEAORM] Hydrating triggers from database");
    let hydration_start = Instant::now();

    // Get all active flow versions with their flows
    let flow_versions = match flow_versions::Entity::find()
        .filter(flow_versions::Column::Published.eq(true))
        .find_also_related(flows::Entity)
        .all(&*state.db)
        .await
    {
        Ok(versions) => versions,
        Err(err) => {
            error!("Database error fetching flow versions: {:?}", err);
            return;
        }
    };

    let mut triggers = HashMap::new();
    let mut total_triggers = 0;

    for (version, flow_opt) in flow_versions {
        if let Some(flow) = flow_opt {
            if !flow.active {
                continue;
            }
            
            let definition: WorkflowVersionDefinition = match serde_json::from_value(version.flow_definition) {
                Ok(def) => def,
                Err(err) => {
                    error!("Failed to parse flow definition: {:?}", err);
                    continue;
                }
            };

            let flow_triggers = create_in_memory_triggers_from_flow_definition(
                &definition,
                &version.account_id.to_string(),
                &version.flow_id.to_string(),
                &version.flow_version_id.to_string(),
            ).await;

            total_triggers += flow_triggers.len();

            for trigger in flow_triggers {
                triggers.insert(trigger.flow_version_id.clone(), trigger);
            }
        }
    }

    // Update the shared state
    {
        let mut trigger_state_write = trigger_state.write().await;
        *trigger_state_write = triggers;
    }

    METRICS
        .trigger_hydration_duration
        .record(hydration_start.elapsed().as_secs_f64(), &[]);
    METRICS
        .triggers_loaded_total
        .add(total_triggers as u64, &[]);
    METRICS
        .triggers_active
        .add(total_triggers as i64, &[]);

    info!("[TRIGGER ENGINE SEAORM] Hydrated {} triggers", total_triggers);
}

pub fn should_trigger_run(trigger: &InMemoryTrigger) -> bool {
    let now = Utc::now();

    // If we haven't calculated next_fire yet, do it now
    if trigger.next_fire.is_none() {
        if let Ok(schedule) = Schedule::from_str(&trigger.cron_expression) {
            if let Some(next) = schedule.upcoming(Utc).take(1).next() {
                return now >= next;
            }
        }
        return false;
    }

    // Check if it's time to fire
    if let Some(next_fire) = trigger.next_fire {
        now >= next_fire
    } else {
        false
    }
}

async fn update_trigger_last_run(
    state: Arc<AppState>,
    trigger: &InMemoryTrigger,
    last_run: DateTime<Utc>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[TRIGGER ENGINE SEAORM] Updating trigger last run: {}", trigger.action_id);
    
    // TODO: If we need to persist trigger state, we could create a triggers table
    // For now, we just update the in-memory state (handled in the main loop)
    
    Ok(())
}

async fn create_trigger_task(
    state: Arc<AppState>,
    trigger: &InMemoryTrigger,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[TRIGGER ENGINE SEAORM] Creating trigger task for: {}", trigger.action_id);
    
    let flow_uuid = Uuid::parse_str(&trigger.flow_id)?;
    let version_uuid = Uuid::parse_str(&trigger.flow_version_id)?;

    // Get the flow version from database
    let flow_version = flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowId.eq(flow_uuid))
        .filter(flow_versions::Column::FlowVersionId.eq(version_uuid))
        .one(&*state.db)
        .await?
        .ok_or("Flow version not found")?;

    let definition: WorkflowVersionDefinition = serde_json::from_value(flow_version.flow_definition)?;

    let flow_session_id = Uuid::new_v4();
    let trigger_session_id = Uuid::new_v4();

    let context = bundle_context_from_parts(
        trigger.action_id.clone(),
        trigger.config.clone(),
        json!({}),
        &definition,
    );

    let trigger_plugin_name = match &trigger.plugin_name {
        PluginName::CronTrigger => "cron_trigger",
        _ => "unknown_trigger",
    };

    let now = Utc::now();
    let task_id = Uuid::new_v4();

    let task = Task {
        task_id,
        account_id: Uuid::parse_str(&trigger.account_id)?,
        flow_id: flow_uuid,
        flow_version_id: version_uuid,
        flow_session_id,
        action_id: trigger.action_id.clone(),
        action_label: trigger.action_label.clone(),
        r#type: ActionType::Trigger,
        plugin_name: trigger.plugin_name.clone(),
        plugin_version: trigger.plugin_version.clone(),
        stage: Stage::Processing,
        config: trigger.config.clone(),
        output: None,
        context,
        error_message: None,
        retry_count: 0,
        max_retries: 0,
        trigger_session_id,
        trigger_session_status: "completed".to_string(),
        trigger_id: trigger.action_id.clone(),
        flow_session_status: "processing".to_string(),
        task_status: "processing".to_string(),
        parent_task_id: None,
        assigned_worker_id: None,
        started_at: Some(now),
        completed_at: None,
        created_at: now,
        updated_at: now,
        created_by: Some(Uuid::parse_str(&trigger.account_id)?),
        updated_by: Some(Uuid::parse_str(&trigger.account_id)?),
        execution_time_ms: None,
        current_step: Some(1),
        total_steps: Some(1),
        progress_percentage: Some(0.0),
    };

    // Send task to processor
    let processor_message = ProcessorMessage::ProcessTask(task);
    state
        .processor_sender
        .send(processor_message)
        .await
        .map_err(|_| "Failed to send task to processor")?;

    println!("[TRIGGER ENGINE SEAORM] Successfully created and sent trigger task");
    Ok(())
}

pub async fn create_in_memory_triggers_from_flow_definition(
    workflow_definition: &WorkflowVersionDefinition,
    account_id: &str,
    flow_id: &str,
    flow_version_id: &str,
) -> Vec<InMemoryTrigger> {
    let mut triggers = Vec::new();

    for action in &workflow_definition.actions {
        if let ActionType::Trigger = action.r#type {
            if action.plugin_name == PluginName::CronTrigger {
                if let Some(cron_expression) = action.config.get("cron_expression") {
                    if let Some(cron_str) = cron_expression.as_str() {
                        // Validate cron expression
                        if Schedule::from_str(cron_str).is_ok() {
                            let trigger = InMemoryTrigger {
                                account_id: account_id.to_string(),
                                action_id: action.action_id.clone(),
                                plugin_name: action.plugin_name.clone(),
                                plugin_version: action.plugin_version.clone(),
                                flow_id: flow_id.to_string(),
                                action_label: action.action_label.clone(),
                                flow_version_id: flow_version_id.to_string(),
                                config: action.config.clone(),
                                last_fired: None,
                                next_fire: None,
                                cron_expression: cron_str.to_string(),
                            };
                            triggers.push(trigger);
                        } else {
                            error!("Invalid cron expression: {}", cron_str);
                        }
                    }
                }
            }
        }
    }

    info!("[TRIGGER ENGINE SEAORM] Created {} triggers for flow {}", triggers.len(), flow_id);
    triggers
}

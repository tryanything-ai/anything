use std::sync::Arc;
use std::time::Instant;

use postgrest::Postgrest;

use crate::bundler::bundle_tasks_cached_context;
use crate::processor::process_trigger_utils::process_trigger_task;
use crate::system_plugins::formatter_actions::{
    date_formatter::process_date_task, text_formatter::process_text_task,
};
use crate::system_plugins::webhook_response::process_webhook_response_task;

use crate::system_plugins::agent_tool_trigger_response::process_tool_call_result_task;
use crate::system_plugins::filter::process_filter_task;
use crate::system_plugins::http::http_plugin::process_http_task;
use crate::system_plugins::javascript::process_js_task;
use crate::types::action_types::ActionType;
use crate::types::task_types::Task;
use crate::AppState;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use tracing::{error, info, instrument, Span};

#[derive(Debug, Clone)]
pub struct TaskError {
    pub error: Value,
    pub context: Value,
}

pub type TaskResult = Result<(Option<Value>, Value, DateTime<Utc>, DateTime<Utc>), TaskError>;

#[instrument(skip(state, client, task))]
pub async fn execute_task(state: Arc<AppState>, client: &Postgrest, task: &Task) -> TaskResult {
    let task_id = task.task_id;
    let flow_session_id = task.flow_session_id;
    let plugin_name = task.plugin_name.clone();
    let root_span = tracing::info_span!(
        "execute_task",
        task_id = %task_id,
        flow_session_id = %flow_session_id,
        plugin_name = ?plugin_name
    );
    let _root_entered = root_span.enter();
    let started_at = Utc::now();
    info!("[PROCESS TASK] Processing task {}", task.task_id);

    // Clone state before using it in join
    let state_clone = Arc::clone(&state);

    // Bundle context with results from cache
    let bundle_span = tracing::info_span!("bundle_tasks_cached_context");
    let bundle_start = Instant::now();
    let bundled_context_result: Result<(Value, Value), Box<dyn std::error::Error + Send + Sync>> =
        bundle_tasks_cached_context(state, client, task, true).await;
    let bundle_duration = bundle_start.elapsed();
    info!("[PROCESS TASK] Context bundling took {:?}", bundle_duration);

    let http_client = state_clone.http_client.clone();

    match bundled_context_result {
        Ok((bundled_inputs, bundled_plugin_cofig)) => {
            let plugin_span = tracing::info_span!("plugin_execution", plugin_name = ?plugin_name);
            let plugin_start = Instant::now();
            let task_result = if task.r#type == ActionType::Trigger {
                info!("[PROCESS TASK] Processing trigger task {}", task.task_id);
                process_trigger_task(task)
            } else {
                info!("[PROCESS TASK] Processing regular task {}", task.task_id);
                match &task.plugin_name {
                    Some(plugin_name) => {
                        let result = match plugin_name.as_str() {
                            "@anything/http" => {
                                process_http_task(&http_client, &bundled_plugin_cofig).await
                            }
                            "@anything/filter" => {
                                process_filter_task(&bundled_inputs, &bundled_plugin_cofig).await
                            }
                            "@anything/javascript" => {
                                process_js_task(&bundled_inputs, &bundled_plugin_cofig).await
                            }
                            "@anything/webhook_response" => {
                                process_webhook_response_task(
                                    state_clone,
                                    task.flow_session_id.clone(),
                                    &bundled_plugin_cofig,
                                )
                                .await
                            }
                            "@anything/agent_tool_call_response" => {
                                process_tool_call_result_task(
                                    state_clone,
                                    task.flow_session_id.clone(),
                                    &bundled_plugin_cofig,
                                )
                                .await
                            }
                            "@anything/format_text" => process_text_task(&bundled_plugin_cofig),
                            "@anything/format_date" => process_date_task(&bundled_plugin_cofig),
                            _ => process_missing_plugin(
                                plugin_name.as_str(),
                                &task.task_id.to_string(),
                            ),
                        };
                        let plugin_duration = plugin_start.elapsed();
                        info!(
                            "[SPEED] ExecuteTask::plugin_execution - {:?}",
                            plugin_duration
                        );
                        result
                    }
                    None => process_no_plugin_name(&task.task_id.to_string()),
                }
            };

            match task_result {
                Ok(result) => Ok((result, bundled_plugin_cofig, started_at, Utc::now())),
                Err(e) => {
                    error!("[PROCESS TASK] Plugin execution error: {}", e.to_string());
                    Err(TaskError {
                        error: json!({ "message": e.to_string() }),
                        context: bundled_plugin_cofig,
                    })
                }
            }
        }
        Err(e) => {
            error!("[ERROR] Failed to bundle task context: {}", e);
            // Create empty context since bundling failed
            let empty_context = json!({});
            Err(TaskError {
                error: json!({ "message": format!("Failed to bundle task context: {}", e) }),
                context: empty_context,
            })
        }
    }
}

pub fn process_missing_plugin(
    plugin_id: &str,
    task_id: &str,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Some(json!({
        "message": format!("Processed task {} :: plugin_id {} does not exist.", task_id, plugin_id)
    })))
}

pub fn process_no_plugin_name(
    task_id: &str,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Some(json!({
        "message": format!("Processed task {} :: no plugin_id found.", task_id)
    })))
}

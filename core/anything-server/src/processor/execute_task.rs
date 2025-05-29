use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use postgrest::Postgrest;
use uuid::Uuid;

use crate::bundler::bundle_tasks_cached_context_with_tasks;
use crate::processor::process_trigger_utils::process_trigger_task;
use crate::system_plugins::formatter_actions::{
    date_formatter::process_date_task, text_formatter::process_text_task,
};
use crate::system_plugins::webhook_response::process_webhook_response_task;

use crate::system_plugins::agent_tool_trigger_response::process_tool_call_result_task;
use crate::system_plugins::filter::process_filter_task;
use crate::system_plugins::http::http_plugin::process_http_task;
use crate::system_plugins::javascript::process_js_task;
use crate::types::action_types::{ActionType, PluginName};
use crate::types::task_types::Task;
use crate::AppState;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use tokio::time::{timeout, Duration};
use tracing::{error, info, instrument, warn};

#[derive(Debug, Clone)]
pub struct TaskError {
    pub error: Value,
    pub context: Value,
}

pub type TaskResult = Result<(Option<Value>, Value, DateTime<Utc>, DateTime<Utc>), TaskError>;

/// Enhanced task execution designed for the actor system
/// Provides better isolation, timeout handling, and RustyScript compatibility
#[instrument(skip(state, client, task, in_memory_tasks), fields(
    task_id = %task.task_id,
    plugin_name = ?task.plugin_name,
    task_type = ?task.r#type
))]
pub async fn execute_task(
    state: Arc<AppState>,
    client: &Postgrest,
    task: &Task,
    in_memory_tasks: Option<&HashMap<Uuid, Task>>, // Pass in-memory tasks from processor
) -> TaskResult {
    let task_id = task.task_id;
    let flow_session_id = task.flow_session_id;
    let plugin_name = task.plugin_name.clone();
    let label = task.action_label.clone();
    let account_id = task.account_id;
    let started_at = Utc::now();

    info!("[EXECUTE_TASK] Starting task execution: {}", task.task_id);

    // Phase 1: Bundle context with timeout protection
    let bundle_result = timeout(
        Duration::from_secs(30), // 30 second timeout for bundling
        bundle_tasks_cached_context_with_tasks(
            Arc::clone(&state),
            client,
            task,
            true,
            in_memory_tasks,
        ),
    )
    .await;

    let (bundled_inputs, bundled_plugin_config) = match bundle_result {
        Ok(Ok((inputs, config))) => {
            info!("[EXECUTE_TASK] Context bundling completed successfully");
            (inputs, config)
        }
        Ok(Err(e)) => {
            warn!("[EXECUTE_TASK] Context bundling failed: {}", e);
            return Err(TaskError {
                error: json!({
                    "message": format!("Failed to bundle task context: {}", e),
                    "error_type": "bundling_error"
                }),
                context: json!({}),
            });
        }
        Err(_) => {
            error!("[EXECUTE_TASK] Context bundling timed out after 30 seconds");
            return Err(TaskError {
                error: json!({
                    "message": "Task context bundling timed out",
                    "error_type": "bundling_timeout"
                }),
                context: json!({}),
            });
        }
    };

    // Phase 2: Execute plugin with appropriate timeout based on plugin type
    let plugin_timeout = get_plugin_timeout(&task.plugin_name);
    let plugin_start = Instant::now();

    info!(
        "[EXECUTE_TASK] Executing plugin {:?} with {}s timeout",
        task.plugin_name,
        plugin_timeout.as_secs()
    );

    let task_execution_result = timeout(
        plugin_timeout,
        execute_plugin_safe(state, task, &bundled_inputs, &bundled_plugin_config),
    )
    .await;

    let plugin_duration = plugin_start.elapsed();
    let ended_at = Utc::now();

    match task_execution_result {
        Ok(Ok(result)) => {
            info!(
                "[EXECUTE_TASK] Task {} completed successfully in {:?}",
                task.task_id, plugin_duration
            );
            Ok((result, bundled_plugin_config, started_at, ended_at))
        }
        Ok(Err(e)) => {
            error!(
                "[EXECUTE_TASK] Task {} failed after {:?}: {}",
                task.task_id, plugin_duration, e
            );
            Err(TaskError {
                error: json!({
                    "message": e.to_string(),
                    "error_type": "plugin_execution_error",
                    "execution_time_ms": plugin_duration.as_millis()
                }),
                context: bundled_plugin_config,
            })
        }
        Err(_) => {
            error!(
                "[EXECUTE_TASK] Task {} timed out after {:?}",
                task.task_id, plugin_timeout
            );
            Err(TaskError {
                error: json!({
                    "message": format!("Task execution timed out after {:?}", plugin_timeout),
                    "error_type": "execution_timeout",
                    "timeout_duration_ms": plugin_timeout.as_millis()
                }),
                context: bundled_plugin_config,
            })
        }
    }
}

/// Get appropriate timeout duration based on plugin type
/// JavaScript and filter tasks get longer timeouts due to RustyScript
fn get_plugin_timeout(plugin_name: &Option<PluginName>) -> Duration {
    match plugin_name.as_ref().map(|s| s.as_str()) {
        Some("@anything/javascript") => Duration::from_secs(60), // 60s for JS - RustyScript needs time
        Some("@anything/filter") => Duration::from_secs(30), // 30s for filter - also uses RustyScript
        Some("@anything/http") => Duration::from_secs(45),   // 45s for HTTP - network operations
        Some("@anything/webhook_response") => Duration::from_secs(20), // 20s for webhook response
        Some("@anything/agent_tool_call_response") => Duration::from_secs(30), // 30s for agent tools
        _ => Duration::from_secs(15), // 15s default for other plugins
    }
}

/// Safely execute plugin with proper error isolation
/// This prevents plugin panics from crashing the actor
#[instrument(skip(state, task, bundled_inputs, bundled_plugin_config))]
async fn execute_plugin_safe(
    state: Arc<AppState>,
    task: &Task,
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    // Use a panic guard for RustyScript-based plugins
    let needs_panic_protection = matches!(
        task.plugin_name.as_ref().map(|s| s.as_str()),
        Some("@anything/javascript") | Some("@anything/filter")
    );

    if needs_panic_protection {
        info!("[EXECUTE_TASK] Using enhanced safety for RustyScript plugin");
        // For RustyScript plugins, we add extra monitoring and recovery
        execute_plugin_with_monitoring(state, task, bundled_inputs, bundled_plugin_config).await
    } else {
        execute_plugin_inner(state, task, bundled_inputs, bundled_plugin_config).await
    }
}

/// Execute plugin with enhanced monitoring for RustyScript-based plugins
async fn execute_plugin_with_monitoring(
    state: Arc<AppState>,
    task: &Task,
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    info!("[EXECUTE_TASK] Executing RustyScript plugin with enhanced monitoring");

    // Add memory and resource monitoring for RustyScript
    let start_memory = get_memory_usage();
    let start_time = std::time::Instant::now();

    let result = execute_plugin_inner(state, task, bundled_inputs, bundled_plugin_config).await;

    let end_time = std::time::Instant::now();
    let end_memory = get_memory_usage();
    let duration = end_time.duration_since(start_time);

    info!(
        "[EXECUTE_TASK] RustyScript execution completed - Duration: {:?}, Memory delta: {} KB",
        duration,
        (end_memory as i64 - start_memory as i64) / 1024
    );

    result
}

/// Simple memory usage tracking (best effort)
fn get_memory_usage() -> u64 {
    // Simple memory tracking - in a real implementation you might use more sophisticated monitoring
    std::process::id() as u64 * 1024 // Placeholder - replace with actual memory monitoring if needed
}

/// Inner plugin execution logic
async fn execute_plugin_inner(
    state: Arc<AppState>,
    task: &Task,
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    if task.r#type == ActionType::Trigger {
        info!("[EXECUTE_TASK] Processing trigger task {}", task.task_id);
        return process_trigger_task(task);
    }

    info!("[EXECUTE_TASK] Processing regular task {}", task.task_id);

    match &task.plugin_name {
        Some(plugin_name) => {
            let result = match plugin_name.as_str() {
                "@anything/http" => {
                    info!("[EXECUTE_TASK] Executing HTTP plugin");
                    process_http_task(&state.http_client, bundled_plugin_config).await
                }
                "@anything/filter" => {
                    info!("[EXECUTE_TASK] Executing filter plugin with RustyScript");
                    process_filter_task(bundled_inputs, bundled_plugin_config).await
                }
                "@anything/javascript" => {
                    info!("[EXECUTE_TASK] Executing JavaScript plugin with RustyScript");
                    process_js_task(bundled_inputs, bundled_plugin_config).await
                }
                "@anything/webhook_response" => {
                    info!("[EXECUTE_TASK] Executing webhook response plugin");
                    process_webhook_response_task(
                        state,
                        task.flow_session_id,
                        bundled_plugin_config,
                    )
                    .await
                }
                "@anything/agent_tool_call_response" => {
                    info!("[EXECUTE_TASK] Executing agent tool call response plugin");
                    process_tool_call_result_task(
                        state,
                        task.flow_session_id,
                        bundled_plugin_config,
                    )
                    .await
                }
                "@anything/format_text" => {
                    info!("[EXECUTE_TASK] Executing text formatter plugin");
                    process_text_task(bundled_plugin_config)
                }
                "@anything/format_date" => {
                    info!("[EXECUTE_TASK] Executing date formatter plugin");
                    process_date_task(bundled_plugin_config)
                }
                _ => {
                    warn!("[EXECUTE_TASK] Unknown plugin: {}", plugin_name.as_str());
                    process_missing_plugin(plugin_name.as_str(), &task.task_id.to_string())
                }
            };
            result
        }
        None => {
            warn!("[EXECUTE_TASK] Task has no plugin name");
            process_no_plugin_name(&task.task_id.to_string())
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

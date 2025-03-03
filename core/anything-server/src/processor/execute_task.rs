use std::sync::Arc;

use postgrest::Postgrest;

use crate::bundler::bundle_tasks_cached_context;
use crate::processor::process_trigger_utils::process_trigger_task;
use crate::system_plugins::formatter_actions::{
    date_formatter::process_date_task, text_formatter::process_text_task,
};
use crate::system_plugins::webhook_response::process_webhook_response_task;

use crate::system_plugins::agent_tool_trigger_response::process_tool_call_result_task;
use crate::system_plugins::calls::outbound_call::process_outbound_call_task;
use crate::system_plugins::http::http_plugin::process_http_task;
use crate::system_plugins::javascript::process_js_task;
use crate::types::task_types::Task;
use crate::AppState;
use serde_json::{json, Value};

use crate::types::action_types::ActionType;

#[derive(Debug, Clone)]
pub struct TaskError {
    pub error: Value,
    pub context: Value,
}

pub type TaskResult = Result<(Option<Value>, Value), TaskError>;

pub async fn execute_task(state: Arc<AppState>, client: &Postgrest, task: &Task) -> TaskResult {
    println!("[PROCESS TASK] Processing task {}", task.task_id);

    // Clone state before using it in join
    let state_clone = Arc::clone(&state);

    // Bundle context with results from cache
    let bundled_context_result: Result<(Value, Value), Box<dyn std::error::Error + Send + Sync>> =
        bundle_tasks_cached_context(state, client, task, true).await;

    let http_client = state_clone.http_client.clone();

    match bundled_context_result {
        Ok((bundled_inputs, bundled_plugin_cofig)) => {
            let task_result = if task.r#type == ActionType::Trigger.as_str().to_string() {
                println!("[PROCESS TASK] Processing trigger task {}", task.task_id);
                process_trigger_task(task)
            } else {
                println!("[PROCESS TASK] Processing regular task {}", task.task_id);
                match &task.plugin_name {
                    Some(plugin_name) => match plugin_name.as_str() {
                        "@anything/http" => {
                            process_http_task(&http_client, &bundled_plugin_cofig).await
                        }
                        //JS need bundled variables because variables are injected into the JS runtime vs tempalted into the string like we do other places.
                        //Honestly not sure this is required vs templating the text but it feels safer even if this adds a anit pattern to task processing for JS.
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
                        "@anything/outbound_call" => {
                            process_outbound_call_task(state_clone, &bundled_plugin_cofig).await
                        }
                        "@anything/format_text" => process_text_task(&bundled_plugin_cofig),
                        "@anything/format_date" => process_date_task(&bundled_plugin_cofig),
                        _ => {
                            process_missing_plugin(plugin_name.as_str(), &task.task_id.to_string())
                        }
                    },
                    None => process_no_plugin_name(&task.task_id.to_string()),
                }
            };

            match task_result {
                Ok(result) => Ok((result, bundled_plugin_cofig)),
                Err(e) => Err(TaskError {
                    error: json!({ "message": e.to_string() }),
                    context: bundled_plugin_cofig,
                }),
            }
        }
        Err(e) => {
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
    Err(format!(
        "Failed to process task {}: plugin_id {} does not exist",
        task_id, plugin_id
    )
    .into())
}

pub fn process_no_plugin_name(
    task_id: &str,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    Err(format!("Failed to process task {}: no plugin_id specified", task_id).into())
}

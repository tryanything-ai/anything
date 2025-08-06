use serde_json::Value;
use std::time::Instant;
use tracing::{error, info, instrument};
use uuid::Uuid;

// Import the JavaScript executor functionality
use crate::system_plugins::javascript::{execute_javascript_grpc, JsExecutorManager};

/// Enhanced filter task processor using gRPC JavaScript executor
/// This is used for conditional logic and boolean expressions
/// Now uses the same gRPC JavaScript executor as the main JavaScript plugin
#[instrument(skip(bundled_inputs, bundled_plugin_config))]
pub async fn process_filter_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    info!("[FILTER] Starting filter task processing");

    // Extract condition code
    let js_code = match bundled_plugin_config["condition"].as_str() {
        Some(code) => {
            info!("[FILTER] Extracted condition code: {:?}", code);
            code
        }
        None => {
            error!("[FILTER] No condition code found in configuration");
            return Err("Filter condition not found in task configuration".into());
        }
    };

    // Execute filter condition using the gRPC JavaScript executor
    let result = match execute_filter_condition_grpc(js_code, bundled_inputs).await {
        Ok(result) => result,
        Err(e) => {
            error!("[FILTER] Filter execution failed: {}", e);
            // When filter execution fails, treat it as filter not passing (return null)
            // This prevents the filter from "always passing" due to errors
            let total_duration = start.elapsed();
            info!(
                "[FILTER] Filter task failed in {:?}, treating as filter not passed",
                total_duration
            );
            return Ok(Some(Value::Null));
        }
    };

    let total_duration = start.elapsed();
    info!("[FILTER] Filter task completed in {:?}", total_duration);

    Ok(Some(result))
}

/// Execute filter condition using gRPC JavaScript executor
async fn execute_filter_condition_grpc(
    js_code: &str,
    inputs: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    info!("[FILTER] Executing filter condition via gRPC JavaScript executor");

    // Create gRPC client connection
    let mut js_manager = JsExecutorManager::new().await?;
    let js_client = js_manager.get_client().await;

    // Execute via gRPC - pass inputs as separate parameter, not embedded in code
    let execution_id = Uuid::new_v4().to_string();
    let result = execute_javascript_grpc(js_client, js_code, inputs, &execution_id).await?;

    // Log the actual result returned from JavaScript execution
    info!("[FILTER] JavaScript execution returned: {:?}", result);

    // Process the boolean result from JavaScript service
    process_filter_result(result, inputs)
}

/// Process the filter result (boolean) returned by the JavaScript service and return appropriate data
fn process_filter_result(
    result: Value,
    original_inputs: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    info!("[FILTER] Processing filter result: {:?}", result);

    // Filter should ONLY pass for explicit boolean true
    // Any other value (including truthy values like "hello", 1, etc.) should fail
    match result {
        Value::Bool(true) => {
            info!("[FILTER] Filter condition passed (returned true), returning original inputs");
            Ok(original_inputs.clone())
        }
        _ => {
            info!(
                "[FILTER] Filter condition failed (did not return true: {:?}), returning null",
                result
            );
            Ok(Value::Null)
        }
    }
}

/// Utility function to check if a JSON Value is truthy using JavaScript truthiness rules
/// This is available for other parts of the system that may need JavaScript truthiness evaluation
#[allow(unused)]
pub fn is_value_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::String(s) => !s.is_empty() && s != "false" && s != "0",
        Value::Number(n) => {
            let val = n.as_f64().unwrap_or(0.0);
            val != 0.0 && !val.is_nan()
        }
        Value::Array(arr) => !arr.is_empty(),
        Value::Object(obj) => !obj.is_empty(),
        Value::Null => false,
    }
}

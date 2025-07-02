use serde_json::{json, Value};
use std::time::Instant;
use tracing::{error, info, instrument};

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
    let result = execute_filter_condition_grpc(js_code, bundled_inputs).await?;

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

    // Determine if this is a simple expression or function-style condition
    let is_simple_expression = !js_code.trim_start().starts_with("function")
        && !js_code.contains("return")
        && !js_code.contains("{");

    // Create wrapped filter code that returns a proper result
    let wrapped_code = create_wrapped_filter_code(js_code, inputs, is_simple_expression)?;

    // Create gRPC client connection
    let mut js_manager = JsExecutorManager::new().await?;
    let js_client = js_manager.get_client().await;

    // Execute via gRPC using a simple inputs object since the code is already wrapped
    let simple_inputs = json!({});
    let result =
        execute_javascript_grpc(js_client, &wrapped_code, &simple_inputs, "filter").await?;

    // Process the filter result
    process_filter_result(result, inputs)
}

/// Create properly wrapped filter condition code
fn create_wrapped_filter_code(
    js_code: &str,
    inputs: &Value,
    is_simple_expression: bool,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let inputs_json = serde_json::to_string(inputs)?;

    let wrapped_code = if is_simple_expression {
        format!(
            r#"
            // Enhanced filter wrapper for simple expressions
            const inputs = {inputs_json};

            try {{
                const result = {js_code};
                
                // Ensure we got a value
                if (result === undefined) {{
                    return {{ 
                        error: 'Filter expression returned undefined. Please ensure your expression evaluates to a boolean value.',
                        actual_value: 'undefined'
                    }};
                }}

                // If result is a boolean, use it directly
                if (typeof result === 'boolean') {{
                    return {{ success: true, result, passed: result }};
                }}
                
                // If result is a string "true" or "false", convert it
                if (typeof result === 'string' && (result.toLowerCase() === 'true' || result.toLowerCase() === 'false')) {{
                    const boolResult = result.toLowerCase() === 'true';
                    return {{ success: true, result: boolResult, passed: boolResult }};
                }}
                
                // Truthy/falsy conversion for other types
                const boolResult = Boolean(result);
                return {{ success: true, result: boolResult, passed: boolResult }};
                
            }} catch (error) {{
                return {{ 
                    error: `Filter expression error: ${{error.message}}`,
                    error_type: error.name || 'Error',
                    error_stack: error.stack || 'No stack trace available'
                }};
            }}
            "#
        )
    } else {
        format!(
            r#"
            // Enhanced filter wrapper for function-style conditions
            const inputs = {inputs_json};

            try {{
                const result = (() => {{
                    {js_code}
                }})();
                
                if (result === undefined) {{
                    return {{ 
                        error: 'Filter function must return a value. Add a return statement to your condition.',
                        actual_value: 'undefined'
                    }};
                }}

                // Convert to boolean
                if (typeof result === 'boolean') {{
                    return {{ success: true, result, passed: result }};
                }}
                
                const boolResult = Boolean(result);
                return {{ success: true, result: boolResult, passed: boolResult }};
                
            }} catch (error) {{
                return {{ 
                    error: `Filter function error: ${{error.message}}`,
                    error_type: error.name || 'Error',
                    error_stack: error.stack || 'No stack trace available'
                }};
            }}
            "#
        )
    };

    info!(
        "[FILTER] Generated wrapped condition code, length: {} chars",
        wrapped_code.len()
    );

    Ok(wrapped_code)
}

/// Process the filter result and return appropriate data
fn process_filter_result(
    result: Value,
    original_inputs: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    // Check for errors first
    if let Some(error) = result.get("error") {
        if let Some(error_msg) = error.as_str() {
            error!("[FILTER] Filter condition error: {}", error_msg);
            return Err(error_msg.into());
        }
    }

    // Check for success
    if let Some(success) = result.get("success") {
        if success.as_bool() == Some(true) {
            if let Some(passed) = result.get("passed") {
                if passed.as_bool() == Some(true) {
                    info!("[FILTER] Condition passed, returning original inputs");
                    return Ok(original_inputs.clone());
                } else {
                    info!("[FILTER] Condition failed, returning null");
                    return Ok(Value::Null);
                }
            }
        }
    }

    // Fallback: try to interpret the result directly
    if let Some(passed) = result.as_bool() {
        if passed {
            info!("[FILTER] Direct boolean result: passed, returning original inputs");
            Ok(original_inputs.clone())
        } else {
            info!("[FILTER] Direct boolean result: failed, returning null");
            Ok(Value::Null)
        }
    } else {
        error!("[FILTER] Unexpected filter result format: {:?}", result);
        Err("Unexpected filter result format".into())
    }
}

use rustyscript::worker::{DefaultWorker, DefaultWorkerOptions};
use serde_json::Value;
use std::time::Duration;
use tokio::time::Instant;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

/// Enhanced JavaScript task processor optimized for the actor system
/// Uses RustyScript workers for safe JavaScript execution
#[instrument(skip(bundled_inputs, bundled_plugin_config))]
pub async fn process_js_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    info!("[RUSTYSCRIPT] Starting JavaScript task execution");

    // Extract JavaScript code
    let js_code = match bundled_plugin_config["code"].as_str() {
        Some(code) => {
            info!(
                "[RUSTYSCRIPT] Extracted JS code, length: {} chars",
                code.len()
            );
            code
        }
        None => {
            error!("[RUSTYSCRIPT] No JavaScript code found in configuration");
            return Err("JavaScript code not found in task configuration".into());
        }
    };

    // Prepare execution context
    let input_size = serde_json::to_string(bundled_inputs)
        .map(|s| s.len())
        .unwrap_or(0);

    info!("[RUSTYSCRIPT] Input data size: {} bytes", input_size);

    // Execute JavaScript in a controlled manner using RustyScript workers
    let result = execute_javascript_safe(js_code, bundled_inputs).await?;

    let total_duration = start.elapsed();
    info!(
        "[RUSTYSCRIPT] JavaScript task completed successfully in {:?}",
        total_duration
    );

    Ok(Some(result))
}

/// Safe JavaScript execution with RustyScript workers and proper error handling
async fn execute_javascript_safe(
    js_code: &str,
    inputs: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    info!("[RUSTYSCRIPT] Preparing JavaScript execution environment");

    // Create wrapped code with better error handling
    let wrapped_code = create_wrapped_javascript(js_code, inputs)?;

    info!("[RUSTYSCRIPT] Creating RustyScript worker for JavaScript execution");

    // Execute with appropriate timeout for actor system
    let execution_start = Instant::now();
    info!("[RUSTYSCRIPT] Starting script execution with 30 second timeout");

    // Add retry logic and better error handling
    let max_retries = 2;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        if attempt > 0 {
            warn!(
                "[RUSTYSCRIPT] Retrying execution, attempt {}/{}",
                attempt + 1,
                max_retries + 1
            );
            // Small delay between retries
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let wrapped_code_for_attempt = wrapped_code.clone();

        let execution_result = tokio::task::spawn_blocking(move || {
            // Create worker inside the blocking task
            let worker = match DefaultWorker::new(DefaultWorkerOptions {
                default_entrypoint: None,
                timeout: Duration::from_secs(25), // Slightly less than outer timeout
                startup_snapshot: None,
                shared_array_buffer_store: None,
            }) {
                Ok(worker) => worker,
                Err(e) => return Err(format!("Failed to create RustyScript worker: {}", e)),
            };

            // Execute with panic catching
            let result: Result<Result<Value, rustyscript::Error>, Box<dyn std::any::Any + Send>> =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    worker.eval::<Value>(wrapped_code_for_attempt)
                }));

            match result {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(e)) => Err(format!("RustyScript error: {}", e)),
                Err(panic) => {
                    let panic_msg = if let Some(s) = panic.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = panic.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "Unknown panic".to_string()
                    };
                    Err(format!("RustyScript panicked: {}", panic_msg))
                }
            }
        })
        .await;

        match execution_result {
            Ok(Ok(result)) => {
                let execution_duration = execution_start.elapsed();
                info!(
                    "[RUSTYSCRIPT] Script executed successfully in {:?}",
                    execution_duration
                );

                // Check for internal error markers
                if let Some(error) = result.get("internal_error") {
                    if let Some(error_msg) = error.as_str() {
                        error!("[RUSTYSCRIPT] JavaScript internal error: {}", error_msg);
                        last_error = Some(error_msg.to_string());
                        continue; // Retry on internal errors
                    }
                }

                log_result_info(&result);
                return Ok(result);
            }
            Ok(Err(e)) => {
                error!("[RUSTYSCRIPT] Execution error: {}", e);
                last_error = Some(e);
                continue; // Retry
            }
            Err(join_error) => {
                error!("[RUSTYSCRIPT] Task join error: {}", join_error);

                // Check if it's a panic
                if join_error.is_panic() {
                    let panic_info = join_error.into_panic();
                    let panic_msg = if let Some(s) = panic_info.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "Unknown panic".to_string()
                    };
                    error!("[RUSTYSCRIPT] Task panicked: {}", panic_msg);
                    last_error = Some(format!("Task panicked: {}", panic_msg));
                } else {
                    last_error = Some("Task was cancelled".to_string());
                }
                continue; // Retry
            }
        }
    }

    // All retries failed
    let final_error = last_error.unwrap_or_else(|| "Unknown error after retries".to_string());
    error!(
        "[RUSTYSCRIPT] All execution attempts failed after {:?}: {}",
        execution_start.elapsed(),
        final_error
    );
    Err(final_error.into())
}

/// Create properly wrapped JavaScript code with comprehensive error handling
fn create_wrapped_javascript(
    js_code: &str,
    inputs: &Value,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let inputs_json = serde_json::to_string(inputs)?;

    let wrapped_code = format!(
        r#"
        // Enhanced JavaScript wrapper for actor system execution
        // Inject variables into globalThis.inputs for compatibility
        Object.assign(globalThis, {{ inputs: {inputs_json} }});
        
        // Create a safer execution environment
        const executeUserCode = () => {{
            try {{
                // Execute user code in an IIFE to capture return value
                const result = (() => {{
                    {js_code}
                }})();
                
                // Validate return value
                if (result === undefined) {{
                    return {{ 
                        internal_error: 'JavaScript code must explicitly return a value. Add a return statement to your code.' 
                    }};
                }}
                
                // Handle different result types appropriately
                if (result === null) {{
                    return {{ result: null }};
                }}
                
                if (typeof result === 'object') {{
                    // Return objects as-is
                    return result;
                }}
                
                // Wrap primitives in a result object
                return {{ result }};
                
            }} catch (error) {{
                // Comprehensive error reporting
                return {{ 
                    internal_error: `JavaScript execution error: ${{error.message}}`,
                    error_type: error.name || 'Error',
                    error_stack: error.stack || 'No stack trace available',
                    error_line: error.lineNumber || 'Unknown'
                }};
            }}
        }};

        // Execute and return result
        executeUserCode();
        "#
    );

    info!(
        "[RUSTYSCRIPT] Generated wrapped code, total length: {} chars",
        wrapped_code.len()
    );

    Ok(wrapped_code)
}

/// Log detailed information about the execution result
fn log_result_info(result: &Value) {
    let result_type = match result {
        Value::Object(_) => "object",
        Value::Array(_) => "array",
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
    };

    let result_size = serde_json::to_string(result).map(|s| s.len()).unwrap_or(0);

    info!(
        "[RUSTYSCRIPT] Result type: {}, size: {} bytes",
        result_type, result_size
    );

    // Log object structure for debugging (but not the full content)
    if let Value::Object(obj) = result {
        let keys: Vec<&String> = obj.keys().collect();
        info!("[RUSTYSCRIPT] Result object keys: {:?}", keys);
    }
}

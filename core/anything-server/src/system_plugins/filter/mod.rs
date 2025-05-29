use rustyscript::{json_args, Module, Runtime, RuntimeOptions};
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

/// Enhanced filter task processor optimized for the actor system
/// This is used for conditional logic and boolean expressions
/// Removes unnecessary thread spawning since actors already provide isolation
#[instrument(skip(bundled_inputs, bundled_plugin_config))]
pub async fn process_filter_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    info!("[FILTER] Starting filter task processing");
    info!("[FILTER] Input data: {:?}", bundled_inputs);

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

    // Execute filter condition
    let result = execute_filter_condition(js_code, bundled_inputs).await?;

    let total_duration = start.elapsed();
    info!("[FILTER] Filter task completed in {:?}", total_duration);

    Ok(Some(result))
}

/// Execute filter condition with proper error handling
async fn execute_filter_condition(
    js_code: &str,
    inputs: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    info!("[FILTER] Preparing filter condition execution");

    // Determine if this is a simple expression or a function
    let is_simple_expression = !js_code.contains("return");

    // Create wrapped code appropriate for the expression type
    let wrapped_code = create_wrapped_filter_code(js_code, inputs, is_simple_expression)?;

    // Generate unique module name
    let module_name = format!("user_condition_{}.js", Uuid::new_v4());

    info!("[FILTER] Creating module: {}", module_name);

    // Execute with appropriate timeout for actor system
    let execution_start = Instant::now();
    info!("[FILTER] Starting condition execution with 15 second timeout");

    // Clone values for the blocking task
    let wrapped_code_clone = wrapped_code.clone();
    let module_name_clone = module_name.clone();

    // Add retry logic and better error handling
    let max_retries = 2;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        if attempt > 0 {
            warn!(
                "[FILTER] Retrying execution, attempt {}/{}",
                attempt + 1,
                max_retries + 1
            );
            // Small delay between retries
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let wrapped_code_for_attempt = wrapped_code_clone.clone();
        let module_name_for_attempt = module_name_clone.clone();

        let execution_result = tokio::task::spawn_blocking(move || {
            // Create module inside the blocking task
            let module = Module::new(&module_name_for_attempt, &wrapped_code_for_attempt);

            // Configure runtime with more conservative settings
            let runtime_options = RuntimeOptions {
                timeout: Duration::from_secs(12), // Slightly less than outer timeout
                default_entrypoint: Some("default".to_string()),
                ..Default::default()
            };

            // Execute with panic catching
            type PanicResult =
                Result<Result<Value, rustyscript::Error>, Box<dyn std::any::Any + Send>>;
            let result: PanicResult =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    Runtime::execute_module(&module, vec![], runtime_options, json_args!())
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
                    "[FILTER] Condition executed successfully in {:?}",
                    execution_duration
                );

                // Check for internal error markers
                if let Some(error) = result.get("internal_error") {
                    if let Some(error_msg) = error.as_str() {
                        error!("[FILTER] Filter condition error: {}", error_msg);
                        last_error = Some(error_msg.to_string());
                        continue; // Retry on internal errors
                    }
                }

                info!("[FILTER] Condition result: {:?}", result);
                return Ok(result);
            }
            Ok(Err(e)) => {
                error!("[FILTER] Execution error: {}", e);
                last_error = Some(e);
                continue; // Retry
            }
            Err(join_error) => {
                error!("[FILTER] Task join error: {}", join_error);

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
                    error!("[FILTER] Task panicked: {}", panic_msg);
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
        "[FILTER] All execution attempts failed after {:?}: {}",
        execution_start.elapsed(),
        final_error
    );
    Err(final_error.into())
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
            Object.assign(globalThis, {{ inputs: {inputs_json} }});

            export default () => {{
                try {{
                    const result = {js_code};
                    
                    // Ensure we got a value
                    if (result === undefined) {{
                        return {{ 
                            internal_error: 'Filter expression returned undefined. Please ensure your expression evaluates to a boolean value.',
                            actual_value: 'undefined'
                        }};
                    }}

                    // If result is a boolean, use it directly
                    if (typeof result === 'boolean') {{
                        return {{ result }};
                    }}
                    
                    // If result is a string "true" or "false", convert it
                    if (typeof result === 'string' && (result.toLowerCase() === 'true' || result.toLowerCase() === 'false')) {{
                        return {{ result: result.toLowerCase() === 'true' }};
                    }}
                    
                    // Truthy/falsy conversion for other types
                    return {{ result: Boolean(result) }};
                    
                }} catch (error) {{
                    return {{ 
                        internal_error: `Filter expression error: ${{error.message}}`,
                        error_type: error.name || 'Error',
                        error_stack: error.stack || 'No stack trace available'
                    }};
                }}
            }}
            "#
        )
    } else {
        format!(
            r#"
            // Enhanced filter wrapper for function-style conditions
            Object.assign(globalThis, {{ inputs: {inputs_json} }});

            export default () => {{
                try {{
                    const result = (() => {{
                        {js_code}
                    }})();
                    
                    if (result === undefined) {{
                        return {{ 
                            internal_error: 'Filter function must return a value. Add a return statement to your condition.',
                            actual_value: 'undefined'
                        }};
                    }}

                    // Convert to boolean
                    if (typeof result === 'boolean') {{
                        return {{ result }};
                    }}
                    
                    return {{ result: Boolean(result) }};
                    
                }} catch (error) {{
                    return {{ 
                        internal_error: `Filter function error: ${{error.message}}`,
                        error_type: error.name || 'Error',
                        error_stack: error.stack || 'No stack trace available'
                    }};
                }}
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

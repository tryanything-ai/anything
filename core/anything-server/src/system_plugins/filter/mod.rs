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
    let module = Module::new(&module_name, &wrapped_code);

    // Execute with appropriate timeout for actor system
    let execution_start = Instant::now();
    info!("[FILTER] Starting condition execution with 15 second timeout");

    let runtime_options = RuntimeOptions {
        timeout: Duration::from_secs(15), // Increased from 1 second for complex conditions
        default_entrypoint: Some("default".to_string()),
        ..Default::default()
    };

    let execution_result: Result<Value, rustyscript::Error> =
        Runtime::execute_module(&module, vec![], runtime_options, json_args!());

    let execution_duration = execution_start.elapsed();

    match execution_result {
        Ok(result) => {
            info!(
                "[FILTER] Condition executed successfully in {:?}",
                execution_duration
            );

            // Check for internal error markers
            if let Some(error) = result.get("internal_error") {
                if let Some(error_msg) = error.as_str() {
                    error!("[FILTER] Filter condition error: {}", error_msg);
                    return Err(error_msg.into());
                }
            }

            info!("[FILTER] Condition result: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            error!(
                "[FILTER] Condition execution failed after {:?}: {:?}",
                execution_duration, e
            );
            Err(format!("Filter condition execution failed: {}", e).into())
        }
    }
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

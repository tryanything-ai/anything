use rustyscript::{json_args, Module, Runtime, RuntimeOptions};
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use tokio::task;

//This is meant to be used for function calls if we do agents and voice call type thing
//And to be how we do reusable flows or sublfows
//TODO: maybe just make this expect JS, and we just let it always be JS for determining truth
pub async fn process_filter_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[FILTER] Processing filter task");
    let start = Instant::now();
    println!("[FILTER] Starting process_filter_task");
    println!("[FILTER] Bundled variables: {:?}", bundled_inputs);

    // Clone the context since we need to move it to the new thread
    let bundled_plugin_config_clone = bundled_plugin_config.clone();
    let bundled_inputs_clone = bundled_inputs.clone();

    // Spawn blocking task in a separate thread
    let result = task::spawn_blocking(move || {
        // Move the JavaScript execution logic into this closure
        let js_code = bundled_plugin_config_clone["condition"]
            .as_str()
            .ok_or("JS code not found in context")?;

        println!("[FILTER] Extracted JS code: {:?}", js_code);
        println!("[FILTER] Extracted inputs: {:?}", bundled_inputs_clone);

        // Simple check - if it has 'return', treat as function, otherwise as expression
        let is_simple_expression = !js_code.contains("return");

        let wrapped_code = if is_simple_expression {
            format!(
                r#"
                // Inject variables into globalThis.inputs
                Object.assign(globalThis, {{ inputs: {} }});

                export default () => {{
                    try {{
                        const result = {js_code};
                        
                        // Ensure we got a value
                        if (result === undefined) {{
                            return {{ 
                                internal_error: 'Expression returned undefined. Please ensure your expression evaluates to a boolean value.',
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
                        
                        // Invalid result type
                        return {{ 
                            internal_error: `Expression must evaluate to a boolean value or "true"/"false" string, got: ${{typeof result}}`,
                            actual_value: JSON.stringify(result)
                        }};
                    }} catch (error) {{
                        return {{ 
                            internal_error: `JavaScript execution error: ${{error.message}}`,
                            error_type: error.name,
                            error_stack: error.stack
                        }};
                    }}
                }}
                "#,
                serde_json::to_string(&bundled_inputs_clone)?
            )
        } else {
            format!(
                r#"
                // Inject variables into globalThis.inputs
                Object.assign(globalThis, {{ inputs: {} }});

                export default () => {{
                    try {{
                        const result = (function() {{
                            {js_code}
                        }})();
                        
                        // Ensure the function returned a value
                        if (result === undefined) {{
                            return {{ 
                                internal_error: 'Function did not return a value. Please add an explicit return statement that returns a boolean.',
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
                        
                        // Invalid return type
                        return {{ 
                            internal_error: `Function must return a boolean value or "true"/"false" string, got: ${{typeof result}}`,
                            actual_value: JSON.stringify(result)
                        }};
                    }} catch (error) {{
                        return {{ 
                            internal_error: `JavaScript execution error: ${{error.message}}`,
                            error_type: error.name,
                            error_stack: error.stack
                        }};
                    }}
                }}
                "#,
                serde_json::to_string(&bundled_inputs_clone)?
            )
        };

        println!("[FILTER] Generated wrapped code: {:?}", wrapped_code);

        // Create the module
        let module = Module::new("user_condition.js", &wrapped_code);
        println!("[FILTER] Created module");

        // Execute the module
        let script_start = Instant::now();
        println!("[FILTER] Starting script execution");

        let result: Value = Runtime::execute_module(
            &module,
            vec![], // No additional modules needed
            RuntimeOptions {
                timeout: Duration::from_secs(1), //TODO: this actually does not prevent script from long runs
                ..Default::default()
            },
            json_args!(), // No arguments needed since we inject via globalThis
        )?;

        // Check if the result is our error object and convert it to a Rust error
        if let Some(error) = result.get("internal_error") {
            if let Some(error_msg) = error.as_str() {
                return Err(error_msg.into());
            }
        }

        println!(
            "[FILTER] Script execution completed in {:?}",
            script_start.elapsed()
        );
        println!("[FILTER] Execution result: {:?}", result);

        Ok::<Value, Box<dyn std::error::Error + Send + Sync>>(result)
    })
    .await??; // Note the double ?? to handle both the JoinError and the inner Result

    println!("[FILTER] Total task processing took {:?}", start.elapsed());

    // Extract the boolean result
    let condition = result
        .get("result")
        .and_then(|v| v.as_bool())
        .ok_or("Failed to get boolean result from filter")?;

    // Return a result with should_continue flag
    Ok(Some(json!({
        "result": {
            "should_continue": condition,
            "condition_result": condition
        }
    })))
}

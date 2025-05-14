use rustyscript::{json_args, Module, Runtime, RuntimeOptions};
use serde_json::Value;
use std::time::Duration;
use tokio::task;
use tokio::time::Instant;
use tracing::{error, info, instrument, Span};
use uuid::Uuid;

#[instrument(skip(bundled_inputs, bundled_plugin_config))]
pub async fn process_js_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    info!("[RUSTYSCRIPT] Starting process_js_task");
    info!("[RUSTYSCRIPT] Bundled variables: {:?}", bundled_inputs);
    info!("[RUSTYSCRIPT] Plugin config: {:?}", bundled_plugin_config);

    // Clone the context since we need to move it to the new thread
    let clone_span = tracing::info_span!("clone_inputs");
    let clone_start = Instant::now();
    let bundled_plugin_config_clone = bundled_plugin_config.clone();
    let bundled_inputs_clone = bundled_inputs.clone();
    let clone_duration = clone_start.elapsed();
    info!(
        "[RUSTYSCRIPT] Created clones of input data for thread in {:?}",
        clone_duration
    );

    // Spawn blocking task in a separate thread
    info!("[RUSTYSCRIPT] Spawning blocking task in separate thread");
    let result = task::spawn_blocking(move || {
        let res = std::panic::catch_unwind(|| {
            info!("[RUSTYSCRIPT] Inside blocking task");
            // Move the JavaScript execution logic into this closure
            let js_code = match bundled_plugin_config_clone["code"].as_str() {
                Some(code) => {
                    info!("[RUSTYSCRIPT] Successfully extracted JS code, length: {} chars", code.len());
                    code
                },
                None => {
                    error!("[RUSTYSCRIPT] ERROR: JS code not found in context");
                    return Err::<Value, Box<dyn std::error::Error + Send + Sync>>("JS code not found in context".into());
                }
            };

            info!("[RUSTYSCRIPT] Preparing to wrap JS code with context");
            info!("[RUSTYSCRIPT] Input data size: {} bytes", 
                serde_json::to_string(&bundled_inputs_clone)
                    .map(|s| s.len())
                    .unwrap_or(0)
            );

            // Create a module that wraps the user's code with context and exports
            let wrap_span = tracing::info_span!("wrap_code");
            let wrap_start = Instant::now();
            let wrapped_code = format!(
                r#"
                // Inject variables into globalThis.inputs to match autocomplete
                Object.assign(globalThis, {{ inputs: {} }});

                // Export the user's code as default function and let errors propagate
                export default () => {{
                    try {{
                        const result = (() => {{
                            {js_code}
                        }})();
                        
                        // Ensure the user returned a value
                        if (result === undefined) {{
                            return {{ internal_error: 'Please explicitly return a value in your code' }};
                        }}

                        // If result is not an object, wrap it in an object
                        if (result === null || typeof result !== 'object') {{
                            return {{ result }};
                        }}
                        
                        return result;
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
            );
            let wrap_duration = wrap_start.elapsed();
            info!("[RUSTYSCRIPT] Generated wrapped code, length: {} chars, in {:?}", wrapped_code.len(), wrap_duration);

            // Create the module with unique name
            let module_span = tracing::info_span!("create_module");
            let module_start = Instant::now();
            info!("[RUSTYSCRIPT] Creating module from wrapped code");
            let module_name = format!("user_code_{}.js", Uuid::new_v4());
            let module = Module::new(&module_name, &wrapped_code);
            info!("[RUSTYSCRIPT] Successfully created module: {}", module_name);
            let module_duration = module_start.elapsed();
            info!("[RUSTYSCRIPT] Module creation took {:?}", module_duration);

            // Execute the module
            let script_span = tracing::info_span!("script_execution");
            let script_start = Instant::now();
            info!("[RUSTYSCRIPT] Starting script execution with 1 second timeout");

            let result: Value = match Runtime::execute_module(
                &module,
                vec![],
                RuntimeOptions {
                    timeout: Duration::from_secs(1),
                    ..Default::default()
                },
                json_args!(),
            ) {
                Ok(r) => {
                    info!("[RUSTYSCRIPT] Script execution completed successfully");
                    // Clean up the module file
                    if let Err(e) = std::fs::remove_file(&module_name) {
                        info!("[RUSTYSCRIPT] Warning: Failed to clean up module file: {}", e);
                    }
                    r
                },
                Err(e) => {
                    // Clean up on error too
                    if let Err(e) = std::fs::remove_file(&module_name) {
                        info!("[RUSTYSCRIPT] Warning: Failed to clean up module file: {}", e);
                    }
                    error!("[RUSTYSCRIPT] ERROR: Script execution failed: {:?}", e);
                    return Err(e.into());
                }
            };

            // Check if the result is our error object and convert it to a Rust error
            if let Some(error) = result.get("internal_error") {
                if let Some(error_msg) = error.as_str() {
                    error!("[RUSTYSCRIPT] ERROR: Internal JavaScript error: {}", error_msg);
                    return Err(error_msg.into());
                }
            }

            let script_duration = script_start.elapsed();
            info!("[RUSTYSCRIPT] Script execution completed in {:?}", script_duration);
            info!("[RUSTYSCRIPT] Result type: {}", 
                if result.is_object() { "object" }
                else if result.is_array() { "array" }
                else if result.is_string() { "string" }
                else if result.is_number() { "number" }
                else if result.is_boolean() { "boolean" }
                else if result.is_null() { "null" }
                else { "unknown" }
            );
            info!("[RUSTYSCRIPT] Result size: {} bytes", 
                serde_json::to_string(&result)
                    .map(|s| s.len())
                    .unwrap_or(0)
            );

            Ok(result)
        });
        match res {
            Ok(inner_result) => inner_result,
            Err(e) => {
                error!("[RUSTYSCRIPT] Panic caught in JS task: {:?}", e);
                Err("Panic in JS task".into())
            }
        }
    })
    .await??; // Note the double ?? to handle both the JoinError and the inner Result

    info!(
        "[RUSTYSCRIPT] Total task processing completed in {:?}",
        start.elapsed()
    );
    info!("[RUSTYSCRIPT] Successfully returning result");
    Ok(Some(result))
}

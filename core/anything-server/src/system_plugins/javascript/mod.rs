use rustyscript::{json_args, Module, Runtime, RuntimeOptions};
use serde_json::Value;
use std::time::Duration;
use tokio::task;
use tokio::time::Instant;
use uuid::Uuid;

pub async fn process_js_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[RUSTYSCRIPT] Starting process_js_task");
    println!("[RUSTYSCRIPT] Bundled variables: {:?}", bundled_inputs);
    println!("[RUSTYSCRIPT] Plugin config: {:?}", bundled_plugin_config);

    // Clone the context since we need to move it to the new thread
    let bundled_plugin_config_clone = bundled_plugin_config.clone();
    let bundled_inputs_clone = bundled_inputs.clone();
    println!("[RUSTYSCRIPT] Created clones of input data for thread");

    // Spawn blocking task in a separate thread
    println!("[RUSTYSCRIPT] Spawning blocking task in separate thread");
    let result = task::spawn_blocking(move || {
        println!("[RUSTYSCRIPT] Inside blocking task");
        
        // Move the JavaScript execution logic into this closure
        let js_code = match bundled_plugin_config_clone["code"].as_str() {
            Some(code) => {
                println!("[RUSTYSCRIPT] Successfully extracted JS code, length: {} chars", code.len());
                code
            },
            None => {
                println!("[RUSTYSCRIPT] ERROR: JS code not found in context");
                return Err::<Value, Box<dyn std::error::Error + Send + Sync>>("JS code not found in context".into());
            }
        };

        println!("[RUSTYSCRIPT] Preparing to wrap JS code with context");
        println!("[RUSTYSCRIPT] Input data size: {} bytes", 
            serde_json::to_string(&bundled_inputs_clone)
                .map(|s| s.len())
                .unwrap_or(0)
        );

        // Create a module that wraps the user's code with context and exports
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

        println!("[RUSTYSCRIPT] Generated wrapped code, length: {} chars", wrapped_code.len());

        // Create the module with unique name
        println!("[RUSTYSCRIPT] Creating module from wrapped code");
        let module_name = format!("user_code_{}.js", Uuid::new_v4());
        let module = Module::new(&module_name, &wrapped_code);
        println!("[RUSTYSCRIPT] Successfully created module: {}", module_name);

        // Execute the module
        let script_start = Instant::now();
        println!("[RUSTYSCRIPT] Starting script execution with 1 second timeout");

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
                println!("[RUSTYSCRIPT] Script execution completed successfully");
                // Clean up the module file
                if let Err(e) = std::fs::remove_file(&module_name) {
                    println!("[RUSTYSCRIPT] Warning: Failed to clean up module file: {}", e);
                }
                r
            },
            Err(e) => {
                // Clean up on error too
                if let Err(e) = std::fs::remove_file(&module_name) {
                    println!("[RUSTYSCRIPT] Warning: Failed to clean up module file: {}", e);
                }
                println!("[RUSTYSCRIPT] ERROR: Script execution failed: {:?}", e);
                return Err(e.into());
            }
        };

        // Check if the result is our error object and convert it to a Rust error
        if let Some(error) = result.get("internal_error") {
            if let Some(error_msg) = error.as_str() {
                println!("[RUSTYSCRIPT] ERROR: Internal JavaScript error: {}", error_msg);
                return Err(error_msg.into());
            }
        }

        println!(
            "[RUSTYSCRIPT] Script execution completed in {:?}",
            script_start.elapsed()
        );
        println!("[RUSTYSCRIPT] Result type: {}", 
            if result.is_object() { "object" }
            else if result.is_array() { "array" }
            else if result.is_string() { "string" }
            else if result.is_number() { "number" }
            else if result.is_boolean() { "boolean" }
            else if result.is_null() { "null" }
            else { "unknown" }
        );
        println!("[RUSTYSCRIPT] Result size: {} bytes", 
            serde_json::to_string(&result)
                .map(|s| s.len())
                .unwrap_or(0)
        );

        Ok(result)
    })
    .await??; // Note the double ?? to handle both the JoinError and the inner Result

    println!(
        "[RUSTYSCRIPT] Total task processing completed in {:?}",
        start.elapsed()
    );
    println!("[RUSTYSCRIPT] Successfully returning result");
    Ok(Some(result))
}
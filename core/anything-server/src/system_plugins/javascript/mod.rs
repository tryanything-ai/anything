use rustyscript::{json_args, Module, Runtime, RuntimeOptions};
use serde_json::Value;
use std::time::Duration;
use tokio::task;
use tokio::time::Instant;

pub async fn process_js_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[RUSTYSCRIPT] Starting process_js_task");
    println!("[RUSTYSCRIPT] Bundled variables: {:?}", bundled_inputs);

    // Clone the context since we need to move it to the new thread
    let bundled_plugin_config_clone = bundled_plugin_config.clone();
    let bundled_inputs_clone = bundled_inputs.clone();

    // Spawn blocking task in a separate thread
    let result = task::spawn_blocking(move || {
        // Move the JavaScript execution logic into this closure
        let js_code = bundled_plugin_config_clone["code"]
            .as_str()
            .ok_or("JS code not found in context")?;

        println!("[RUSTYSCRIPT] Extracted JS code: {:?}", js_code);
        println!(
            "[RUSTYSCRIPT] Extracted inputs: {:?}",
            bundled_inputs_clone
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

        println!("[RUSTYSCRIPT] Generated wrapped code: {:?}", wrapped_code);

        // Create the module
        let module = Module::new("user_code.js", &wrapped_code);
        println!("[RUSTYSCRIPT] Created module");

        // Execute the module
        let script_start = Instant::now();
        println!("[RUSTYSCRIPT] Starting script execution");

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
            "[RUSTYSCRIPT] Script execution completed in {:?}",
            script_start.elapsed()
        );
        println!("[RUSTYSCRIPT] Execution result: {:?}", result);

        Ok::<Value, Box<dyn std::error::Error + Send + Sync>>(result)
    })
    .await??; // Note the double ?? to handle both the JoinError and the inner Result

    println!(
        "[RUSTYSCRIPT] Total task processing took {:?}",
        start.elapsed()
    );
    Ok(Some(result))
}
use rustyscript::{json_args, Module, Runtime, RuntimeOptions};
use serde_json::Value;
use std::time::Duration;
use tokio::task;
use tokio::time::Instant;

pub async fn process_js_task(
    bundled_variables: &Value,
    bundled_inputs: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[RUSTYSCRIPT] Starting process_js_task");
    println!("[RUSTYSCRIPT] Bundled variables: {:?}", bundled_variables);

    // Clone the context since we need to move it to the new thread
    let bundled_inputs_clone = bundled_inputs.clone();
    let bundled_variables_clone = bundled_variables.clone();

    // Spawn blocking task in a separate thread
    let result = task::spawn_blocking(move || {
        // Move the JavaScript execution logic into this closure
        let js_code = bundled_inputs_clone["code"]
            .as_str()
            .ok_or("JS code not found in context")?;

        println!("[RUSTYSCRIPT] Extracted JS code: {:?}", js_code);
        println!(
            "[RUSTYSCRIPT] Extracted variables: {:?}",
            bundled_variables_clone
        );

        // Create a module that wraps the user's code with context and exports
        let wrapped_code = format!(
            r#"
            // Inject variables into globalThis.variables to match autocomplete
            Object.assign(globalThis, {{ variables: {} }});

            // Export the user's code as default function 
            export default () => {{
                {js_code}
            }}
            "#,
            serde_json::to_string(&bundled_variables_clone)?
        );

        println!("[RUSTYSCRIPT] Generated wrapped code: {:?}", wrapped_code);

        // Create the module
        let module = Module::new("user_code.js", &wrapped_code);
        println!("[RUSTYSCRIPT] Created module");

        // Execute the module
        let script_start = Instant::now();
        println!("[RUSTYSCRIPT] Starting script execution");

        let result = Runtime::execute_module(
            &module,
            vec![], // No additional modules needed
            RuntimeOptions {
                timeout: Duration::from_secs(1), //TODO: this actually does not prevent script from long runs
                ..Default::default()
            },
            json_args!(), // No arguments needed since we inject via globalThis
        )?;

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;

//     #[test]
//     fn test_js_with_variables() {
//         println!("[RUSTYSCRIPT TEST] Running test_js_with_variables");
//         let context = json!({
//             "code": "return x + y;",
//             "variables": {
//                 "x": 20,
//                 "y": 22
//             }
//         });

//         let result = process_js_task(&context).await.unwrap().unwrap();
//         assert_eq!(result, json!(42));
//         println!("[RUSTYSCRIPT TEST] test_js_with_variables completed successfully");
//     }

//     #[test]
//     fn test_js_missing_code() {
//         println!("[RUSTYSCRIPT TEST] Running test_js_missing_code");
//         let context = json!({
//             "variables": {
//                 "x": 20,
//                 "y": 22
//             }
//         });

//         let result = process_js_task(&context);
//         assert!(result.is_err());
//         println!("[RUSTYSCRIPT TEST] test_js_missing_code completed successfully");
//     }

//     #[test]
//     fn test_js_no_variables() {
//         println!("[RUSTYSCRIPT TEST] Running test_js_no_variables");
//         let context = json!({
//             "code": "return 40 + 2;"
//         });

//         let result = process_js_task(&context).unwrap().unwrap();
//         assert_eq!(result, json!(42));
//         println!("[RUSTYSCRIPT TEST] test_js_no_variables completed successfully");
//     }
// }

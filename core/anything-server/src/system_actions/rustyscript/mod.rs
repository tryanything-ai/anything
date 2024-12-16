use rustyscript::{json_args, Runtime, Module, Error};
use serde_json::Value;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Instant;

pub struct RustyScriptTask {
    pub context: Value,
    pub response_channel: Sender<Result<Option<Value>, String>>,
}

fn process_js_task(
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[TASK_ENGINE] Entering process_js_task");

    // Extract js_code and variables from bundled_context
    let js_code = bundled_context["code"]
        .as_str()
        .ok_or("JS code not found in context")?;

    let variables = bundled_context
        .get("variables")
        .unwrap_or(&Value::Object(serde_json::Map::new()))
        .clone();

    // Create a module that wraps the user's code with context and exports
    let wrapped_code = format!(
        r#"
        // Inject all context variables into the scope
        Object.assign(globalThis, {});

        // Export the user's code as default function
        export default () => {{
            {js_code}
        }}
        "#,
        serde_json::to_string(&variables)?
    );

    println!(
        "[TASK_ENGINE] Creating module with code: {:?}",
        wrapped_code
    );

    // Create the module
    let module = Module::new("user_code.js", &wrapped_code);

    // Execute the module
    let script_start = Instant::now();
    let result = Runtime::execute_module(
        &module,
        vec![],  // No additional modules needed
        Default::default(),
        json_args!()  // No arguments needed since we inject via globalThis
    )?;

    println!("[SPEED] JS execution took {:?}", script_start.elapsed());
    println!("[TASK_ENGINE] JS Execution Result: {:?}", result);
    println!(
        "[SPEED] Total JS task processing took {:?}",
        start.elapsed()
    );

    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_js_with_variables() {
        let context = json!({
            "code": "return x + y;",
            "variables": {
                "x": 20,
                "y": 22
            }
        });

        let result = process_js_task(&context).unwrap().unwrap();
        assert_eq!(result, json!(42));
    }

    #[test]
    fn test_js_missing_code() {
        let context = json!({
            "variables": {
                "x": 20,
                "y": 22
            }
        });

        let result = process_js_task(&context);
        assert!(result.is_err());
    }

    #[test]
    fn test_js_no_variables() {
        let context = json!({
            "code": "return 40 + 2;"
        });

        let result = process_js_task(&context).unwrap().unwrap();
        assert_eq!(result, json!(42));
    }
}

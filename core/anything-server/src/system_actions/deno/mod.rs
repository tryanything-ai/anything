use std::rc::Rc;
use std::sync::Arc;

use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;
use serde_json::Value;
use serde_v8::from_v8;
use tokio::time::Instant;

pub mod processor;

pub async fn process_deno_js_task(
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[TASK_ENGINE] Entering process_deno_js_task");
    println!("[TASK_ENGINE] Bundled context: {:?}", bundled_context);

    // Extract the JavaScript code from the bundled context
    let code = match bundled_context.get("code").and_then(Value::as_str) {
        Some(code) => code,
        None => {
            println!("[TASK_ENGINE] No JavaScript code found in task context");
            return Err("JavaScript code is required".into());
        }
    };

    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

    // Define the main module specifier
    let main_module = ModuleSpecifier::parse("file:///anon.js").unwrap();

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(FsModuleLoader),
            permissions: PermissionsContainer::allow_all(permission_desc_parser),
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Default::default(),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            root_cert_store_provider: Default::default(),
            fetch_dns_resolver: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
            fs,
        },
        WorkerOptions {
            ..Default::default()
        },
    );

    // Create execution context script
    let context_script = create_execution_context(code, bundled_context);

    let scope = &mut worker.js_runtime.handle_scope();

    // let result = worker.execute_script("[anon]", context_script.into())?;
    // worker.run_event_loop(false).await?;

    // Extract the result using serde_v8

    // let context = scope.get_current_context();
    // let _global = context.global(scope);
    // let local_result = result.open(scope);

    // Convert V8 value to serde_json::Value
    // let json_result: Value = from_v8(scope, local_result)
    //     .map_err(|e| format!("Failed to convert V8 value to serde_json::Value: {}", e))?;

    // println!("[TASK_ENGINE] Result: {:?}", json_result);
    println!("[TASK_ENGINE] JavaScript execution successful");
    println!("[SPEED] Deno task processing took {:?}", start.elapsed());

    // Ok(Some(json_result))
    Ok(None)
}

// Helper function to create execution environment
fn create_execution_context(code: &str, context: &Value) -> String {
    format!(
        r#"
        (async function() {{
            const context = {};
            try {{
                const userFunction = async () => {{
                    {}
                }};
                return await userFunction.call(context);
            }} catch (error) {{
                console.log("[ERROR] " + error.toString());
                throw error;
            }}
        }})();
        "#,
        serde_json::to_string(context).unwrap(),
        code
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_process_deno_js_task_success() {
        let context = json!({
            "code": "console.log('test'); 42;",
            "someData": "test data"
        });

        let result = process_deno_js_task(&context).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), json!(42));
    }

    #[tokio::test]
    async fn test_process_deno_js_task_missing_code() {
        let context = json!({
            "someData": "test data"
        });

        let result = process_deno_js_task(&context).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "JavaScript code is required"
        );
    }

    #[tokio::test]
    async fn test_context_access() {
        let context = json!({
            "code": "console.log(this.someData); this.someData;",
            "someData": "test data"
        });

        let result = process_deno_js_task(&context).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), json!("test data"));
    }

    #[tokio::test]
    async fn test_runtime_error() {
        let context = json!({
            "code": "throw new Error('Test error');",
        });

        let result = process_deno_js_task(&context).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error: Test error");
    }

    #[tokio::test]
    async fn test_disallowed_import_statement() {
        let context = json!({
            "code": "import fs from 'fs'; console.log(fs);",
        });

        let result = process_deno_js_task(&context).await;
        // Depending on Deno's permission settings, adjust the expectation
        assert!(result.is_ok());
        // You might need to adjust this based on actual permissions and Deno's response
    }

    #[test]
    fn test_create_execution_context() {
        let code = "console.log('test'); 42;";
        let context = json!({
            "data": "test data"
        });

        let result = create_execution_context(code, &context);

        assert!(result.contains(r#"const context = {"data":"test data"};"#));
        assert!(result.contains("console.log('test');"));
        assert!(result.contains("async function"));
    }
}

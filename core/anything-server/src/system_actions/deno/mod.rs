use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;
use serde_json::Value;
use tokio::time::Instant;

pub mod processor;

#[op2(fast)]
fn op_hello(#[string] text: &str) {
    println!("Hello {} from an op!", text);
}

deno_core::extension!(
    hello_runtime,
    ops = [op_hello],
    // esm = [r#"
    //     const globalThis = {}; // Minimal bootstrap
    //     // Add any additional ESM code here
    // "#]
);

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

    // Create a temporary file for the code
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
            extensions: vec![hello_runtime::init_ops_and_esm()],
            ..Default::default()
        },
    );

    // Create context and execute code
    let context_script = create_execution_context(code, bundled_context);
    worker.execute_script("[anon]", context_script.into())?;
    worker.run_event_loop(false).await?;

    println!("[TASK_ENGINE] JavaScript execution successful");
    println!("[SPEED] Deno task processing took {:?}", start.elapsed());

    // For now just return None since we need to implement proper result handling
    Ok(None)
}

// Helper function to create execution environment
fn create_execution_context(code: &str, context: &Value) -> String {
    format!(
        r#"
        (async function() {{
            const context = {};
            try {{
                const result = await (async () => {{
                    {}
                }})();
                return result;
            }} catch (error) {{
                console.log("[ERROR] " + error.toString());
                return {{ error: error.toString() }};
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
            "code": "console.log('test'); return 42;",
            "someData": "test data"
        });

        let result = process_deno_js_task(&context).await;
        assert!(result.is_ok());
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

    #[test]
    fn test_create_execution_context() {
        let code = "console.log('test');";
        let context = json!({
            "data": "test data"
        });

        let result = create_execution_context(code, &context);

        assert!(result.contains("const context = {\"data\":\"test data\"}"));
        assert!(result.contains("console.log('test');"));
        assert!(result.contains("async function()"));
    }
}

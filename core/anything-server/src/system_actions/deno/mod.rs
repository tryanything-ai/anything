use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::Permissions;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;
use env_logger::{Builder, Target};
use log::{debug, error, info};
use serde_json::Value;
use tokio::time::Instant;

pub mod module_loader;
pub mod processor;

use module_loader::TypescriptModuleLoader;

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

    let source_map_store = Rc::new(RefCell::new(HashMap::new()));

    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

    // Create a temporary file for the code
    let main_module = ModuleSpecifier::parse("file:///anon.js").unwrap();

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(TypescriptModuleLoader {
                source_maps: source_map_store,
            }),
            permissions: PermissionsContainer::new(
                permission_desc_parser,
                Permissions::allow_all(),
            ),
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

    let context_script = create_execution_context(code, bundled_context);

    // Execute as module if it contains imports
    if code.contains("import ") {
        worker.execute_main_module(&main_module).await?;
    } else {
        worker.execute_script("[anon]", context_script.into())?;
    }

    worker.run_event_loop(false).await?;

    println!("[TASK_ENGINE] JavaScript execution successful");
    println!("[SPEED] Deno task processing took {:?}", start.elapsed());

    Ok(None)
}

// Helper function to create execution environment
fn create_execution_context(code: &str, context: &Value) -> String {
    // Check if the code contains import statements
    if code.contains("import ") {
        // For ESM, return the code directly
        code.to_string()
    } else {
        // For regular scripts, use the IIFE wrapper
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn init() {
        let _ = Builder::new()
            .target(Target::Stdout)
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    #[tokio::test]
    async fn test_process_deno_js_task_success() {
        init(); // Initialize logging
        info!("Starting success test");

        let context = json!({
            "code": "console.log('test'); return 42;",
            "someData": "test data"
        });

        let result = process_deno_js_task(&context).await;
        debug!("Test result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_deno_js_task_missing_code() {
        init(); // Initialize logging
        info!("Starting missing code test");

        let context = json!({
            "someData": "test data"
        });

        let result = process_deno_js_task(&context).await;
        debug!("Test result: {:?}", result);
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

    #[tokio::test]
    async fn test_esm_import() {
        init(); // Initialize logging
        info!("Starting ESM import test");

        let code = r#"
            import * as cowsay from "https://esm.sh/cowsay@1.6.0";
            
            const message = "Hello from ESM import test!";
            const result = cowsay.say({ text: message });
            console.log(result);
            return result;
        "#;

        let context = json!({
            "code": code,
        });

        let result = process_deno_js_task(&context).await;
        debug!("ESM import test result: {:?}", result);
        assert!(result.is_ok());
    }
}

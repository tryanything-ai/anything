use anyhow::{anyhow, Result};
use deno_core::{FastString, JsRuntime, RuntimeOptions};
use std::thread;
use std::time::{Duration, Instant};
use tracing::{error, info};

/// JavaScript execution engine using Deno Core
/// Uses thread pool for execution isolation without tokio
#[derive(Debug)]
pub struct JavaScriptEngine {
    // Thread pool for JavaScript execution
    worker_pool: crossbeam::channel::Sender<ExecutionTask>,
}

struct ExecutionTask {
    code: String,
    inputs_json: String,
    timeout_ms: u64,
    execution_id: String,
    response_sender: crossbeam::channel::Sender<Result<String>>,
}

impl JavaScriptEngine {
    pub fn new() -> Result<Self> {
        info!("🦀 Initializing Deno Core JavaScript engine (thread pool)");

        let (task_sender, task_receiver) = crossbeam::channel::unbounded::<ExecutionTask>();
        let pool_size = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .min(8); // Cap at 8 threads

        info!("Creating {} JavaScript worker threads", pool_size);

        // Create worker threads
        for worker_id in 0..pool_size {
            let receiver = task_receiver.clone();

            thread::Builder::new()
                .name(format!("js-worker-{}", worker_id))
                .spawn(move || {
                    info!("🧵 JavaScript worker {} started", worker_id);
                    Self::worker_thread(worker_id, receiver);
                    info!("🧵 JavaScript worker {} stopped", worker_id);
                })?;
        }

        Ok(Self {
            worker_pool: task_sender,
        })
    }

    fn worker_thread(worker_id: usize, receiver: crossbeam::channel::Receiver<ExecutionTask>) {
        while let Ok(task) = receiver.recv() {
            let start = Instant::now();
            info!(
                "[{}] Worker {} executing task",
                task.execution_id, worker_id
            );

            let result = Self::execute_on_worker(
                &task.code,
                &task.inputs_json,
                &task.execution_id,
                task.timeout_ms,
            );

            let duration = start.elapsed();
            match &result {
                Ok(_) => info!(
                    "[{}] Worker {} completed in {:?}",
                    task.execution_id, worker_id, duration
                ),
                Err(e) => error!(
                    "[{}] Worker {} failed in {:?}: {}",
                    task.execution_id, worker_id, duration, e
                ),
            }

            // Send result back (ignore if receiver is dropped)
            let _ = task.response_sender.send(result);
        }
    }

    fn create_runtime() -> Result<JsRuntime> {
        let options = RuntimeOptions {
            // Disable module loading for security
            module_loader: None,
            // Disable extensions that could be unsafe
            extensions: vec![],
            // Enable V8 inspector for debugging (optional)
            inspector: false,
            // Disable web platform APIs for security
            is_main: false,
            ..Default::default()
        };

        let mut runtime = JsRuntime::new(options);

        // Set up safe execution environment
        let setup_code = r#"
            // Create safe console object (using a simpler approach)
            globalThis.console = {
                log: (...args) => { /* log to stdout silently */ },
                error: (...args) => { /* log to stderr silently */ },
                warn: (...args) => { /* log to stderr silently */ },
                info: (...args) => { /* log to stdout silently */ },
            };
            
            // Remove dangerous globals
            delete globalThis.Deno;
            delete globalThis.fetch; // Remove network access
            delete globalThis.WebSocket;
            delete globalThis.Worker;
            
            // Create safe execution function
            globalThis.executeUserCode = function(code, inputs) {
                try {
                    // Create isolated function scope
                    const userFunction = new Function('inputs', `
                        "use strict";
                        
                        // Execute user code and capture result
                        const executeCode = () => {
                            ${code}
                        };
                        
                        const result = executeCode();
                        
                        // Validate return value
                        if (result === undefined) {
                            throw new Error('JavaScript code must explicitly return a value. Add a return statement to your code.');
                        }
                        
                        return result;
                    `);
                    
                    return userFunction(inputs);
                } catch (error) {
                    throw new Error(`JavaScript execution error: ${error.message}`);
                }
            };
        "#;

        runtime.execute_script("setup.js", FastString::Static(setup_code))?;

        Ok(runtime)
    }

    fn execute_on_worker(
        code: &str,
        inputs_json: &str,
        execution_id: &str,
        _timeout_ms: u64,
    ) -> Result<String> {
        info!("[{}] Creating new JsRuntime in worker thread", execution_id);

        // Create a fresh runtime for this execution
        let mut runtime = Self::create_runtime()?;

        // Parse inputs to validate JSON
        let _inputs: serde_json::Value =
            serde_json::from_str(inputs_json).map_err(|e| anyhow!("Invalid inputs JSON: {}", e))?;

        // Prepare execution script
        let execution_script = format!(
            r#"
            try {{
                const inputs = {};
                const result = globalThis.executeUserCode(`{}`, inputs);
                JSON.stringify({{ success: true, result: result }});
            }} catch (error) {{
                JSON.stringify({{ 
                    success: false, 
                    error: error.message,
                    stack: error.stack || 'No stack trace available'
                }});
            }}
            "#,
            inputs_json,
            code.replace('`', r#"\`"#).replace('\\', r#"\\"#)
        );

        // Execute the script
        let result = runtime.execute_script(
            "user_execution.js",
            FastString::Owned(execution_script.into()),
        )?;

        // Convert V8 value to JSON string using proper scope handling
        let scope = &mut runtime.handle_scope();
        let local_result = deno_core::v8::Local::new(scope, result);
        let result_str = local_result.to_string(scope).unwrap();
        let result_string = result_str.to_rust_string_lossy(scope);

        // Parse the result to check for errors
        let parsed_result: serde_json::Value = serde_json::from_str(&result_string)
            .map_err(|e| anyhow!("Failed to parse execution result: {}", e))?;

        if parsed_result["success"].as_bool() == Some(true) {
            // Successful execution, return the result as JSON
            let user_result = &parsed_result["result"];
            Ok(serde_json::to_string(user_result)?)
        } else {
            // Execution failed, return error
            let error_msg = parsed_result["error"].as_str().unwrap_or("Unknown error");
            Err(anyhow!("JavaScript execution failed: {}", error_msg))
        }
    }

    pub fn execute_javascript(
        &self,
        code: &str,
        inputs_json: &str,
        timeout_ms: u64,
        execution_id: &str,
    ) -> Result<String> {
        info!("[{}] Submitting task to worker pool", execution_id);

        let (response_sender, response_receiver) = crossbeam::channel::bounded(1);

        let task = ExecutionTask {
            code: code.to_string(),
            inputs_json: inputs_json.to_string(),
            timeout_ms,
            execution_id: execution_id.to_string(),
            response_sender,
        };

        // Submit task to worker pool
        self.worker_pool
            .send(task)
            .map_err(|_| anyhow!("Worker pool is shut down"))?;

        // Wait for result with timeout
        let timeout_duration = Duration::from_millis(timeout_ms.max(1000).min(60000));

        match response_receiver.recv_timeout(timeout_duration) {
            Ok(result) => result,
            Err(crossbeam::channel::RecvTimeoutError::Timeout) => Err(anyhow!(
                "JavaScript execution timed out after {}ms",
                timeout_ms
            )),
            Err(crossbeam::channel::RecvTimeoutError::Disconnected) => {
                Err(anyhow!("Worker thread disconnected"))
            }
        }
    }
}

impl Drop for JavaScriptEngine {
    fn drop(&mut self) {
        info!("🧹 Cleaning up JavaScript engine");
    }
}

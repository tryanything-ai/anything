use rustyscript::worker::{DefaultWorker, DefaultWorkerOptions};
use rustyscript::{Error as RustyScriptError, Module};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, warn};

// Worker request/response types
#[derive(Debug)]
pub struct WorkerRequest {
    pub execution_type: ExecutionType,
    pub inputs: Value,
    pub config: Value,
    pub response_tx: oneshot::Sender<WorkerResult>,
}

#[derive(Debug)]
pub enum ExecutionType {
    JavaScript,
    Filter,
}

pub type WorkerResult = Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct JsWorkerPool {
    request_tx: mpsc::Sender<WorkerRequest>,
    worker_count: usize,
}

impl JsWorkerPool {
    pub fn new(pool_size: usize) -> Result<Self, RustyScriptError> {
        let (request_tx, request_rx) = mpsc::channel::<WorkerRequest>(1000);
        let request_rx = Arc::new(tokio::sync::Mutex::new(request_rx));

        // Spawn worker tasks
        for i in 0..pool_size {
            let request_rx = Arc::clone(&request_rx);
            tokio::spawn(async move {
                if let Err(e) = worker_task(i, request_rx).await {
                    error!("Worker {} failed: {}", i, e);
                }
            });
        }

        info!("Created JS worker pool with {} workers", pool_size);

        Ok(Self {
            request_tx,
            worker_count: pool_size,
        })
    }

    pub async fn execute_javascript(
        &self,
        inputs: &Value,
        config: &Value,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        self.execute_request(ExecutionType::JavaScript, inputs, config)
            .await
    }

    pub async fn execute_filter(
        &self,
        inputs: &Value,
        config: &Value,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        self.execute_request(ExecutionType::Filter, inputs, config)
            .await
    }

    async fn execute_request(
        &self,
        execution_type: ExecutionType,
        inputs: &Value,
        config: &Value,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let (response_tx, response_rx) = oneshot::channel();

        let request = WorkerRequest {
            execution_type,
            inputs: inputs.clone(),
            config: config.clone(),
            response_tx,
        };

        // Send request to worker pool
        self.request_tx
            .send(request)
            .await
            .map_err(|_| "Worker pool is closed")?;

        // Wait for response
        response_rx
            .await
            .map_err(|_| "Worker response channel closed")?
    }

    pub async fn shutdown(&self) {
        info!("Shutting down JS worker pool");
        // Closing the request channel will cause workers to shutdown
        // The Drop implementation will handle this automatically
    }

    pub fn worker_count(&self) -> usize {
        self.worker_count
    }
}

// Worker task that runs in its own tokio task
async fn worker_task(
    worker_id: usize,
    request_rx: Arc<tokio::sync::Mutex<mpsc::Receiver<WorkerRequest>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting JS worker {}", worker_id);

    // Create worker in this task (each worker gets its own runtime)
    let worker = DefaultWorker::new(DefaultWorkerOptions {
        default_entrypoint: None,
        timeout: Duration::from_secs(30),
        startup_snapshot: None,
        shared_array_buffer_store: None,
    })?;

    info!("JS worker {} initialized successfully", worker_id);

    // Process requests
    loop {
        let request = {
            let mut rx = request_rx.lock().await;
            match rx.recv().await {
                Some(request) => request,
                None => {
                    info!("Worker {} shutting down - channel closed", worker_id);
                    break;
                }
            }
        };

        let result = match request.execution_type {
            ExecutionType::JavaScript => {
                execute_javascript_on_worker(&worker, &request.inputs, &request.config)
            }
            ExecutionType::Filter => {
                execute_filter_on_worker(&worker, &request.inputs, &request.config)
            }
        };

        // Send response back (ignore if receiver dropped)
        let _ = request.response_tx.send(result);
    }

    info!("JS worker {} shut down", worker_id);
    Ok(())
}

fn execute_javascript_on_worker(
    worker: &DefaultWorker,
    inputs: &Value,
    config: &Value,
) -> WorkerResult {
    // Extract the JavaScript code from config
    let code = config
        .get("code")
        .and_then(|c| c.as_str())
        .ok_or("No JavaScript code provided")?;

    // Create the execution context
    let execution_code = format!(
        r#"
        const inputs = {};
        const config = {};
        
        // Execute the user's JavaScript code
        const result = (function() {{
            {}
        }})();
        
        result;
        "#,
        serde_json::to_string(inputs)?,
        serde_json::to_string(config)?,
        code
    );

    info!("Executing JavaScript code on worker");

    match worker.eval::<Value>(execution_code) {
        Ok(result) => {
            info!("JavaScript execution completed successfully");
            Ok(Some(result))
        }
        Err(e) => {
            error!("JavaScript execution failed: {}", e);
            Err(Box::new(e))
        }
    }
}

fn execute_filter_on_worker(
    worker: &DefaultWorker,
    inputs: &Value,
    config: &Value,
) -> WorkerResult {
    // Extract filter configuration
    let filter_logic = config
        .get("filter_logic")
        .and_then(|f| f.as_str())
        .unwrap_or("true"); // Default to pass-through

    // Create the filter execution context
    let execution_code = format!(
        r#"
        const inputs = {};
        const config = {};
        
        // Execute the filter logic
        const filterResult = (function() {{
            {}
        }})();
        
        // Return the inputs if filter passes, null if it doesn't
        filterResult ? inputs : null;
        "#,
        serde_json::to_string(inputs)?,
        serde_json::to_string(config)?,
        filter_logic
    );

    info!("Executing filter logic on worker");

    match worker.eval::<Value>(execution_code) {
        Ok(result) => {
            info!("Filter execution completed successfully");
            Ok(Some(result))
        }
        Err(e) => {
            error!("Filter execution failed: {}", e);
            Err(Box::new(e))
        }
    }
}

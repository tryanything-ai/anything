use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{error, info, instrument, warn};

mod js_executor {
    tonic::include_proto!("js_executor");
}

use js_executor::{
    js_executor_server::{JsExecutor, JsExecutorServer},
    ExecuteRequest, ExecuteResponse, HealthRequest, HealthResponse,
};

mod javascript_engine;
use javascript_engine::JavaScriptEngine;

#[derive(Debug)]
pub struct JsExecutorService {
    engine: Arc<JavaScriptEngine>,
    start_time: Instant,
    execution_counter: AtomicU64,
    active_executions: AtomicU32,
    max_concurrent_executions: u32,
}

impl JsExecutorService {
    pub fn new() -> anyhow::Result<Self> {
        let engine = Arc::new(JavaScriptEngine::new()?);
        
        Ok(Self {
            engine,
            start_time: Instant::now(),
            execution_counter: AtomicU64::new(0),
            active_executions: AtomicU32::new(0),
            max_concurrent_executions: 50, // Max 50 concurrent executions
        })
    }
}

#[tonic::async_trait]
impl JsExecutor for JsExecutorService {
    #[instrument(skip(self, request))]
    async fn execute_java_script(
        &self,
        request: Request<ExecuteRequest>,
    ) -> Result<Response<ExecuteResponse>, Status> {
        let req = request.into_inner();
        let execution_id = if req.execution_id.is_empty() {
            format!("exec_{}", self.execution_counter.fetch_add(1, Ordering::Relaxed))
        } else {
            req.execution_id.clone()
        };

        info!("[{}] Starting JavaScript execution", execution_id);
        info!("[{}] Code length: {} chars", execution_id, req.code.len());

        // Check concurrent execution limit
        let current_executions = self.active_executions.load(Ordering::Relaxed);
        if current_executions >= self.max_concurrent_executions {
            warn!("[{}] Too many concurrent executions ({}), rejecting", execution_id, current_executions);
            return Ok(Response::new(ExecuteResponse {
                success: false,
                result_json: String::new(),
                error_message: "Server overloaded: too many concurrent JavaScript executions".to_string(),
                error_type: "ResourceExhausted".to_string(),
                error_stack: String::new(),
                execution_time_ms: 0,
                execution_id,
            }));
        }

        self.active_executions.fetch_add(1, Ordering::Relaxed);
        let start = Instant::now();

        // Execute JavaScript in a blocking task (tonic handles this efficiently)
        let engine = self.engine.clone();
        let code = req.code.clone();
        let inputs_json = req.inputs_json.clone();
        let timeout_ms = req.timeout_ms;
        let exec_id = execution_id.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            engine.execute_javascript(&code, &inputs_json, timeout_ms, &exec_id)
        }).await;

        let execution_time = start.elapsed().as_millis() as u64;
        self.active_executions.fetch_sub(1, Ordering::Relaxed);

        let response = match result {
            Ok(Ok(result_json)) => {
                info!("[{}] Execution completed successfully in {}ms", execution_id, execution_time);
                ExecuteResponse {
                    success: true,
                    result_json,
                    error_message: String::new(),
                    error_type: String::new(),
                    error_stack: String::new(),
                    execution_time_ms: execution_time,
                    execution_id,
                }
            }
            Ok(Err(e)) => {
                error!("[{}] Execution failed in {}ms: {}", execution_id, execution_time, e);
                ExecuteResponse {
                    success: false,
                    result_json: String::new(),
                    error_message: e.to_string(),
                    error_type: "ExecutionError".to_string(),
                    error_stack: String::new(),
                    execution_time_ms: execution_time,
                    execution_id,
                }
            }
            Err(e) => {
                error!("[{}] Task spawn failed in {}ms: {}", execution_id, execution_time, e);
                ExecuteResponse {
                    success: false,
                    result_json: String::new(),
                    error_message: format!("Task execution failed: {}", e),
                    error_type: "InternalError".to_string(),
                    error_stack: String::new(),
                    execution_time_ms: execution_time,
                    execution_id,
                }
            }
        };

        Ok(Response::new(response))
    }

    async fn health_check(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let uptime_ms = self.start_time.elapsed().as_millis() as u64;
        let active_executions = self.active_executions.load(Ordering::Relaxed);

        let response = HealthResponse {
            healthy: true,
            version: "1.0.0".to_string(),
            uptime_ms,
            active_executions,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("🦀 Starting Rust JavaScript Executor with Deno Core (No-Tokio Mode)");

    let addr = "0.0.0.0:50051".parse()?;
    let js_executor = JsExecutorService::new()?;

    info!("🚀 gRPC JavaScript Executor listening on {}", addr);

    Server::builder()
        .add_service(JsExecutorServer::new(js_executor))
        .serve(addr)
        .await?;

    Ok(())
} 
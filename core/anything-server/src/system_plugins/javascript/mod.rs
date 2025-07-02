use serde_json::Value;
use std::time::Duration;
use tokio::time::Instant;
use tonic::transport::Channel;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

// Generated gRPC client code
pub mod js_executor {
    tonic::include_proto!("js_executor");
}

use js_executor::{js_executor_client::JsExecutorClient, ExecuteRequest, HealthRequest};

/// gRPC-based JavaScript task processor using Rust Deno executor
/// This replaces RustyScript with a separate containerized service
#[instrument(skip(bundled_inputs, bundled_plugin_config))]
pub async fn process_js_task(
    bundled_inputs: &Value,
    bundled_plugin_config: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let execution_id = Uuid::new_v4().to_string();

    info!(
        "[JS_GRPC] Starting JavaScript task execution: {}",
        execution_id
    );

    // Extract JavaScript code
    let js_code = match bundled_plugin_config["code"].as_str() {
        Some(code) => {
            info!("[JS_GRPC] Extracted JS code, length: {} chars", code.len());
            code
        }
        None => {
            error!("[JS_GRPC] No JavaScript code found in configuration");
            return Err("JavaScript code not found in task configuration".into());
        }
    };

    // Prepare execution context
    let input_size = serde_json::to_string(bundled_inputs)
        .map(|s| s.len())
        .unwrap_or(0);

    info!("[JS_GRPC] Input data size: {} bytes", input_size);

    // Create gRPC client connection
    let mut js_manager = JsExecutorManager::new().await?;
    let js_client = js_manager.get_client().await;

    // Execute JavaScript via gRPC
    let result = execute_javascript_grpc(js_client, js_code, bundled_inputs, &execution_id).await?;

    let total_duration = start.elapsed();
    info!(
        "[JS_GRPC] JavaScript task completed successfully in {:?}",
        total_duration
    );

    Ok(Some(result))
}

/// Execute JavaScript via gRPC call to Rust Deno executor
pub async fn execute_javascript_grpc(
    client: &mut JsExecutorClient<Channel>,
    js_code: &str,
    inputs: &Value,
    execution_id: &str,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    info!("[JS_GRPC] Sending gRPC request to Rust Deno executor");

    let inputs_json = serde_json::to_string(inputs)?;

    let request = tonic::Request::new(ExecuteRequest {
        code: js_code.to_string(),
        inputs_json,
        timeout_ms: 30000, // 30 second timeout
        execution_id: execution_id.to_string(),
    });

    let response = client.execute_java_script(request).await?;
    let result = response.into_inner();

    if result.success {
        info!(
            "[JS_GRPC] JavaScript executed successfully in {}ms",
            result.execution_time_ms
        );

        // Parse the result JSON
        let parsed_result: Value = serde_json::from_str(&result.result_json)?;
        log_result_info(&parsed_result);
        Ok(parsed_result)
    } else {
        error!(
            "[JS_GRPC] JavaScript execution failed: {} ({})",
            result.error_message, result.error_type
        );
        Err(format!("{}: {}", result.error_type, result.error_message).into())
    }
}

/// JavaScript executor client manager
pub struct JsExecutorManager {
    client: JsExecutorClient<Channel>,
}

impl JsExecutorManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let js_executor_url = std::env::var("JS_EXECUTOR_URL")
            .unwrap_or_else(|_| "http://js-executor:50051".to_string());

        info!(
            "[JS_GRPC] Connecting to JavaScript executor at {}",
            js_executor_url
        );

        let channel = Channel::from_shared(js_executor_url)?
            .timeout(Duration::from_secs(60))
            .connect()
            .await?;

        let client = JsExecutorClient::new(channel);

        Ok(Self { client })
    }

    pub async fn get_client(&mut self) -> &mut JsExecutorClient<Channel> {
        &mut self.client
    }

    pub async fn health_check(&mut self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let request = tonic::Request::new(HealthRequest {});

        match self.client.health_check(request).await {
            Ok(response) => {
                let health = response.into_inner();
                info!(
                    "[JS_GRPC] Health check successful - uptime: {}ms, active executions: {}",
                    health.uptime_ms, health.active_executions
                );
                Ok(health.healthy)
            }
            Err(e) => {
                warn!("[JS_GRPC] Health check failed: {}", e);
                Ok(false)
            }
        }
    }
}

/// Log detailed information about the execution result
fn log_result_info(result: &Value) {
    let result_type = match result {
        Value::Object(_) => "object",
        Value::Array(_) => "array",
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
    };

    let result_size = serde_json::to_string(result).map(|s| s.len()).unwrap_or(0);

    info!(
        "[JS_GRPC] Result type: {}, size: {} bytes",
        result_type, result_size
    );

    // Log object structure for debugging (but not the full content)
    if let Value::Object(obj) = result {
        let keys: Vec<&String> = obj.keys().collect();
        info!("[JS_GRPC] Result object keys: {:?}", keys);
    }
}

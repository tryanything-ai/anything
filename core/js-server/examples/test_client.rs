use js_executor::{
    js_executor_client::JsExecutorClient,
    ExecuteRequest, HealthRequest,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the JavaScript executor
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    let mut client = JsExecutorClient::new(channel);

    println!("🧪 Testing JavaScript Executor gRPC Server");
    println!("==========================================");

    // Test 1: Health Check
    println!("\n1. Health Check");
    let health_request = tonic::Request::new(HealthRequest {});
    let health_response = client.health_check(health_request).await?;
    let health = health_response.into_inner();
    println!("   ✅ Server is healthy: {}", health.healthy);
    println!("   📊 Version: {}", health.version);
    println!("   ⏱️  Uptime: {}ms", health.uptime_ms);
    println!("   🔄 Active executions: {}", health.active_executions);

    // Test 2: Simple JavaScript execution
    println!("\n2. Simple JavaScript Execution");
    let code = "return inputs.value * 2;";
    let inputs = json!({"value": 21});
    
    let request = tonic::Request::new(ExecuteRequest {
        code: code.to_string(),
        inputs_json: inputs.to_string(),
        timeout_ms: 5000,
        execution_id: "test_simple".to_string(),
    });

    let response = client.execute_java_script(request).await?;
    let result = response.into_inner();
    
    if result.success {
        println!("   ✅ Execution successful");
        println!("   📊 Result: {}", result.result_json);
        println!("   ⏱️  Execution time: {}ms", result.execution_time_ms);
    } else {
        println!("   ❌ Execution failed: {}", result.error_message);
    }

    // Test 3: Complex JavaScript execution
    println!("\n3. Complex JavaScript Execution");
    let complex_code = r#"
        const data = inputs.items.map(item => ({
            ...item,
            doubled: item.value * 2,
            processed: true
        }));
        
        return {
            original_count: inputs.items.length,
            processed_data: data,
            total_sum: data.reduce((sum, item) => sum + item.doubled, 0)
        };
    "#;
    
    let complex_inputs = json!({
        "items": [
            {"id": 1, "value": 10, "name": "Item 1"},
            {"id": 2, "value": 20, "name": "Item 2"},
            {"id": 3, "value": 30, "name": "Item 3"}
        ]
    });
    
    let request = tonic::Request::new(ExecuteRequest {
        code: complex_code.to_string(),
        inputs_json: complex_inputs.to_string(),
        timeout_ms: 5000,
        execution_id: "test_complex".to_string(),
    });

    let response = client.execute_java_script(request).await?;
    let result = response.into_inner();
    
    if result.success {
        println!("   ✅ Complex execution successful");
        println!("   📊 Result: {}", result.result_json);
        println!("   ⏱️  Execution time: {}ms", result.execution_time_ms);
    } else {
        println!("   ❌ Complex execution failed: {}", result.error_message);
    }

    // Test 4: Error handling
    println!("\n4. Error Handling Test");
    let error_code = "throw new Error('This is a test error');";
    let error_inputs = json!({});
    
    let request = tonic::Request::new(ExecuteRequest {
        code: error_code.to_string(),
        inputs_json: error_inputs.to_string(),
        timeout_ms: 5000,
        execution_id: "test_error".to_string(),
    });

    let response = client.execute_java_script(request).await?;
    let result = response.into_inner();
    
    if !result.success {
        println!("   ✅ Error handling works correctly");
        println!("   📊 Error: {}", result.error_message);
        println!("   🏷️  Error type: {}", result.error_type);
        println!("   ⏱️  Execution time: {}ms", result.execution_time_ms);
    } else {
        println!("   ❌ Expected error but execution succeeded");
    }

    println!("\n🎉 All tests completed!");
    Ok(())
} 
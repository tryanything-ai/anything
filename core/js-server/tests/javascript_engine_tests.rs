use js_executor::JavaScriptEngine;
use serde_json::json;

#[test]
fn test_simple_javascript_execution() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        return inputs.value * 2;
    "#;

    let inputs = json!({
        "value": 21
    });

    let result = engine
        .execute_javascript(code, &inputs.to_string(), 5000, "test_simple")
        .expect("Execution failed");

    let parsed_result: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed_result, json!(42));
}

#[test]
fn test_object_return() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        return {
            original: inputs.data,
            processed: true,
            timestamp: new Date().toISOString(),
            count: inputs.data.length
        };
    "#;

    let inputs = json!({
        "data": [1, 2, 3, 4, 5]
    });

    let result = engine
        .execute_javascript(code, &inputs.to_string(), 5000, "test_object")
        .expect("Execution failed");

    let parsed_result: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed_result["original"], json!([1, 2, 3, 4, 5]));
    assert_eq!(parsed_result["processed"], json!(true));
    assert_eq!(parsed_result["count"], json!(5));
    assert!(parsed_result["timestamp"].is_string());
}

#[test]
fn test_console_output() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        console.log("Hello from JavaScript!");
        console.error("This is an error message");
        console.warn("This is a warning");
        return "completed";
    "#;

    let inputs = json!({});

    let result = engine
        .execute_javascript(code, &inputs.to_string(), 5000, "test_console")
        .expect("Execution failed");

    let parsed_result: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed_result, json!("completed"));
}

#[test]
fn test_error_handling() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        throw new Error("This is a test error");
    "#;

    let inputs = json!({});

    let result = engine.execute_javascript(code, &inputs.to_string(), 5000, "test_error");

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("This is a test error"));
}

#[test]
fn test_undefined_return_error() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        // This doesn't return anything explicitly
        const value = 42;
    "#;

    let inputs = json!({});

    let result = engine.execute_javascript(code, &inputs.to_string(), 5000, "test_undefined");

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("must explicitly return a value"));
}

#[test]
fn test_timeout_handling() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        // This will run for a long time
        let start = Date.now();
        while (Date.now() - start < 10000) {
            // Busy wait for 10 seconds
        }
        return "Should not reach here";
    "#;

    let inputs = json!({});

    let result = engine.execute_javascript(
        code,
        &inputs.to_string(),
        1000, // 1 second timeout
        "test_timeout",
    );

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("timed out"));
}

#[test]
fn test_concurrent_executions() {
    use std::sync::Arc;
    use std::thread;

    let engine = Arc::new(JavaScriptEngine::new().expect("Failed to create engine"));

    let mut handles = vec![];

    // Spawn 10 concurrent executions
    for i in 0..10 {
        let engine_clone = engine.clone();
        let handle = thread::spawn(move || {
            let code = r#"
                return {
                    execution_id: inputs.id,
                    result: inputs.value * 2,
                    timestamp: Date.now()
                };
            "#;

            let inputs = json!({
                "id": i,
                "value": i * 10
            });

            engine_clone.execute_javascript(
                code,
                &inputs.to_string(),
                5000,
                &format!("concurrent_{}", i),
            )
        });
        handles.push(handle);
    }

    // Wait for all executions to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all executions succeeded
    for (i, result) in results.into_iter().enumerate() {
        let execution_result = result.expect("Execution failed");
        let parsed: serde_json::Value = serde_json::from_str(&execution_result).unwrap();
        assert_eq!(parsed["execution_id"], json!(i));
        assert_eq!(parsed["result"], json!(i * 20));
    }
}

#[test]
fn test_json_manipulation() {
    let engine = JavaScriptEngine::new().expect("Failed to create engine");

    let code = r#"
        const processedData = inputs.items.map(item => ({
            ...item,
            processed: true,
            doubled: item.value * 2
        }));
        
        return {
            original_count: inputs.items.length,
            processed_data: processedData,
            total_doubled: processedData.reduce((sum, item) => sum + item.doubled, 0)
        };
    "#;

    let inputs = json!({
        "items": [
            {"id": 1, "value": 10},
            {"id": 2, "value": 20},
            {"id": 3, "value": 30}
        ]
    });

    let result = engine
        .execute_javascript(code, &inputs.to_string(), 5000, "test_json")
        .expect("Execution failed");

    let parsed_result: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed_result["original_count"], json!(3));
    assert_eq!(parsed_result["total_doubled"], json!(120)); // (10+20+30) * 2
    assert_eq!(parsed_result["processed_data"][0]["doubled"], json!(20));
}

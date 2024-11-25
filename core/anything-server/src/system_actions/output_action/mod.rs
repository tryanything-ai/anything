use std::sync::Arc;

use serde_json::Value;

use crate::AppState;

pub async fn process_response_task(
    state: Arc<AppState>,
    flow_session_id: String,
    bundled_context: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS RESPONSE] Starting process_response_task");
    println!(
        "[PROCESS RESPONSE] Initial bundled context: {:?}",
        bundled_context
    );

    // Helper function to recursively clean and parse JSON strings
    fn deep_parse_json(input: &str) -> Result<Value, serde_json::Error> {
        // First try parsing directly
        match serde_json::from_str(input) {
            Ok(mut parsed) => {
                // Recursively clean any string values that might be JSON
                fn clean_recursive(value: &mut Value) {
                    match value {
                        Value::Object(map) => {
                            for (_, v) in map.iter_mut() {
                                clean_recursive(v);
                            }
                        }
                        Value::Array(arr) => {
                            for v in arr.iter_mut() {
                                clean_recursive(v);
                            }
                        }
                        Value::String(s) => {
                            if let Ok(parsed) = deep_parse_json(s) {
                                *value = parsed;
                            }
                        }
                        _ => {}
                    }
                }
                clean_recursive(&mut parsed);
                Ok(parsed)
            }
            Err(_) => {
                // If direct parsing fails, try cleaning the string
                let cleaned = input
                    .replace("\\n", "\n")
                    .replace("\\\"", "\"")
                    .replace("\\/", "/");

                // Try parsing cleaned string
                match serde_json::from_str(&cleaned) {
                    Ok(mut parsed) => {
                        // Apply same recursive cleaning to cleaned parse result
                        fn clean_recursive(value: &mut Value) {
                            match value {
                                Value::Object(map) => {
                                    for (_, v) in map.iter_mut() {
                                        clean_recursive(v);
                                    }
                                }
                                Value::Array(arr) => {
                                    for v in arr.iter_mut() {
                                        clean_recursive(v);
                                    }
                                }
                                Value::String(s) => {
                                    if let Ok(parsed) = deep_parse_json(s) {
                                        *value = parsed;
                                    }
                                }
                                _ => {}
                            }
                        }
                        clean_recursive(&mut parsed);
                        Ok(parsed)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    // Get the required fields from the bundled context
    let status_code = bundled_context
        .get("status_code")
        .and_then(|v| v.as_str())
        .unwrap_or("200");

    let content_type = bundled_context
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("application/json");

    let headers = bundled_context
        .get("headers")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let body = bundled_context
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Build response object
    let mut response = serde_json::Map::new();
    response.insert(
        "status_code".to_string(),
        Value::String(status_code.to_string()),
    );

    // Build headers map
    let mut headers_map = serde_json::Map::new();
    headers_map.insert(
        "content-type".to_string(),
        Value::String(content_type.to_string()),
    );

    // Parse and add additional headers if present
    if !headers.is_empty() {
        match deep_parse_json(headers) {
            Ok(Value::Object(parsed_headers)) => {
                for (key, value) in parsed_headers {
                    headers_map.insert(key, value);
                }
            }
            Ok(Value::Null) | Ok(Value::Bool(_)) | Ok(Value::Number(_)) | Ok(Value::String(_))
            | Ok(Value::Array(_)) => {
                println!("[PROCESS RESPONSE] Headers must be a JSON object");
            }
            Err(e) => {
                println!("[PROCESS RESPONSE] Failed to parse headers: {}", e);
            }
        }
    }

    response.insert("headers".to_string(), Value::Object(headers_map));

    // Parse and add body if present
    if !body.is_empty() {
        if content_type == "application/json" {
            match deep_parse_json(body) {
                Ok(parsed_body) => {
                    response.insert("body".to_string(), parsed_body);
                }
                Err(e) => {
                    println!("[PROCESS RESPONSE] Failed to parse JSON body: {}", e);
                    response.insert("body".to_string(), Value::String(body.to_string()));
                }
            }
        } else {
            response.insert("body".to_string(), Value::String(body.to_string()));
        }
    }

    println!("[PROCESS RESPONSE] Generated response: {:?}", response);

    // Send the response through the flow_completions channel
    let mut completions = state.flow_completions.lock().await;
    if let Some(completion) = completions.remove(&flow_session_id) {
        if completion.needs_response {
            println!("[PROCESS RESPONSE] Sending result through completion channel");
            let _ = completion.sender.send(Value::Object(response.clone()));
        }
    }

    Ok(Value::Object(response))
}

use std::sync::Arc;

use serde_json::Value;

use crate::AppState;

pub async fn process_response_task(
    state: Arc<AppState>,
    flow_session_id: String,
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS RESPONSE] Starting process_response_task");
    println!(
        "[PROCESS RESPONSE] Initial bundled context: {:?}",
        bundled_context
    );

    // Deep parse JSON to handle common escape issues helpful for all the dirty json we have
    fn deep_parse_json(input: &str) -> Result<Value, serde_json::Error> {
        // Try multiple parsing strategies in order
        let attempts = [
            // 1. Try parsing directly first
            input.to_string(),
            // 2. If wrapped in quotes, remove them and unescape
            if input.starts_with('"') && input.ends_with('"') {
                let inner = &input[1..input.len() - 1];
                inner
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\/", "/")
                    .replace("\\\\", "\\")
            } else {
                input.to_string()
            },
            // 3. Try cleaning common escape issues
            input
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\/", "/")
                .replace("\\\\", "\\"),
        ];

        // Try each cleaning strategy
        for (i, attempt) in attempts.iter().enumerate() {
            if let Ok(mut parsed) = serde_json::from_str(attempt) {
                println!(
                    "[DEEP PARSE JSON IN RESPONSE] Successfully parsed JSON using strategy {}",
                    i + 1
                );

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
                            // Use Ok() to unwrap the Result, falling back to the original string
                            if let Ok(parsed) = deep_parse_json(s) {
                                *value = parsed;
                            }
                        }
                        _ => {}
                    }
                }

                clean_recursive(&mut parsed);
                return Ok(parsed);
            }
        }

        // If all parsing attempts fail, return the original input as a string Value
        Ok(Value::String(input.to_string()))
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

    // Get body based on content type
    let body = match content_type {
        "application/json" => bundled_context
            .get("json_body")
            .and_then(|v| v.as_str())
            .unwrap_or("{}"),
        "text/plain" => bundled_context
            .get("text_body")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        "text/html" => bundled_context
            .get("html_body")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        "text/xml" => bundled_context
            .get("xml_body")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        _ => bundled_context
            .get("json_body")
            .and_then(|v| v.as_str())
            .unwrap_or("{}"),
    };

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

    Ok(Some(Value::Object(response)))
}

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::Instant;

fn is_binary_content(content_type: &str) -> bool {
    let content_type = content_type.split(';').next().unwrap_or("").trim();
    content_type.starts_with("image/")
        || content_type.starts_with("application/pdf")
        || content_type.starts_with("application/msword")
        || content_type.starts_with("application/vnd.openxmlformats-")
        || content_type.starts_with("application/zip")
        || content_type.starts_with("application/x-rar")
        || content_type == "application/octet-stream"
}

fn format_binary_response(content_type: &str, bytes: &[u8]) -> Value {
    let base64_data = base64::encode(bytes);

    // For images, return a complete data URL that can be used directly in <img> tags
    if content_type.starts_with("image/") {
        serde_json::json!({
            "type": "image",
            "content_type": content_type,
            "size": bytes.len(),
            "data": format!("data:{};base64,{}", content_type, base64_data),
        })
    } else {
        // For other binary files, return just the base64 data
        serde_json::json!({
            "type": "binary",
            "content_type": content_type,
            "size": bytes.len(),
            "data": base64_data,
        })
    }
}

pub async fn process_http_task(
    http_client: &Client,
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    println!("[TASK_ENGINE] Entering process_http_task");
    println!("[TASK_ENGINE] Bundled context: {:?}", bundled_context);

    if let (Some(method), Some(url)) = (
        bundled_context.get("method").and_then(Value::as_str),
        bundled_context.get("url").and_then(Value::as_str),
    ) {
        println!(
            "[TASK_ENGINE] Processing HTTP task with method: {}, url: {}",
            method, url
        );
        let method = match method.to_uppercase().as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            "HEAD" => reqwest::Method::HEAD,
            "OPTIONS" => reqwest::Method::OPTIONS,
            "PATCH" => reqwest::Method::PATCH,
            _ => {
                println!("[TASK_ENGINE] Unsupported HTTP method: {}", method);
                return Err(format!("Unsupported HTTP method: {}", method).into());
            }
        };

        let mut request_builder = http_client.request(method.clone(), url);

        println!("[TASK_ENGINE] Processing headers");
        let headers = parse_headers(bundled_context);
        for (key, value) in headers {
            println!("[TASK_ENGINE] Adding header: {} = {}", key, value);
            request_builder = request_builder.header(key, value);
        }

        let should_skip_empty = matches!(
            method,
            reqwest::Method::GET | reqwest::Method::HEAD | reqwest::Method::OPTIONS
        );

        if let Some(body) = bundled_context.get("body") {
            let is_empty = match body {
                Value::String(s) => {
                    // Check if string is empty, "{}", or just whitespace
                    let trimmed = s.trim();
                    trimmed.is_empty() || trimmed == "{}"
                }
                Value::Object(obj) => obj.is_empty(),
                _ => {
                    println!("[TASK_ENGINE] Body is not a string or an object");
                    return Err("HTTP task body must be a string or an object".into());
                }
            };

            if !is_empty || !should_skip_empty {
                let body_str = match body {
                    Value::String(s) => s.to_string(),
                    Value::Object(obj) => serde_json::to_string(obj)?,
                    _ => unreachable!(), // We already checked above
                };
                println!("[TASK_ENGINE] Adding body: {}", body_str);
                request_builder = request_builder.body(body_str);
            } else {
                println!("[TASK_ENGINE] Skipping empty body for {} request", method);
            }
        } else {
            println!("[TASK_ENGINE] No body found in task context");
        }

        println!("[TASK_ENGINE] Sending HTTP request");
        let request_start = Instant::now();

        let response = request_builder.send().await?;
        println!("[SPEED] HTTP request took {:?}", request_start.elapsed());

        let status = response.status();
        let headers = response.headers().clone();

        let content_type = headers
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        // Check file size
        const MAX_SIZE: u64 = 10 * 1024 * 1024; // 10MB
        if let Some(length) = headers
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
        {
            if length > MAX_SIZE {
                return Err(format!(
                    "File size {} bytes exceeds maximum allowed size of {} bytes",
                    length, MAX_SIZE
                )
                .into());
            }
        }

        let body = if is_binary_content(content_type) {
            let bytes = response.bytes().await?;

            let filename = headers
                .get("content-disposition")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| {
                    s.split(';')
                        .find(|part| part.trim().starts_with("filename="))
                        .map(|part| part.trim()[9..].trim_matches('"').to_string())
                });

            let mut response = format_binary_response(content_type, &bytes);

            // Add filename if available
            if let Some(name) = filename {
                response
                    .as_object_mut()
                    .unwrap()
                    .insert("filename".to_string(), Value::String(name));
            }

            response
        } else {
            // Handle text/JSON response
            let text = response.text().await?;
            match serde_json::from_str::<Value>(&text) {
                Ok(json_value) => serde_json::json!({
                    "type": "json",
                    "data": json_value
                }),
                Err(_) => serde_json::json!({
                    "type": "text",
                    "data": text
                }),
            }
        };

        let result = serde_json::json!({
            "status_code": status.as_u16(),
            "headers": headers
                .iter()
                .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>(),
            "body": body
        });

        println!("[TASK_ENGINE] Returning result: {:?}", result);
        println!(
            "[SPEED] Total HTTP task processing took {:?}",
            start.elapsed()
        );
        Ok(Some(result))
    } else {
        println!("[TASK_ENGINE] Missing required fields (method, url) in task context");
        Err("HTTP Missing required fields (method, url) in task context.".into())
    }
}

pub fn parse_headers(bundled_context: &Value) -> Vec<(String, String)> {
    println!("[TASK_ENGINE] Processing headers");
    let mut headers = Vec::new();

    if let Some(headers_value) = bundled_context.get("headers") {
        match headers_value {
            Value::Object(headers_obj) => {
                println!("[TASK_ENGINE] Headers are an object: {:?}", headers_obj);
                for (key, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        println!("[TASK_ENGINE] Adding header: {} = {}", key, value_str);
                        headers.push((key.to_string(), value_str.to_string()));
                    }
                }
            }
            Value::String(headers_str) => {
                println!("[TASK_ENGINE] Headers are a string: {}", headers_str);
                match serde_json::from_str::<Value>(headers_str) {
                    Ok(Value::Object(parsed_headers)) => {
                        println!("[TASK_ENGINE] Parsed headers: {:?}", parsed_headers);
                        for (key, value) in parsed_headers {
                            if let Some(value_str) = value.as_str() {
                                println!("[TASK_ENGINE] Adding header: {} = {}", key, value_str);
                                headers.push((key.to_string(), value_str.to_string()));
                            }
                        }
                    }
                    _ => {
                        println!("[TASK_ENGINE] Failed to parse headers string as JSON object")
                    }
                }
            }
            _ => println!("[TASK_ENGINE] Headers are neither an object nor a string"),
        }
    } else {
        println!("[TASK_ENGINE] No headers found in bundled context");
    }

    headers
}

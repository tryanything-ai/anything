use tokio::time::Instant;

use std::collections::HashMap;

use reqwest::Client;

use serde_json::{json, Value};

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

        let mut request_builder = http_client.request(method, url);

        println!("[TASK_ENGINE] Processing headers");
        if let Some(headers) = bundled_context.get("headers") {
            match headers {
                Value::Object(headers_obj) => {
                    println!("[TASK_ENGINE] Headers are an object: {:?}", headers_obj);
                    for (key, value) in headers_obj {
                        if let Some(value_str) = value.as_str() {
                            println!("[TASK_ENGINE] Adding header: {} = {}", key, value_str);
                            request_builder = request_builder.header(key.as_str(), value_str);
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
                                    println!(
                                        "[TASK_ENGINE] Adding header: {} = {}",
                                        key, value_str
                                    );
                                    request_builder =
                                        request_builder.header(key.as_str(), value_str);
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

        if let Some(body) = bundled_context.get("body") {
            if let Some(body_str) = body.as_str() {
                if !body_str.is_empty() {
                    println!("[TASK_ENGINE] Adding body: {}", body_str);
                    request_builder = request_builder.body(body_str.to_string());
                } else {
                    println!("[TASK_ENGINE] Body is an empty string, sending request without body");
                }
            } else if let Some(body_object) = body.as_object() {
                let body_json = serde_json::to_string(body_object)?;
                println!("[TASK_ENGINE] Adding body: {}", body_json);
                request_builder = request_builder.body(body_json);
            } else {
                println!("[TASK_ENGINE] Body is not a string or an object");
                return Err("HTTP task body must be a string or an object".into());
            }
        } else {
            println!("[TASK_ENGINE] No body found in task context");
        }

        println!("[TASK_ENGINE] Sending HTTP request");
        let request_start = Instant::now();
        let response = request_builder.send().await?;
        println!("[SPEED] HTTP request took {:?}", request_start.elapsed());
        println!(
            "[TASK_ENGINE] HTTP request response received: {:?}",
            response
        );
        let status = response.status();
        let headers = response.headers().clone();
        let _content_type = response
            .headers()
            .get("content-type")
            .map(|v| v.to_str().unwrap_or(""));

        // Try to parse the response as JSON, if it fails, return the raw text
        let body = match response.text().await {
            Ok(text) => {
                println!("[TASK_ENGINE] Response text: {}", text);
                match serde_json::from_str::<Value>(&text) {
                    Ok(json_value) => {
                        println!(
                            "[TASK_ENGINE] HTTP request successful. JSON Response: {:?}",
                            json_value
                        );
                        json_value
                    }
                    Err(_) => {
                        println!(
                            "[TASK_ENGINE] HTTP request successful. Text Response: {}",
                            text
                        );
                        Value::String(text)
                    }
                }
            }
            Err(e) => {
                println!("[TASK_ENGINE] Error reading response body: {:?}", e);
                return Err(e.into());
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

use std::sync::Arc;

use serde_json::Value;

use crate::system_plugins::webhook_response::deep_parse_json;
use crate::AppState;

//TODO: Something in here I thik is what makes it so we can only support returning JSON from webhooks.
//For now just going to make webhooks only return json
pub async fn process_tool_call_result_task(
    state: Arc<AppState>,
    flow_session_id: String,
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS RESPONSE] Starting process_response_task");
    println!(
        "[PROCESS RESPONSE] Initial bundled context: {:?}",
        bundled_context
    );

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
    let tool_call_result = match content_type {
        "application/json" => bundled_context
            .get("json_body")
            .map(|v| v.to_string())
            .unwrap_or_else(|| "{}".to_string()),
        "text/plain" => bundled_context
            .get("text_body")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        "text/html" => bundled_context
            .get("html_body")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        "text/xml" => bundled_context
            .get("xml_body")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        _ => bundled_context
            .get("json_body")
            .and_then(|v| v.as_str())
            .unwrap_or("{}")
            .to_string(),
    };

    println!("[PROCESS RESPONSE] Status code: {}", status_code);
    println!("[PROCESS RESPONSE] Content type: {}", content_type);
    println!("[PROCESS RESPONSE] Headers: {}", headers);
    println!("[PROCESS RESPONSE] Body: {}", tool_call_result);

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
        Value::String("application/json".to_string()),
    );

    response.insert("headers".to_string(), Value::Object(headers_map));

    // Parse and add body if present
    if !tool_call_result.is_empty() {
        let mut body_map = serde_json::Map::new();
        let mut results_array = Vec::new();
        let mut result_object = serde_json::Map::new();

        if content_type == "application/json" {
            match deep_parse_json(&tool_call_result) {
                Ok(parsed_tool_call_response) => {
                    // result_object.insert("toolCallId".to_string(), Value::String("1".to_string())); // TODO: Get actual tool call ID
                    result_object.insert("result".to_string(), parsed_tool_call_response);
                }
                Err(e) => {
                    println!("[PROCESS RESPONSE] Failed to parse JSON body: {}", e);
                    // result_object.insert("toolCallId".to_string(), Value::String("1".to_string())); // TODO: Get actual tool call ID
                    result_object.insert(
                        "result".to_string(),
                        Value::String(tool_call_result.to_string()),
                    );
                }
            }
        } else {
            // result_object.insert("toolCallId".to_string(), Value::String("1".to_string())); // TODO: Get actual tool call ID
            result_object.insert(
                "result".to_string(),
                Value::String(tool_call_result.to_string()),
            );
        }

        results_array.push(Value::Object(result_object));
        body_map.insert("results".to_string(), Value::Array(results_array));
        response.insert("body".to_string(), Value::Object(body_map));
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

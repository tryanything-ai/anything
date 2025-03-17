use std::sync::Arc;
use uuid::Uuid;

use serde_json::Value;

use crate::system_plugins::webhook_response::deep_parse_json;
use crate::AppState;

//TODO: Something in here I thik is what makes it so we can only support returning JSON from webhooks.
//For now just going to make webhooks only return json
pub async fn process_tool_call_result_task(
    state: Arc<AppState>,
    flow_session_id: Uuid,
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS RESPONSE] Starting process_response_task");
    println!(
        "[PROCESS RESPONSE] Initial bundled context: {:?}",
        bundled_context
    );

    let content_type = bundled_context
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("application/json");

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

    // println!("[PROCESS RESPONSE] Status code: {}", status_code);
    // println!("[PROCESS RESPONSE] Content type: {}", content_type);
    // println!("[PROCESS RESPONSE] Headers: {}", headers);
    println!("[PROCESS RESPONSE] Body: {}", tool_call_result);

    // Build response object
    let mut response = serde_json::Map::new();

    let mut result_object = serde_json::Map::new();
    result_object.insert(
        "result".to_string(),
        Value::String(tool_call_result.to_string()),
    );

    response.insert(
        "results".to_string(),
        Value::Array(vec![Value::Object(result_object)]),
    );

    println!("[PROCESS RESPONSE] Generated response: {:?}", response);

    // Send the response through the flow_completions channel
    let mut completions = state.flow_completions.lock().await;
    if let Some(completion) = completions.remove(&flow_session_id.to_string()) {
        if completion.needs_response {
            println!("[PROCESS RESPONSE] Sending result through completion channel");
            let _ = completion.sender.send(Value::Object(response.clone()));
        }
    }

    Ok(Some(Value::Object(response)))
}

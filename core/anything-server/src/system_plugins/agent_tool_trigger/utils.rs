use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::{
    types::action_types::{Action, ActionType, PluginName},
    types::workflow_types::WorkflowVersionDefinition,
};

use serde_json::{json, Value};

pub fn parse_tool_call_request_to_result(body: Json<Value>) -> (Value, String) {
    println!(
        "[TOOL_CALLS] Parsing tool call request to result Body: {:?}",
        body
    );

    // Get the call data
    let call_data = body.get("message").and_then(|m| m.get("call")).cloned();

    // Navigate through the nested structure to get to tool_calls
    let tool_call = body
        .get("message")
        .and_then(|m| m.get("toolCalls"))
        .and_then(|tc| tc.get(0));

    println!("[TOOL_CALLS] Tool calls: {:?}", tool_call);

    if let Some(tool_call) = tool_call {
        let tool_call_id = tool_call
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default();

        println!("[TOOL_CALLS] Tool call ID: {}", tool_call_id);

        // Get the function arguments
        let arguments = tool_call
            .get("function")
            .and_then(|f| f.get("arguments"))
            .and_then(Value::as_str)
            .unwrap_or("{}");

        // Parse the arguments string into a Value
        let parsed_args = serde_json::from_str::<Value>(arguments).unwrap_or(json!({}));

        // Create the result object with both arguments and call data
        let result = json!({
            "arguments": parsed_args,
            "call": call_data
        });

        (result, tool_call_id.to_string())
    } else {
        println!("[TOOL_CALLS] No tool call found");
        (
            json!({"arguments": {}, "call": call_data}),
            String::new(),
        )
    }
}

//TODO: this should know about our new error system at some point where erors are in an error object not stored_result
pub fn parse_tool_response_into_api_response(
    tool_call_id: String,
    stored_result: Option<Value>,
    stored_error: Option<Value>,
) -> impl IntoResponse {
    println!("[TOOL_CALLS] Parsing tool response into API response");

    println!("[TOOL_CALLS] Stored result: {:?}", stored_result.clone());

    println!("[TOOL_CALLS] Tool call ID: {}", tool_call_id);
    // Check for error first
    if let Some(error) = stored_error {
        let error_message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown error occurred");
        println!("[TOOL_CALLS] Returning error response: {}", error_message);
        return (StatusCode::INTERNAL_SERVER_ERROR, error_message.to_string()).into_response();
    }

    let mut response = stored_result.clone().unwrap_or(json!({}));

    // Get the results array from the response
    if let Some(body) = response.get_mut("body") {
        if let Some(results) = body.get_mut("results") {
            if let Some(results_array) = results.as_array_mut() {
                if let Some(first_result) = results_array.get_mut(0) {
                    if let Some(result_obj) = first_result.as_object_mut() {
                        result_obj.insert("toolCallId".to_string(), Value::String(tool_call_id));
                    }
                }
            }
        }
    }

    println!("[TOOL_CALLS] Response: {:?}", response);

    (StatusCode::OK, Json(response)).into_response()
}

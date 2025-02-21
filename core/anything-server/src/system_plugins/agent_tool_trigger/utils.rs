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
    // Get the call data
    let call_data = body.get("message").and_then(|m| m.get("call")).cloned();

    // Navigate through the nested structure to get to tool_calls
    let tool_calls = body
        .get("message")
        .and_then(|m| m.get("tool_calls"))
        .and_then(|tc| tc.get(0));

    if let Some(tool_call) = tool_calls {
        let tool_call_id = tool_call
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default();

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
        (json!({"arguments": {}, "call": call_data}), String::new())
    }
}

//TODO: this should know about our new error system at some point where erors are in an error object not stored_result
pub fn parse_tool_response_into_api_response(
    tool_call_id: String,
    stored_result: Option<Value>,
    stored_error: Option<Value>,
) -> impl IntoResponse {
    println!("[TOOL_CALLS] Parsing tool response into API response");
    // Check for error first
    if let Some(error) = stored_error {
        let error_message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown error occurred");
        println!("[TOOL_CALLS] Returning error response: {}", error_message);
        return (StatusCode::INTERNAL_SERVER_ERROR, error_message.to_string()).into_response();
    }

    // let mut headers = HeaderMap::new();

    // Get content type from the stored result
    // let body = stored_result
    //     .as_ref()
    //     .and_then(|r| r.get("body"))
    //     .and_then(Value::as_str)
    //     .unwrap_or("body");

    // println!("[TOOL_CALLS] Response content type: {}", content_type);

    static EMPTY: Vec<Value> = Vec::new();
    let results = stored_result
        .as_ref()
        .and_then(|r| r.get("body"))
        .and_then(|b| b.get("results"))
        .and_then(|r| r.as_array())
        .unwrap_or(&EMPTY);

    println!("[TOOL_CALLS] Results: {:?}", results);

    // Get first result and add tool call ID
    let result = results.first().map(|r| r.clone()).unwrap_or(json!({}));

    // Convert to object if not already
    let result_obj = match result {
        Value::Object(mut obj) => {
            obj.insert("toolCallId".to_string(), Value::String(tool_call_id));
            Value::Object(obj)
        }
        _ => {
            // Wrap non-object value
            json!({
                "result": result,
                "toolCallId": tool_call_id
            })
        }
    };

    // Return the original object with enriched results
    let mut response = stored_result.unwrap_or(json!({}));

    if let Some(obj) = response.as_object_mut() {
        // Update the results array in the body with the enriched result
        if let Some(body) = obj.get_mut("body") {
            if let Some(body_obj) = body.as_object_mut() {
                body_obj.insert("results".to_string(), json!([result_obj]));
            }
        }
    }

    println!("[TOOL_CALLS] Response: {:?}", response);

    (StatusCode::OK, Json(response)).into_response()
}

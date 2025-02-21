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

pub fn parse_tool_call_request_to_result(
    body: Json<Value>,
) -> (Value, String) {
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
    // Check for error first
    if let Some(error) = stored_error {
        let error_message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown error occurred");
        println!(
            "[WEBHOOK API] [CREATE RESPONSE] Returning error response: {}",
            error_message
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, error_message.to_string()).into_response();
    }

    let mut headers = HeaderMap::new();

    // Get content type from the stored result
    let content_type = stored_result
        .as_ref()
        .and_then(|r| r.get("content_type"))
        .and_then(Value::as_str)
        .unwrap_or("application/json");

    // Handle different content types
    match content_type {
        "text/html" => {
            if let Some(body) = stored_result
                .as_ref()
                .and_then(|r| r.get("html_body"))
                .and_then(Value::as_str)
            {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("text/html"),
                );
                (StatusCode::OK, headers, Json(json!({"result": body}))).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid HTML response").into_response()
            }
        }

        "text/xml" => {
            if let Some(body) = stored_result
                .as_ref()
                .and_then(|r| r.get("xml_body"))
                .and_then(Value::as_str)
            {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("text/xml"),
                );
                (StatusCode::OK, headers, Json(json!({"result": body}))).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid XML response").into_response()
            }
        }

        "text/plain" => {
            if let Some(body) = stored_result
                .as_ref()
                .and_then(|r| r.get("text_body"))
                .and_then(Value::as_str)
            {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("text/plain"),
                );
                (StatusCode::OK, headers, Json(json!({"result": body}))).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid text response").into_response()
            }
        }

        // Default to JSON
        _ => {
            headers.insert(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/json"),
            );
            let body = stored_result
                .as_ref()
                .and_then(|r| r.get("json_body"))
                .cloned()
                .unwrap_or(json!({}));
            (StatusCode::OK, headers, Json(json!({"result": body}))).into_response()
        }
    }
}

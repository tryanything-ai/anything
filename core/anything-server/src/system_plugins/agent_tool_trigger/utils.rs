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

pub fn validate_agent_tool_input_and_response(
    workflow: &WorkflowVersionDefinition,
    require_response: bool,
) -> Result<(Box<&Action>, Option<Box<&Action>>), impl IntoResponse> {
    // Find the trigger action in the workflow
    println!("[AGENT_TOOL_API] Looking for trigger node in workflow");
    let trigger_node = match workflow
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
    {
        Some(trigger) => trigger,
        None => {
            println!("[AGENT_TOOL_API] No trigger found in workflow");
            return Err((StatusCode::BAD_REQUEST, "No trigger found in workflow").into_response());
        }
    };

    // Check if trigger node has plugin_id of "@anything/agent_tool_call"
    if trigger_node.plugin_name != PluginName::new("@anything/agent_tool_call".to_string()).unwrap()
    {
        println!(
            "[AGENT_TOOL_API] Invalid trigger type: {:?}",
            trigger_node.plugin_name
        );
        return Err((
            StatusCode::BAD_REQUEST,
            "Workflow trigger must be an webhook trigger to receive webhook",
        )
            .into_response());
    }

    let mut output_node = None;
    // Check for output node if required
    if require_response {
        println!("[AGENT_TOOL_API] Looking for output node in workflow");
        output_node = match workflow.actions.iter().find(|action| {
            action.plugin_name
                == PluginName::new("@anything/agent_tool_call_response".to_string()).unwrap()
        }) {
            Some(output) => Some(Box::new(output)),
            None => {
                println!("[AGENT_TOOL_API] No output node found in workflow");
                return Err(
                    (StatusCode::BAD_REQUEST, "No output node found in workflow").into_response(),
                );
            }
        };
    }

    Ok((Box::new(trigger_node), output_node))
}

pub fn parse_tool_call_request_to_result(body: Option<Json<Value>>) -> (Value, String) {
    match body {
        Some(Json(body)) => {
            let tool_call_id = body.get("id").and_then(Value::as_str).unwrap_or_default();

            // Get the function arguments as a string
            let arguments = body
                .get("function")
                .and_then(|f| f.get("arguments"))
                .and_then(Value::as_str)
                .unwrap_or("{}");

            // Parse the arguments string into a Value
            let mut parsed_args = serde_json::from_str::<Value>(arguments).unwrap_or(json!({}));

            // Add the tool_call_id to the arguments if parsed_args is an object
            let result = if let Value::Object(ref mut map) = parsed_args {
                map.insert("tool_call_id".to_string(), json!(tool_call_id));
                Value::Object(map.clone())
            } else {
                // If arguments wasn't an object, create a new object with just tool_call_id
                json!({
                    "tool_call_id": tool_call_id
                })
            };

            (result, tool_call_id.to_string())
        }
        None => (json!({}), String::new()),
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

use axum::{
    extract::Query,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};

use serde_json::{json, Value};
use std::str::FromStr;

use std::collections::HashMap;

use std::sync::Arc;

use crate::{secrets::get_secret_by_secret_value, workflow_types::Workflow, CachedApiKey};
use crate::{workflow_types::Action, AppState};

use crate::task_types::ActionType;

pub fn validate_webhook_input_and_response(
    workflow: &Workflow,
    require_response: bool,
) -> Result<(Box<&Action>, Option<Box<&Action>>), impl IntoResponse> {
    // Find the trigger action in the workflow
    println!("[WEBHOOK API] Looking for trigger node in workflow");
    let trigger_node = match workflow
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
    {
        Some(trigger) => trigger,
        None => {
            println!("[WEBHOOK API] No trigger found in workflow");
            return Err((StatusCode::BAD_REQUEST, "No trigger found in workflow").into_response());
        }
    };

    // Check if trigger node has plugin_id of "webhook"
    if trigger_node.plugin_id != "webhook" {
        println!(
            "[WEBHOOK API] Invalid trigger type: {}",
            trigger_node.plugin_id
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
        println!("[WEBHOOK API] Looking for output node in workflow");
        output_node = match workflow
            .actions
            .iter()
            .find(|action| action.plugin_id == "response")
        {
            Some(output) => Some(Box::new(output)),
            None => {
                println!("[WEBHOOK API] No output node found in workflow");
                return Err(
                    (StatusCode::BAD_REQUEST, "No output node found in workflow").into_response(),
                );
            }
        };
    }

    Ok((Box::new(trigger_node), output_node))
}

pub async fn validate_api_key(state: Arc<AppState>, api_key: String) -> Result<String, StatusCode> {
    println!("[VALIDATE API KEY] Starting API key validation");

    // Check cache first
    let cached_account = {
        println!("[VALIDATE API KEY] Checking cache for API key");
        let cache = state.api_key_cache.read().await;
        if let Some(cached) = cache.get(&api_key) {
            println!("[VALIDATE API KEY] Found cached API key");
            Some(cached.account_id.clone())
        } else {
            println!("[VALIDATE API KEY] API key not found in cache");
            None
        }
    };

    // Return early if we have a valid cached value
    if let Some(account_id) = cached_account {
        println!("[VALIDATE API KEY] Returning cached account ID");
        return Ok(account_id);
    }

    // Not in cache, check database
    println!("[VALIDATE API KEY] Checking database for API key");
    let secret = match get_secret_by_secret_value(state.clone(), api_key.clone()).await {
        Ok(secret) => {
            println!("[VALIDATE API KEY] Found secret in database");
            secret
        }
        Err(_) => {
            println!("[VALIDATE API KEY] Secret not found in database");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify this is an API key secret
    if !secret.anything_api_key {
        println!("[VALIDATE API KEY] Secret is not an API key");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update cache with new value
    {
        println!("[VALIDATE API KEY] Updating cache with new API key");
        let mut cache = state.api_key_cache.write().await;
        cache.insert(
            api_key,
            CachedApiKey {
                account_id: secret.account_id.clone(),
                secret_id: uuid::Uuid::parse_str(&secret.secret_id).unwrap(),
                secret_name: secret.secret_name.clone(),
            },
        );
    }

    println!("[VALIDATE API KEY] API key validation successful");
    Ok(secret.account_id)
}

pub async fn validate_security_model(
    rendered_inputs: &Value,
    headers: &HeaderMap,
    state: Arc<AppState>,
) -> Option<impl IntoResponse> {
    // Extract the security model from the rendered inputs
    println!("[WEBHOOK API] Extracting security model from rendered inputs");
    let security_model = rendered_inputs
        .get("security_model")
        .and_then(|v| v.as_str())
        .unwrap_or("none");

    println!(
        "[WEBHOOK API] Validating security with model: {}",
        security_model
    );

    match security_model {
        "none" => None,
        "basic_auth" => {
            println!("[WEBHOOK API] Validating Basic Auth");
            let expected_username = rendered_inputs.get("username").and_then(|v| v.as_str());
            let expected_password = rendered_inputs.get("password").and_then(|v| v.as_str());

            if expected_username.is_none() || expected_password.is_none() {
                println!("[WEBHOOK API] Missing username or password configuration");
                return Some((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
            }

            let auth_header = match headers.get("authorization") {
                Some(header) => header,
                None => {
                    println!("[WEBHOOK API] No Authorization header found");
                    return Some(
                        (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
                    );
                }
            };

            let auth_str = String::from_utf8_lossy(auth_header.as_bytes());
            if !auth_str.starts_with("Basic ") {
                println!("[WEBHOOK API] Invalid Authorization header format");
                return Some(
                    (StatusCode::UNAUTHORIZED, "Invalid Authorization header").into_response(),
                );
            }

            let credentials = match base64::decode(&auth_str[6..]) {
                Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
                Err(_) => {
                    println!("[WEBHOOK API] Failed to decode Basic Auth credentials");
                    return Some((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
                }
            };

            let parts: Vec<&str> = credentials.split(':').collect();
            if parts.len() != 2
                || parts[0] != expected_username.unwrap()
                || parts[1] != expected_password.unwrap()
            {
                println!("[WEBHOOK API] Invalid Basic Auth credentials");
                return Some((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
            }
            None
        }
        "api_key" => {
            println!("[WEBHOOK API] Validating API Key");
            let api_key = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
                Some(header) if header.starts_with("Bearer ") => header[7..].to_string(),
                _ => {
                    return Some(
                        (StatusCode::UNAUTHORIZED, "Missing or invalid API key").into_response(),
                    );
                }
            };

            // Validate the API key
            match validate_api_key(state, api_key.clone()).await {
                Ok(_account_id) => None,
                Err(status) => Some((status, "Invalid API key").into_response()),
            }
        }
        "custom_header" => {
            println!("[WEBHOOK API] Validating custom header");
            let header_name = match rendered_inputs
                .get("custom_header_name")
                .and_then(|v| v.as_str())
            {
                Some(name) => name,
                None => {
                    println!("[WEBHOOK API] No custom header name configured");
                    return Some(
                        (StatusCode::UNAUTHORIZED, "Invalid header configuration").into_response(),
                    );
                }
            };

            let expected_value = match rendered_inputs
                .get("custom_header_value")
                .and_then(|v| v.as_str())
            {
                Some(value) => value,
                None => {
                    println!("[WEBHOOK API] No custom header value configured");
                    return Some(
                        (StatusCode::UNAUTHORIZED, "Invalid header configuration").into_response(),
                    );
                }
            };

            let header_value = match headers.get(header_name) {
                Some(value) => String::from_utf8_lossy(value.as_bytes()),
                None => {
                    println!("[WEBHOOK API] Required custom header not found");
                    return Some(
                        (StatusCode::UNAUTHORIZED, "Missing required header").into_response(),
                    );
                }
            };

            if header_value != expected_value {
                println!("[WEBHOOK API] Invalid custom header value");
                return Some((StatusCode::UNAUTHORIZED, "Invalid header value").into_response());
            }
            None
        }
        _ => {
            println!("[WEBHOOK API] Invalid security model specified");
            Some((StatusCode::BAD_REQUEST, "Invalid security model").into_response())
        }
    }
}

pub fn validate_request_method(
    rendered_inputs: &Value,
    request_method: &str,
) -> Option<impl IntoResponse> {
    println!("[WEBHOOK API] Validating request method");

    // Extract the allowed method from rendered inputs
    let allowed_method = rendered_inputs
        .get("request_method")
        .and_then(|v| v.as_str())
        .unwrap_or("POST");

    println!(
        "[WEBHOOK API] Checking request method: {} against allowed method: {}",
        request_method, allowed_method
    );

    // If ANY is configured, allow all methods
    if allowed_method == "ANY" {
        return None;
    }

    // Otherwise check if methods match
    if request_method != allowed_method {
        println!("[WEBHOOK API] Invalid request method");
        return Some(
            (
                StatusCode::METHOD_NOT_ALLOWED,
                format!(
                    "Method {} not allowed. Expected {}",
                    request_method, allowed_method
                ),
            )
                .into_response(),
        );
    }

    None
}

pub fn convert_request_to_payload(
    method: axum::http::Method,
    query: Option<Query<HashMap<String, String>>>,
    body: Option<Json<Value>>,
) -> Value {
    match method {
        axum::http::Method::GET => {
            if let Some(Query(params)) = query {
                let mut result = serde_json::Map::new();
                let mut array_params: HashMap<String, Vec<Value>> = HashMap::new();

                // First pass - collect all parameters
                for (key, value) in params.iter() {
                    if let Some(base_key) = key.split('[').next() {
                        if key.contains('[') {
                            // Try to parse as number or boolean first
                            let parsed_value = value
                                .parse::<i64>()
                                .map(Value::from)
                                .or_else(|_| value.parse::<f64>().map(Value::from))
                                .or_else(|_| match value.to_lowercase().as_str() {
                                    "true" => Ok(Value::Bool(true)),
                                    "false" => Ok(Value::Bool(false)),
                                    _ => Ok(Value::String(value.clone())),
                                })
                                .unwrap_or_else(|_: std::num::ParseFloatError| {
                                    Value::String(value.clone())
                                });

                            array_params
                                .entry(base_key.to_string())
                                .or_default()
                                .push(parsed_value);
                            continue;
                        }
                    }

                    // Handle regular key-value pairs with type inference
                    let parsed_value = value
                        .parse::<i64>()
                        .map(Value::from)
                        .or_else(|_| value.parse::<f64>().map(Value::from))
                        .or_else(|_| match value.to_lowercase().as_str() {
                            "true" => Ok(Value::Bool(true)),
                            "false" => Ok(Value::Bool(false)),
                            _ => Ok(Value::String(value.clone())),
                        })
                        .unwrap_or_else(|_: std::num::ParseFloatError| {
                            Value::String(value.clone())
                        });

                    result.insert(key.clone(), parsed_value);
                }

                // Second pass - process array parameters
                for (key, values) in array_params {
                    let array_values: Vec<Value> = values.into_iter().collect();
                    result.insert(key, Value::Array(array_values));
                }

                Value::Object(result)
            } else {
                json!({})
            }
        }
        _ => {
            // For non-GET requests, merge query params with body if both exist
            let mut final_payload = body.map_or(json!({}), |Json(payload)| payload);

            if let Some(Query(params)) = query {
                if let Value::Object(ref mut map) = final_payload {
                    for (key, value) in params {
                        // Don't overwrite existing body parameters
                        if !map.contains_key(&key) {
                            map.insert(key, Value::String(value));
                        }
                    }
                }
            }

            final_payload
        }
    }
}

pub fn parse_response_action_response_into_api_response(stored_result: Value) -> impl IntoResponse {
    let status_code = stored_result
        .get("status_code")
        .and_then(Value::as_u64)
        .unwrap_or(200) as u16;

    let mut headers = HeaderMap::new();

    // Add headers from stored result
    if let Some(stored_headers) = stored_result.get("headers").and_then(Value::as_object) {
        for (key, value) in stored_headers {
            if let Some(value_str) = value.as_str() {
                if let (Ok(header_name), Ok(header_value)) =
                    (HeaderName::from_str(key), HeaderValue::from_str(value_str))
                {
                    headers.insert(header_name, header_value);
                }
            }
        }
    }

    let content_type = stored_result
        .get("content_type")
        .and_then(Value::as_str)
        .unwrap_or("application/json");

    // Handle different content types
    match content_type {
        // Handle HTML responses
        ct if ct.contains("text/html") => {
            if let Some(body) = stored_result.get("body").and_then(Value::as_str) {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("text/html; charset=utf-8"),
                );
                (
                    StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK),
                    headers,
                    body.to_string(),
                )
                    .into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid HTML response").into_response()
            }
        }

        // Handle XML responses
        ct if ct.contains("xml") => {
            if let Some(body) = stored_result.get("body").and_then(Value::as_str) {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("application/xml; charset=utf-8"),
                );
                (
                    StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK),
                    headers,
                    body.to_string(),
                )
                    .into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid XML response").into_response()
            }
        }

        // Handle plain text responses
        ct if ct.contains("text/plain") => {
            if let Some(body) = stored_result.get("body").and_then(Value::as_str) {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("text/plain; charset=utf-8"),
                );
                (
                    StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK),
                    headers,
                    body.to_string(),
                )
                    .into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid text response").into_response()
            }
        }

        // Default to JSON responses
        _ => {
            headers.insert(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/json; charset=utf-8"),
            );
            let body = stored_result.get("body").cloned().unwrap_or(json!({}));
            (
                StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK),
                headers,
                body.to_string(),
            )
                .into_response()
        }
    }
}

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use serde_json::Value;
use std::sync::Arc;

use crate::{secrets::get_secret_by_secret_value, workflow_types::Workflow, CachedApiKey};
use crate::{workflow_types::Action, AppState};

use crate::task_types::ActionType;

pub async fn validate_webhook_inputs_and_outputs(
    workflow: &Workflow,
    require_output: bool,
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
    if require_output {
        println!("[WEBHOOK API] Looking for output node in workflow");
        output_node = match workflow
            .actions
            .iter()
            .find(|action| action.plugin_id == "output")
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

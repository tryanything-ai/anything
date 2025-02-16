use anyhow::Result;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddToolInput {
    workflow_id: String,
}

pub async fn add_tool(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<AddToolInput>,
) -> impl IntoResponse {
    let client = &state.anything_client;
    println!("Adding tool to agent: {}", agent_id);

    // Get the workflow and its published version
    let workflow_response = match client
        .from("flows")
        .auth(&user.jwt)
        .eq("flow_id", &payload.workflow_id)
        .eq("account_id", &account_id)
        .select("*,flow_versions(*)")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch workflow",
            )
                .into_response()
        }
    };

    let body = match workflow_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let workflow: Value = match serde_json::from_str(&body) {
        Ok(workflow) => workflow,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    // Get the published version
    let published_version = workflow["flow_versions"].as_array().and_then(|versions| {
        versions
            .iter()
            .find(|v| v["published"].as_bool().unwrap_or(false))
    });

    let published_version = match published_version {
        Some(version) => version,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "No published version found for workflow",
            )
                .into_response()
        }
    };

    //TODO: make sure this is a valid Workflow input and output, can introspect variables to give to workflow
    //Make sure it has correct  input and reponse

    //Update Vapi
    let vapi_api_key = match std::env::var("VAPI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "VAPI_API_KEY environment variable not found",
            )
                .into_response()
        }
    };

    let reqwest_client = Client::new();

    //Vapi function calling docs
    //https://docs.vapi.ai/server-url/events#function-calling
    let response = reqwest_client
        .patch(&format!("https://api.vapi.ai/assistant/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": {
                "tools": [
                    {
                        "type": "function",
                        "function": {
                            "name": "check_customer_identity",
                            "description": "Check if the customer is a valid customer",
                            "parameters": {
                                "type": "object",
                                "properties": {
                                    "last_name": { "type": "string" },
                                    "last_4_ssn": { "type": "string" }
                                },
                                "required": ["last_name", "last_4_ssn"]
                            }
                        },
                        "server": { //https://docs.vapi.ai/server-url -> in this case a special URL just for the tool. we will need a more top level one for our application too.
                            "url": format!("https://api.tryanything.xyz/api/v1/workflow/{}/start/respond", payload.workflow_id),
                            // "secret": "a-super-secret-key", //x-vapi-secret header
                            "method": "POST",
                            // "headers": {
                            //     "Authorization": "Bearer {{vapi_api_key}}"
                            // }
                        }
                    }
                ]
            }
        }))
        .send()
        .await
        .map_err(|_| {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "[VAPI] Failed to send request to VAPI",
            )
                .into_response();
        });

    let response = match response {
        Ok(resp) => resp,
        Err(err) => return err,
    };

    let vapi_response = match response.json::<serde_json::Value>().await {
        Ok(vapi_config) => vapi_config,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse VAPI response",
            )
                .into_response()
        }
    };

    let agent_update = serde_json::json!({
        "vapi_config": vapi_response
    });

    //Take update and persist to our database
    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .eq("agent_id", agent_id)
        .eq("account_id", account_id)
        .update(agent_update.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update agent record",
            )
                .into_response()
        }
    };

    let agent = match response.json::<serde_json::Value>().await {
        Ok(agent) => agent,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent response",
            )
                .into_response()
        }
    };

    Json(agent).into_response()
}

pub async fn remove_tool(
    Path((account_id, agent_id, tool_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;
    println!("Removing tool from agent: {}", agent_id);

    // Get VAPI API key
    let vapi_api_key = match std::env::var("VAPI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "VAPI_API_KEY environment variable not found",
            )
                .into_response()
        }
    };

    let reqwest_client = Client::new();

    // Update VAPI to remove the tool
    let response = reqwest_client
        .patch(&format!("https://api.vapi.ai/assistant/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": {
                "tools": [] // Empty array to remove all tools
            }
        }))
        .send()
        .await
        .map_err(|_| {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "[VAPI] Failed to send request to VAPI",
            )
                .into_response();
        });

    let response = match response {
        Ok(resp) => resp,
        Err(err) => return err,
    };

    let vapi_response = match response.json::<serde_json::Value>().await {
        Ok(vapi_config) => vapi_config,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse VAPI response",
            )
                .into_response()
        }
    };

    // Update our database with the new VAPI config
    let agent_update = serde_json::json!({
        "vapi_config": vapi_response
    });

    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .eq("agent_id", agent_id)
        .eq("account_id", account_id)
        .update(agent_update.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update agent record",
            )
                .into_response()
        }
    };

    let agent = match response.json::<serde_json::Value>().await {
        Ok(agent) => agent,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent response",
            )
                .into_response()
        }
    };

    Json(agent).into_response()
}

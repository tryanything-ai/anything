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
use slugify::slugify;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::types::action_types::Action;
use crate::types::action_types::ActionType;
use crate::types::json_schema::{JsonSchema, JsonSchemaProperty};
use crate::types::workflow_types::DatabaseFlowVersion;
use crate::AppState;
use std::collections::HashMap;

// Define a struct for simplified agent tool properties that only allows basic types
#[derive(Debug, Serialize)]
struct AgentToolProperty {
    r#type: String,
}

#[derive(Debug, Serialize, Default)]
struct AgentToolProperties(HashMap<String, AgentToolProperty>);

impl AgentToolProperties {
    fn new() -> Self {
        AgentToolProperties(HashMap::new())
    }

    fn add_property(&mut self, name: String, property_type: &str) {
        let valid_type = match property_type {
            "string" | "number" | "boolean" | "null" => property_type,
            _ => "string",
        };

        self.0.insert(
            name,
            AgentToolProperty {
                r#type: valid_type.to_string(),
            },
        );
    }
}

impl From<HashMap<String, JsonSchemaProperty>> for AgentToolProperties {
    fn from(properties: HashMap<String, JsonSchemaProperty>) -> Self {
        let mut tool_properties = AgentToolProperties::new();

        for (name, property) in properties {
            tool_properties.add_property(name, property.r#type.as_deref().unwrap_or("string"));
        }

        tool_properties
    }
}

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
    println!("[TOOLS] Adding tool to agent: {}", agent_id);
    println!("[TOOLS] Workflow ID: {}", payload.workflow_id);

    // Get the workflow and its published version
    println!("[TOOLS] Fetching workflow details from database");

    let workflow_response = match client
        .from("flow_versions")
        .auth(&user.jwt)
        .select("*, flow:flows(*)")
        .eq("archived", "false")
        .eq("flow_id", &payload.workflow_id)
        .eq("account_id", &account_id)
        .eq("published", "true")
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Workflow response: {:?}", workflow_response);

    let body = match workflow_response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[TOOLS] Failed to read response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Body: {:?}", body);

    let workflow_version: DatabaseFlowVersion = match serde_json::from_str(&body) {
        Ok(version) => version,
        Err(e) => {
            println!("[TOOLS] Failed to parse workflow version: {:?}", e);
            return (StatusCode::BAD_REQUEST, "No workflow version found").into_response();
        }
    };

    let workflow = workflow_version.clone().flow.unwrap();
    println!("[TOOLS] Workflow: {:?}", workflow);

    println!("[TOOLS] Workflow version: {:?}", workflow_version);

    // Get the trigger action from the published version
    let trigger_action: Option<Action> = workflow_version
        .flow_definition
        .actions
        .iter()
        .find(|action| action.r#type == ActionType::Trigger)
        .cloned();

    println!("[TOOLS] Trigger action: {:?}", trigger_action);

    if trigger_action.is_none() {
        println!("[TOOLS] No trigger action found in workflow");
        return (
            StatusCode::BAD_REQUEST,
            "No trigger action found in workflow",
        )
            .into_response();
    }

    let trigger_action = trigger_action.unwrap();

    // Get workflow name and slugify it for the function name
    let tool_slug = slugify!(workflow["flow_name"].as_str().unwrap_or("unnamed-workflow"), separator = "_");

    let tool_description = workflow["description"].as_str().unwrap_or("");

    let tool_properties = AgentToolProperties::from(
        trigger_action
            .inputs_schema
            .as_ref()
            .and_then(|schema| schema.properties.clone())
            .unwrap_or_default(),
    );

    let required = trigger_action
        .inputs_schema
        .as_ref()
        .and_then(|schema| schema.required.clone())
        .unwrap_or_default();

    println!("[TOOLS] Properties: {:?}", tool_properties);

    //Update Vapi
    println!("[TOOLS] Getting VAPI API key");
    let vapi_api_key = match std::env::var("VAPI_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("[TOOLS] Failed to get VAPI API key: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "VAPI_API_KEY environment variable not found",
            )
                .into_response();
        }
    };

    let reqwest_client = Client::new();

    //TODO: 
    //Vapi function calling docs
    //https://docs.vapi.ai/server-url/events#function-calling
    println!("[TOOLS] Sending update to VAPI for agent: {}", agent_id);
    let response = reqwest_client
        .patch(&format!("https://api.vapi.ai/assistant/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": {
                "provider": "openai",
                "model": "gpt-4o-mini",
                "tools": [
                    {
                        "type": "function",
                        "function": {
                            "name": tool_slug,
                            "description": tool_description,
                            "parameters": {
                                "type": "object",
                                "properties": tool_properties,
                                "required": required
                            }
                        },
                        "server": {
                            "url": format!("https://api.tryanything.xyz/api/v1/workflow/{}/start/respond", payload.workflow_id),
                        }
                    }
                ]
            }
        }))
        .send()
        .await
        .map_err(|e| {
            println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
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

    println!("[TOOLS] Parsing VAPI response");
    let vapi_response = match response.json::<serde_json::Value>().await {
        Ok(vapi_config) => vapi_config,
        Err(e) => {
            println!("[TOOLS] Failed to parse VAPI response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse VAPI response",
            )
                .into_response();
        }
    };

    let agent_update = serde_json::json!({
        "vapi_config": vapi_response
    });

    //Take update and persist to our database
    println!("[TOOLS] Updating agent record in database");
    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .eq("agent_id", agent_id.clone())
        .eq("account_id", account_id)
        .update(agent_update.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TOOLS] Failed to update agent record: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update agent record",
            )
                .into_response();
        }
    };

    let agent = match response.json::<serde_json::Value>().await {
        Ok(agent) => agent,
        Err(e) => {
            println!("[TOOLS] Failed to parse agent response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent response",
            )
                .into_response();
        }
    };

    let agent_tool = serde_json::json!({
        "agent_id": agent_id.clone(),
        "flow_id": payload.workflow_id,
        "active": true,
        "archived": false
    });

    //Persist to our database
    println!("[TOOLS] Persisting agent tool to database");
    let response = match client
        .from("agent_tools")
        .auth(&user.jwt)
        .insert(agent_tool.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TOOLS] Failed to persist agent tool: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to persist agent tool to database",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Successfully added tool to agent: {}", agent_id);
    Json(agent).into_response()
}

pub async fn remove_tool(
    Path((account_id, agent_id, tool_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;
    println!("[TOOLS] Removing tool {} from agent: {}", tool_id, agent_id);

    // Get VAPI API key
    println!("[TOOLS] Getting VAPI API key");
    let vapi_api_key = match std::env::var("VAPI_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("[TOOLS] Failed to get VAPI API key: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "VAPI_API_KEY environment variable not found",
            )
                .into_response();
        }
    };

    let reqwest_client = Client::new();

    // Update VAPI to remove the tool
    println!("[TOOLS] Sending update to VAPI to remove tools");
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
        .map_err(|e| {
            println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
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

    println!("[TOOLS] Parsing VAPI response");
    let vapi_response = match response.json::<serde_json::Value>().await {
        Ok(vapi_config) => vapi_config,
        Err(e) => {
            println!("[TOOLS] Failed to parse VAPI response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse VAPI response",
            )
                .into_response();
        }
    };

    // Update our database with the new VAPI config
    let agent_update = serde_json::json!({
        "vapi_config": vapi_response
    });

    println!("[TOOLS] Updating agent record in database");
    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .eq("agent_id", agent_id.clone())
        .eq("account_id", account_id)
        .update(agent_update.to_string())
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TOOLS] Failed to update agent record: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update agent record",
            )
                .into_response();
        }
    };

    let agent = match response.json::<serde_json::Value>().await {
        Ok(agent) => agent,
        Err(e) => {
            println!("[TOOLS] Failed to parse agent response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent response",
            )
                .into_response();
        }
    };

    println!(
        "[TOOLS] Successfully removed tool from agent: {}",
        agent_id.clone()
    );
    Json(agent).into_response()
}

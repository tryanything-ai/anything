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
use crate::types::json_schema::JsonSchemaProperty;
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

    // Get both the agent and workflow in parallel
    println!("[TOOLS] Fetching agent and workflow details");
    let agent_future = client
        .from("agents")
        .auth(&user.jwt)
        .select("*")
        .eq("agent_id", &agent_id)
        .eq("account_id", &account_id.clone())
        .single()
        .execute();

    let workflow_future = client
        .from("flow_versions")
        .auth(&user.jwt)
        .select("*, flow:flows(*)")
        .eq("archived", "false")
        .eq("flow_id", &payload.workflow_id)
        .eq("account_id", &account_id.clone())
        .eq("published", "true")
        .single()
        .execute();

    let agent_tools_future = client
        .from("agent_tools")
        .auth(&user.jwt)
        .select("*")
        .eq("agent_id", &agent_id)
        .eq("flow_id", &payload.workflow_id)
        .eq("account_id", &account_id.clone())
        .eq("archived", "false")
        .single()
        .execute();

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
    //get tools definition from vapi
    let vapi_update_future = async {
        reqwest_client
            .get(&format!("https://api.vapi.ai/assistant/{}", agent_id))
            .header("Authorization", format!("Bearer {}", vapi_api_key))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| {
                println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "[VAPI] Failed to send request to VAPI",
                )
                    .into_response()
            })
    };

    let (agent_response, workflow_response, agent_tools_response, vapi_config_response) = tokio::join!(
        agent_future,
        workflow_future,
        agent_tools_future,
        vapi_update_future
    );

    // Handle agent response
    let agent_response = match agent_response {
        Ok(response) => response,
        Err(err) => {
            println!("[TOOLS] Failed to fetch agent: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch agent details",
            )
                .into_response();
        }
    };

    let agent = match agent_response.json::<Value>().await {
        Ok(agent) => agent,
        Err(e) => {
            println!("[TOOLS] Failed to parse agent response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent details",
            )
                .into_response();
        }
    };

    if agent.is_null() {
        println!("[TOOLS] Agent not found");
        return (StatusCode::NOT_FOUND, "Agent not found").into_response();
    }

    // Handle agent Tool response
    let agent_tools_response = match agent_tools_response {
        Ok(response) => response,
        Err(err) => {
            println!("[TOOLS] Failed to fetch agent: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch agent details",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Agent tools response: {:?}", agent_tools_response);

    let agent_tool = match agent_tools_response.json::<Value>().await {
        Ok(tools) => {
            // Check if we got an error response from Supabase
            if tools.get("code") == Some(&json!("PGRST116")) {
                // This means no rows were found, which is what we want
                Value::Null
            } else {
                tools
            }
        }
        Err(e) => {
            println!("[TOOLS] Failed to parse agent tools response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent tools response",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Agent tool: {:?}", agent_tool);

    //BLOCK Addition of Tool if tool already exists
    if !agent_tool.is_null() {
        println!("[TOOLS] Agent tool already exists");
        return (StatusCode::CONFLICT, "Agent tool already exists").into_response();
    }

    // Handle workflow response
    let workflow_response = match workflow_response {
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
    let tool_slug = slugify!(
        workflow["flow_name"].as_str().unwrap_or("unnamed-workflow"),
        separator = "_"
    );

    let tool_description = workflow["description"].as_str().unwrap_or("");

    let tool_properties = AgentToolProperties::from(
        trigger_action
            .inputs_schema
            .as_ref()
            .and_then(|schema| schema.properties.clone())
            .unwrap_or_default(),
    );

    println!("[TOOLS] Tool properties: {:?}", tool_properties);

    let required = trigger_action
        .inputs_schema
        .as_ref()
        .and_then(|schema| schema.required.clone())
        .unwrap_or_default();

    println!("[TOOLS] Properties: {:?}", tool_properties);

    // Handle VAPI response
    let vapi_config_response = match vapi_config_response {
        Ok(resp) => resp,
        Err(e) => {
            println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "[VAPI] Failed to send request to VAPI",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Parsing VAPI response");
    let vapi_config = match vapi_config_response.json::<serde_json::Value>().await {
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

    //Remove Tool from vapi config
    let mut new_vapi_config = vapi_config.clone();

    // let mut new_vapi_config = current_vapi_config.clone();
    // Get existing tools or create empty array
    let mut tools = new_vapi_config["model"]["tools"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    // Add new tool
    tools.push(json!({
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
    }));

    println!("[TOOLS] Updated tools array: {:?}", tools);

    new_vapi_config["model"]["tools"] = serde_json::Value::Array(tools);
    //TODO:
    //Vapi function calling docs
    //https://docs.vapi.ai/server-url/events#function-calling
    println!("[TOOLS] Sending update to VAPI for agent: {}", agent_id);
    let response = reqwest_client
        .patch(&format!("https://api.vapi.ai/assistant/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": new_vapi_config["model"]
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
        .eq("account_id", account_id.clone())
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
    let properties = json!({"parameters": {
      "type": "object",
      "required": required,
      "properties": tool_properties
    }});

    let agent_tool = serde_json::json!({
        "agent_id": agent_id.clone(),
        "flow_id": payload.workflow_id,
        "account_id": account_id.clone(),
        "tool_slug": tool_slug,
        "tool_name": workflow["flow_name"],
        "tool_description": tool_description,
        "tool_parameters": properties,
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

    //get tool definition from our database
    let agent_tools_future = client
        .from("agent_tools")
        .auth(&user.jwt)
        .select("*")
        .eq("agent_id", &agent_id)
        .eq("flow_id", &tool_id)
        .eq("account_id", &account_id.clone())
        .eq("archived", "false")
        .single()
        .execute();

    let reqwest_client = Client::new();

    //get tools definition from vapi
    let vapi_update_future = async {
        reqwest_client
            .get(&format!("https://api.vapi.ai/assistant/{}", agent_id))
            .header("Authorization", format!("Bearer {}", vapi_api_key))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| {
                println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "[VAPI] Failed to send request to VAPI",
                )
                    .into_response()
            })
    };

    let (agent_tools_response, vapi_response) =
        tokio::join!(agent_tools_future, vapi_update_future);

    // Handle agent Tool response
    let agent_tools_response = match agent_tools_response {
        Ok(response) => response,
        Err(err) => {
            println!("[TOOLS] Failed to fetch agent: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch agent details",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Agent tools response: {:?}", agent_tools_response);

    let agent_tool = match agent_tools_response.json::<Value>().await {
        Ok(tools) => {
            // Check if we got an error response from Supabase
            if tools.get("code") == Some(&json!("PGRST116")) {
                // This means no rows were found, which is what we want
                Value::Null
            } else {
                tools
            }
        }
        Err(e) => {
            println!("[TOOLS] Failed to parse agent tools response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse agent tools response",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Agent tool: {:?}", agent_tool);

    //BLOCK Addition of Tool if tool already exists
    if agent_tool.is_null() {
        println!("[TOOLS] Agent tool not found");
        return (StatusCode::NOT_FOUND, "Agent tool not found").into_response();
    }

    // Handle VAPI response
    let vapi_response = match vapi_response {
        Ok(resp) => resp,
        Err(e) => {
            println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "[VAPI] Failed to send request to VAPI",
            )
                .into_response();
        }
    };

    println!("[TOOLS] Parsing VAPI response");
    let vapi_response = match vapi_response.json::<serde_json::Value>().await {
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

    //Remove Tool from vapi config
    let mut new_vapi_config = vapi_response.clone();

    //Remove specific tool from vapi config
    if let Some(tools) = new_vapi_config["model"]["tools"].as_array() {
        let filtered_tools: Vec<_> = tools
            .iter()
            .filter(|tool| {
                if let Some(server) = tool["server"].as_object() {
                    if let Some(url) = server["url"].as_str() {
                        !url.contains(&tool_id)
                    } else {
                        true
                    }
                } else {
                    true
                }
            })
            .cloned()
            .collect();
        new_vapi_config["model"]["tools"] = serde_json::Value::Array(filtered_tools);
    }

    println!("[TOOLS] New VAPI config: {:?}", new_vapi_config);

    //Update VAPI with new config
    let update_reponse = reqwest_client
        .patch(&format!("https://api.vapi.ai/assistant/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": new_vapi_config["model"]
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

    let update_response = match update_reponse {
        Ok(resp) => resp,
        Err(e) => {
            println!("[TOOLS] Failed to send request to VAPI: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "[VAPI] Failed to send request to VAPI",
            )
                .into_response();
        }
    };

    let update_response = match update_response.json::<Value>().await {
        Ok(json) => json,
        Err(e) => {
            println!("[TOOLS] Failed to parse VAPI response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse VAPI response",
            )
                .into_response();
        }
    };

    let agent_update: Value = serde_json::json!({
        "vapi_config": update_response
    });

    //Take update and persist to our database for the agent
    println!("[TOOLS] Updating agent record in database");
    let response = match client
        .from("agents")
        .auth(&user.jwt)
        .eq("agent_id", agent_id.clone())
        .eq("account_id", account_id.clone())
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

    // Remove the tool from agent_tools table
    println!("[TOOLS] Removing tool from agent_tools table");
    match client
        .from("agent_tools")
        .auth(&user.jwt)
        .eq("agent_id", agent_id.clone())
        .eq("account_id", account_id.clone())
        .eq("flow_id", tool_id.clone())
        .delete()
        .execute()
        .await
    {
        Ok(_) => println!("[TOOLS] Successfully removed tool from agent_tools table"),
        Err(e) => {
            println!(
                "[TOOLS] Failed to remove tool from agent_tools table: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to remove tool from agent_tools table",
            )
                .into_response();
        }
    };
    Json(agent).into_response()
}

pub async fn get_agent_tools(
    Path((account_id, agent_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[TOOLS] Handling get_agent_tools for agent {}", agent_id);

    let client = &state.anything_client;

    let response = match client
        .from("agent_tools")
        .auth(&user.jwt)
        .select("*, flow:flows(*)")
        .eq("agent_id", &agent_id)
        .eq("account_id", &account_id)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            println!("[TOOLS] Failed to execute request: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    if response.status() == 204 {
        return (StatusCode::NO_CONTENT, "No content").into_response();
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(err) => {
            println!("[TOOLS] Failed to read response body: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(err) => {
            println!("[TOOLS] Failed to parse JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    Json(items).into_response()
}

pub async fn update_agent_tool_if_needed_on_workflow_publish(
    workflow_id: String,
    workflow_version_id: String,
    account_id: String,
    state: Arc<AppState>,
    user: User,
) -> Result<Value> {
    let client = &state.anything_client;

    // First check if this workflow is being used as an agent tool
    let agent_tools_response = match client
        .from("agent_tools")
        .auth(&user.jwt)
        .select("*, agent:agents(*)")
        .eq("flow_id", &workflow_id)
        .eq("account_id", &account_id)
        .eq("archived", "false")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TOOLS] Failed to fetch agent tools: {:?}", e);
            return Err(anyhow::anyhow!("Failed to fetch agent tools"));
        }
    };

    let agent_tools = match agent_tools_response.json::<Vec<Value>>().await {
        Ok(tools) => tools,
        Err(e) => {
            println!("[TOOLS] Failed to parse agent tools response: {:?}", e);
            return Err(anyhow::anyhow!("Failed to parse agent tools response"));
        }
    };

    // If no tools found, this workflow isn't used as an agent tool
    if agent_tools.is_empty() {
        return Ok(json!({}));
    }

    //Turns out we need to update one or many agents on vapi and in our database
    //Update Vapie Agent
    //Update Vapi Config in Agents Table
    //Update Agent Tools table in Database

    // Get the workflow version details
    let workflow_version_response = match client
        .from("flow_versions")
        .auth(&user.jwt)
        .select("*, flow:flows(*)")
        .eq("flow_version_id", &workflow_version_id)
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TOOLS] Failed to fetch workflow version: {:?}", e);
            return Err(anyhow::anyhow!("Failed to fetch workflow version"));
        }
    };

    let workflow_version = match workflow_version_response
        .json::<DatabaseFlowVersion>()
        .await
    {
        Ok(version) => version,
        Err(e) => {
            println!("[TOOLS] Failed to parse workflow version: {:?}", e);
            return Err(anyhow::anyhow!("Failed to parse workflow version"));
        }
    };

    //Create the new config needed for vapi from the new workflow
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
        return Err(anyhow::anyhow!("No trigger action found in workflow"));
    }

    let trigger_action = trigger_action.unwrap();

    let tool_properties = AgentToolProperties::from(
        trigger_action
            .inputs_schema
            .as_ref()
            .and_then(|schema| schema.properties.clone())
            .unwrap_or_default(),
    );

    let tool_slug = slugify!(
        workflow["flow_name"].as_str().unwrap_or("unnamed-workflow"),
        separator = "_"
    );

    let required = trigger_action
        .inputs_schema
        .as_ref()
        .and_then(|schema| schema.required.clone())
        .unwrap_or_default();

    let tool_description = workflow["description"].as_str().unwrap_or("");

    let properties = json!({"parameters": {
      "type": "object",
      "required": required,
      "properties": tool_properties
    }});

    let agent_tool_update_input = serde_json::json!({
        "tool_slug": tool_slug,
        "tool_name": workflow["flow_name"],
        "tool_description": tool_description,
        "tool_parameters": properties,
    });

    //Update Properties on every agent_tool that uses this workflow
    let update_all_agent_tools_response = match client
        .from("agent_tools")
        .auth(&user.jwt)
        .eq("flow_id", &workflow_id)
        .eq("account_id", &account_id)
        .update(agent_tool_update_input.to_string())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[TOOLS] Failed to update agent tools: {:?}", e);
            return Err(anyhow::anyhow!("Failed to update agent tools"));
        }
    };

    //TODO: this should be parallelized in future if someone has lots of agents it could be slow or break?
    // Process each agent tool sequentially
    for tool in agent_tools.iter() {
        let agent_id = tool["agent"]["vapi_assitant_id"]
            .as_str()
            .unwrap_or_default();
        let vapi_api_key = std::env::var("VAPI_API_KEY").unwrap_or_default();

        let reqwest_client = reqwest::Client::new();
        // 1. Get current VAPI assistant config
        let vapi_response = match reqwest_client
            .get(&format!("https://api.vapi.ai/assistant/{}", agent_id))
            .header("Authorization", format!("Bearer {}", vapi_api_key))
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                println!(
                    "[TOOLS] Failed to get VAPI config for agent {}: {:?}",
                    agent_id, e
                );
                continue;
            }
        };

        let vapi_config = match vapi_response.json::<Value>().await {
            Ok(config) => config,
            Err(e) => {
                println!(
                    "[TOOLS] Failed to parse VAPI config for agent {}: {:?}",
                    agent_id, e
                );
                continue;
            }
        };

        // 2. Update the tools array in the config
        let mut new_vapi_config = vapi_config.clone();

        let mut tools = new_vapi_config["model"]["tools"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        //Remove specific tool from vapi config
        if let Some(tools) = new_vapi_config["model"]["tools"].as_array() {
            let filtered_tools: Vec<_> = tools
                .iter()
                .filter(|tool| {
                    if let Some(server) = tool["server"].as_object() {
                        if let Some(url) = server["url"].as_str() {
                            !url.contains(&workflow_id)
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            new_vapi_config["model"]["tools"] = serde_json::Value::Array(filtered_tools);
        }

        //push the new one on
        tools.push(json!({
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
                "url": format!("https://api.tryanything.xyz/api/v1/workflow/{}/start/respond", workflow_id),
            }
        }));

        new_vapi_config["model"]["tools"] = serde_json::Value::Array(tools);

        //https://docs.vapi.ai/server-url/events#function-calling
        println!("[TOOLS] Sending update to VAPI for agent: {}", agent_id);
        let response = reqwest_client
            .patch(&format!("https://api.vapi.ai/assistant/{}", agent_id))
            .header("Authorization", format!("Bearer {}", vapi_api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": new_vapi_config["model"]
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
            Err(err) => return Err(anyhow::anyhow!("Failed to send request to VAPI")),
        };

        println!("[TOOLS] Parsing VAPI response");
        let vapi_response = match response.json::<serde_json::Value>().await {
            Ok(vapi_config) => vapi_config,
            Err(e) => {
                println!("[TOOLS] Failed to parse VAPI response: {:?}", e);
                return Err(anyhow::anyhow!("Failed to parse VAPI response"));
            }
        };

        //Save response to agent table
        let agent_update: Value = serde_json::json!({
            "vapi_config": vapi_response
        });

        //Update the agent record in the database
        let response = match client
            .from("agents")
            .auth(&user.jwt)
            .eq("agent_id", agent_id)
            .eq("account_id", account_id.clone())
            .update(agent_update.to_string())
            .execute()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                println!("[TOOLS] Failed to update agent record: {:?}", e);
                return Err(anyhow::anyhow!("Failed to update agent record"));
            }
        };
    }

    Ok(json!(agent_tools.len()))
}

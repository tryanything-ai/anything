use crate::AppState;
use anyhow::Result;
use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    Json,
};

use futures::future::join_all;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use axum::http::StatusCode;

pub async fn create_vapi_agent(
    account_id: &str,
    name: &str,
    greeting: &str,
    system_prompt: &str,
) -> Result<Value> {
    println!("[VAPI] Creating new agent with name: {}", name);

    let vapi_api_key = std::env::var("VAPI_API_KEY")
        .map_err(|_| anyhow::anyhow!("VAPI_API_KEY environment variable not found"))?;

    let client = Client::new();
    println!("[VAPI] Sending request to create assistant");

    let response = client
        .post("https://api.vapi.ai/assistant")
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": name,
            "firstMessage": greeting,
            "backgroundSound": "off",
            "metadata": {
                "account_id": account_id,
            },
            "model": {
                "provider": "openai",
                "model": "gpt-4o-mini",
                "messages": [
                    {
                        "role": "system",
                        "content": system_prompt
                    }
                ]
            }
        }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to send request to VAPI: {}", e))?;

    let response_json = response
        .json::<Value>()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))?;

    println!("[VAPI] Response JSON: {:?}", response_json);
    if let Some(error) = response_json.get("error") {
        println!("[VAPI] Error from VAPI: {}", error);
        return Err(anyhow::anyhow!("[VAPI] Error from VAPI: {}", error));
    }

    Ok(response_json)
}

pub async fn update_vapi_agent(
    vapi_agent_id: &str,
    name: &str,
    greeting: &str,
    system_prompt: &str,
) -> Result<Value> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")
        .map_err(|_| anyhow::anyhow!("VAPI_API_KEY environment variable not found"))?;
    let client = Client::new();

    let vapi_agent_response = client
        .get(&format!("https://api.vapi.ai/assistant/{}", vapi_agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to send request to VAPI: {}", e))?;

    let vapi_agent_json = vapi_agent_response
        .json::<Value>()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))?;

    let mut new_vapi_config = vapi_agent_json.clone();

    println!("[VAPI] VAPI agent JSON: {:?}", vapi_agent_json);

    new_vapi_config["model"]["messages"] = serde_json::Value::Array(vec![json!({
        "role": "system",
        "content": system_prompt
    })]);

    new_vapi_config["firstMessage"] = json!(greeting);
    new_vapi_config["name"] = json!(name);

    println!(
        "[VAPI] Sending request to update assistant {}",
        vapi_agent_id
    );

    println!("[VAPI] New VAPI config: {:?}", new_vapi_config);

    let response = client
        .patch(&format!("https://api.vapi.ai/assistant/{}", vapi_agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .json(&json!({
            "firstMessage": new_vapi_config["firstMessage"],
            "backgroundSound": "off",
            "name": new_vapi_config["name"],
            "model": new_vapi_config["model"]
        }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to send request to VAPI: {}", e))?;

    let response_json = response
        .json::<Value>()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))?;

    if let Some(error) = response_json.get("error") {
        println!("[VAPI] Error from VAPI: {}", error);
        return Err(anyhow::anyhow!("[VAPI] Error from VAPI: {}", error));
    }

    Ok(response_json)
}

pub async fn create_vapi_phone_number_from_twilio_number(
    state: Arc<AppState>,
    user: User,
    phone_number_id: &str,
    vapi_agent_id: &str,
) -> Result<Value> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let twilio_account_sid = std::env::var("TWILIO_ACCOUNT_SID")?;
    let twilio_auth_token = std::env::var("TWILIO_AUTH_TOKEN")?;

    let client = &state.anything_client;

    let response = client
        .from("phone_numbers")
        .auth(&user.jwt)
        .eq("phone_number_id", phone_number_id)
        .select("*")
        .execute()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to fetch phone number: {}", e))?;

    let body = response
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to read response body: {}", e))?;

    let phone_numbers: Value = serde_json::from_str(&body)
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse phone numbers: {}", e))?;

    // Get the first phone number
    let phone_number = phone_numbers
        .as_array()
        .and_then(|numbers| numbers.first())
        .ok_or_else(|| anyhow::anyhow!("[VAPI] No phone number found"))?;

    println!("[VAPI] Found phone number: {:?}", phone_number);

    let reqwest_client = Client::new();

    println!("[VAPI] Creating phone number {}", phone_number_id);

    let input = json!({
        "provider": "twilio",
        "number": phone_number["phone_number"],
        "twilioAccountSid": twilio_account_sid,
        "twilioAuthToken": twilio_auth_token,
        // "phoneNumberId": phone_number_id,
        "assistantId": vapi_agent_id,
    });

    println!("[VAPI] Input: {:?}", input);

    let response = reqwest_client
        .post("https://api.vapi.ai/phone-number")
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .json(&input)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to create phone number: {}", e))?;

    let response_json = response
        .json::<Value>()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))?;

    if let Some(error) = response_json.get("error") {
        println!("[VAPI] Error from VAPI: {}", error);
        return Err(anyhow::anyhow!("[VAPI] Error from VAPI: {}", error));
    }

    println!("[VAPI] Response JSON: {:?}", response_json);

    Ok(response_json)
}

pub async fn delete_vapi_phone_number(vapi_phone_number_id: &str) -> Result<()> {
    // Remove any quotes from the ID if present
    let cleaned_id = vapi_phone_number_id.trim_matches('"');

    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let client = Client::new();

    println!("[VAPI] Deleting phone number {}", cleaned_id);

    let response = client
        .delete(&format!("https://api.vapi.ai/phone-number/{}", cleaned_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("[VAPI] Delete Number Response: {:?}", response_text);

    Ok(())
}

pub async fn get_vapi_calls(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[CALLS] Getting calls for account {}", account_id);

    let vapi_api_key = match std::env::var("VAPI_API_KEY") {
        Ok(key) => {
            println!("[CALLS] Successfully got VAPI API key");
            key
        }
        Err(_) => {
            println!("[CALLS] Failed to get VAPI API key from env vars");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get VAPI API key",
            )
                .into_response();
        }
    };

    let client = &state.anything_client;

    println!("[CALLS] Querying Supabase for assistant IDs");
    // Get all VAPI assistant IDs for this account's agents
    let assistant_ids_response = match client
        .from("agents")
        .auth(&user.jwt)
        .select("vapi_assistant_id")
        .eq("account_id", &account_id)
        .execute()
        .await
    {
        Ok(response) => {
            println!("[CALLS] Successfully queried Supabase for assistant IDs");
            response
        }
        Err(e) => {
            println!(
                "[CALLS] Failed to fetch assistant IDs from Supabase: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch assistant IDs",
            )
                .into_response();
        }
    };

    let assistant_ids_body = match assistant_ids_response.text().await {
        Ok(body) => {
            println!("[CALLS] Successfully read assistant IDs response body");
            body
        }
        Err(e) => {
            println!(
                "[CALLS] Failed to read assistant IDs response body: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read assistant IDs response",
            )
                .into_response();
        }
    };

    println!("[CALLS] Assistant IDs body: {}", assistant_ids_body);

    let assistant_ids: Value = match serde_json::from_str(&assistant_ids_body) {
        Ok(ids) => {
            println!("[CALLS] Successfully parsed assistant IDs JSON");
            ids
        }
        Err(e) => {
            println!("[CALLS] Failed to parse assistant IDs JSON: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse assistant IDs",
            )
                .into_response();
        }
    };

    let assistant_ids = match assistant_ids.as_array() {
        Some(ids) => {
            println!("[CALLS] Found {} assistant IDs", ids.len());
            ids
        }
        None => {
            println!("[CALLS] Assistant IDs was not an array");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid assistant IDs format",
            )
                .into_response();
        }
    };

    let reqwest_client = Client::new();

    let mut all_calls = Vec::new();
    for assistant in assistant_ids {
        if let Some(assistant_id) = assistant
            .get("vapi_assistant_id")
            .and_then(|id| id.as_str())
        {
            println!("[CALLS] Fetching calls for assistant ID: {}", assistant_id);
            let response = match reqwest_client
                .get("https://api.vapi.ai/call")
                .header("Authorization", format!("Bearer {}", vapi_api_key))
                .query(&[("assistant_id", assistant_id)])
                .send()
                .await
            {
                Ok(response) => {
                    println!(
                        "[CALLS] Successfully got response from VAPI for assistant {}",
                        assistant_id
                    );
                    response
                }
                Err(e) => {
                    println!(
                        "[CALLS] Failed to fetch VAPI calls for assistant {}: {:?}",
                        assistant_id, e
                    );
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to fetch VAPI calls",
                    )
                        .into_response();
                }
            };

            let calls = match response.json::<Value>().await {
                Ok(calls) => {
                    println!(
                        "[CALLS] Successfully parsed VAPI response for assistant {}",
                        assistant_id
                    );
                    calls
                }
                Err(e) => {
                    println!(
                        "[CALLS] Failed to parse VAPI response for assistant {}: {:?}",
                        assistant_id, e
                    );
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to parse VAPI response",
                    )
                        .into_response();
                }
            };

            if let Some(calls) = calls.as_array() {
                println!(
                    "[CALLS] Found {} calls for assistant {}",
                    calls.len(),
                    assistant_id
                );
                for call in calls {
                    all_calls.push(call.clone());
                }
            } else {
                println!(
                    "[CALLS] No calls array found for assistant {}",
                    assistant_id
                );
            }
        }
    }

    println!(
        "[CALLS] Sorting {} total calls by creation date",
        all_calls.len()
    );
    all_calls.sort_by(|a, b| {
        b.get("createdAt")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .cmp(&a.get("createdAt").and_then(|v| v.as_str()).unwrap_or(""))
    });

    println!(
        "[CALLS] Successfully processed all calls. Returning {} calls",
        all_calls.len()
    );

    Json(Value::Array(all_calls)).into_response()
}

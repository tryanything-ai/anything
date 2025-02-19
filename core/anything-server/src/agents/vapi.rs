use crate::AppState;
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;

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
    phone_number_id: &str,
    vapi_agent_id: &str,
) -> Result<Value> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let twilio_account_sid = std::env::var("TWILIO_ACCOUNT_SID")?;
    let twilio_auth_token = std::env::var("TWILIO_AUTH_TOKEN")?;

    let client = &state.anything_client;

    let response = client
        .from("phone_numbers")
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
        "phoneNumberId": phone_number_id,
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
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let client = Client::new();

    println!("[VAPI] Deleting phone number {}", vapi_phone_number_id);

    client
        .delete(&format!("https://api.vapi.ai/phone-number/{}", vapi_phone_number_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .send()
        .await?;

    Ok(())
}

pub async fn delete_vapi_agent(agent_id: &str) -> Result<()> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let client = Client::new();

    client
        .delete(&format!("https://api.vapi.ai/agents/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .send()
        .await?;

    Ok(())
}


pub async fn get_vapi_calls(assistant_id: &str) -> Result<Value> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let client = Client::new();

    let response = client
    
    .get(&format!("https://api.vapi.ai/call"))
    .header("Authorization", format!("Bearer {}", vapi_api_key))
    .query(&[("assistant_id", assistant_id)])
    .send()
    .await?
    .json::<Value>()
    .await
    .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))?;

    println!("[VAPI] Response JSON: {:?}", response);

    Ok(response)
}
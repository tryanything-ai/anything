use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

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

    //Could set account_id in metadata to have something on vapi side too
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

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        println!("[VAPI] Error creating agent: {} - {}", status, error_text);
        return Err(anyhow::anyhow!(
            "VAPI returned error status {}: {}",
            status,
            error_text
        ));
    }

    println!("[VAPI] Successfully created agent, parsing response");
    response
        .json::<Value>()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))
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

    println!("[VAPI] Sending request to update assistant {}", vapi_agent_id);
    let response = client
        .patch(&format!("https://api.vapi.ai/assistant/{}", vapi_agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": name,
            "firstMessage": greeting,
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

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        println!("[VAPI] Error updating agent: {} - {}", status, error_text);
        return Err(anyhow::anyhow!(
            "VAPI returned error status {}: {}",
            status,
            error_text
        ));
    }

    println!("[VAPI] Successfully updated agent, parsing response");
    response
        .json::<Value>()
        .await
        .map_err(|e| anyhow::anyhow!("[VAPI] Failed to parse VAPI response: {}", e))
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

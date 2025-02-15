use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

pub async fn create_vapi_agent(name: &str, greeting: &str) -> Result<String> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let client = Client::new();

    let response = client
        .post("https://api.vapi.ai/agents")
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .json(&json!({
            "name": name,
            "firstMessage": greeting,
            "provider": "openai",
            "model": "gpt-4",
            "voice": {
                "provider": "11labs",
                "voiceId": "jennifer"
            }
        }))
        .send()
        .await?;

    let agent_data = response.json::<Value>().await?;
    let agent_id = agent_data["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get agent ID"))?;

    Ok(agent_id.to_string())
}

pub async fn update_vapi_agent(agent_id: &str, name: &str, greeting: &str) -> Result<()> {
    let vapi_api_key = std::env::var("VAPI_API_KEY")?;
    let client = Client::new();

    client
        .patch(&format!("https://api.vapi.ai/agents/{}", agent_id))
        .header("Authorization", format!("Bearer {}", vapi_api_key))
        .json(&json!({
            "name": name,
            "firstMessage": greeting
        }))
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

use postgrest::Postgrest;
use serde_json::Value;

use dotenv::dotenv;

use serde::{Deserialize, Serialize};

use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadVaultSecretInput {
    secret_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSecretInput {
    name: String,
    secret: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSecretInput {
    id: String,
    secret: String,
    name: String,
    description: String,
}

pub async fn insert_secret_to_vault(
    client: &Postgrest,
    secret_name: &str,
    secret_value: &str,
    description: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("[VAULT] Starting insert_secret_to_vault");
    
    // Validate secret value is not empty or whitespace-only
    if secret_value.trim().is_empty() {
        println!("[VAULT] Error: Secret value cannot be empty or whitespace-only");
        return Err("Secret value cannot be empty or whitespace-only".into());
    }

    // Validate secret name is not empty
    if secret_name.trim().is_empty() {
        println!("[VAULT] Error: Secret name cannot be empty");
        return Err("Secret name cannot be empty".into());
    }

    println!("[VAULT] Loading environment variables");
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let secret_input = CreateSecretInput {
        name: secret_name.to_string(),
        secret: secret_value.to_string(), // Use original value, we've already validated it contains non-whitespace
        description: description.to_string(),
    };

    println!("[VAULT] Making RPC call to insert_secret with input: {:?}", secret_input);

    let response = client
        .rpc(
            "insert_secret",
            serde_json::to_string(&secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await?;

    let body = response.text().await?;

    println!("[VAULT] Response from vault insert: {:?}", body);

    // Parse the response body as JSON
    let json_response: Value = serde_json::from_str(&body)?;

    // Check if there's an error in the response
    if let Some(error) = json_response.get("code") {
        let error_code = error.as_str().unwrap_or("Unknown");
        let error_message = json_response["message"].as_str().unwrap_or("Unknown error");

        println!("[VAULT] Error in response - code: {}, message: {}", error_code, error_message);

        if error_code == "23505" {
            return Err(format!("Duplicate key error: {}", error_message).into());
        } else {
            return Err(format!("Database error: {} - {}", error_code, error_message).into());
        }
    }

    // If no error, extract the secret_vault_id
    let secret_vault_id = json_response
        .as_str()
        .ok_or("Invalid response format")?
        .trim_matches('"')
        .to_string();

    println!("[VAULT] Successfully inserted secret with vault_id: {}", secret_vault_id);

    Ok(secret_vault_id)
}

pub async fn update_secret_in_vault(
    client: &Postgrest,
    secret_id: &str,
    new_secret_value: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[VAULT] Starting update_secret_in_vault for secret_id: {}", secret_id);

    // Validate new secret value is not empty or whitespace-only
    if new_secret_value.trim().is_empty() {
        println!("[VAULT] Error: Secret value cannot be empty or whitespace-only");
        return Err("Secret value cannot be empty or whitespace-only".into());
    }

    // Validate secret ID is not empty
    if secret_id.trim().is_empty() {
        println!("[VAULT] Error: Secret ID cannot be empty");
        return Err("Secret ID cannot be empty".into());
    }

    println!("[VAULT] Loading environment variables");
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let read_secret_input = ReadVaultSecretInput {
        secret_id: secret_id.to_string(),
    };
    
    println!("[VAULT] Fetching existing secret details");
    //TODO: fetch existing secret to populate name and description
    // Read Secret in Vault
    let response = client
        .rpc(
            "read_secret",
            serde_json::to_string(&read_secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key.clone()) //Need to put service role key here I guess for it to show up current_setting in sql function
        .execute()
        .await?;

    let vault_secret_body = response.text().await?;

    println!("[VAULT] Existing secret details response: {:?}", vault_secret_body);

    let vault_secret_json: serde_json::Value = serde_json::from_str(&vault_secret_body).unwrap();
    let secret_name = vault_secret_json[0]["name"].as_str().unwrap_or_default();
    let secret_description = vault_secret_json[0]["description"]
        .as_str()
        .unwrap_or_default();

    println!("[VAULT] Retrieved existing secret name: {}", secret_name);

    let update_secret_input = UpdateSecretInput {
        id: secret_id.to_string(),
        secret: new_secret_value.to_string(), // Use original value, we've already validated it contains non-whitespace
        name: secret_name.to_string(),
        description: secret_description.to_string(),
    };

    println!("[VAULT] Making RPC call to update_secret with input: {:?}", update_secret_input);

    let response = client
        .rpc(
            "update_secret",
            serde_json::to_string(&update_secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await?;

    let body = response.text().await?;

    println!("[VAULT] Response from vault update: {:?}", body);
    println!("[VAULT] Successfully updated secret");

    Ok(())
}

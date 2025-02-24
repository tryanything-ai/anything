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
    // Validate secret value is not empty
    if secret_value.trim().is_empty() {
        return Err("Secret value cannot be empty".into());
    }

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let secret_input = CreateSecretInput {
        name: secret_name.to_string(),
        secret: secret_value.to_string(),
        description: description.to_string(),
    };

    println!("insert_secret rpc Input: {:?}", secret_input);

    let response = client
        .rpc(
            "insert_secret",
            serde_json::to_string(&secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await?;

    let body = response.text().await?;

    println!("Response from vault insert: {:?}", body);

    // Parse the response body as JSON
    let json_response: Value = serde_json::from_str(&body)?;

    // Check if there's an error in the response
    if let Some(error) = json_response.get("code") {
        let error_code = error.as_str().unwrap_or("Unknown");
        let error_message = json_response["message"].as_str().unwrap_or("Unknown error");

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

    Ok(secret_vault_id)
}

pub async fn update_secret_in_vault(
    client: &Postgrest,
    secret_id: &str,
    new_secret_value: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate new secret value is not empty
    if new_secret_value.trim().is_empty() {
        return Err("Secret value cannot be empty".into());
    }

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let read_secret_input = ReadVaultSecretInput {
        secret_id: secret_id.to_string(),
    };
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

    println!("Vault Secret Body: {:?}", vault_secret_body);

    let vault_secret_json: serde_json::Value = serde_json::from_str(&vault_secret_body).unwrap();
    let secret_name = vault_secret_json[0]["name"].as_str().unwrap_or_default();
    let secret_description = vault_secret_json[0]["description"]
        .as_str()
        .unwrap_or_default();

    println!("Secret Name: {:?}", secret_name);

    let update_secret_input = UpdateSecretInput {
        id: secret_id.to_string(),
        secret: new_secret_value.to_string(),
        name: secret_name.to_string(),
        description: secret_description.to_string(),
    };

    println!("update_secret rpc Input: {:?}", update_secret_input);

    let response = client
        .rpc(
            "update_secret",
            serde_json::to_string(&update_secret_input).unwrap(),
        )
        .auth(supabase_service_role_api_key)
        .execute()
        .await?;

    let body = response.text().await?;

    println!("Response from vault update: {:?}", body);

    Ok(())
}

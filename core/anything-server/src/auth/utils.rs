use postgrest::Postgrest;
use serde_json::Value;
use slugify::slugify;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use dotenv::dotenv;
use rand::Rng;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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

pub async fn generate_code_challenge(code_verifier: &str) -> String {
    let hash = Sha256::digest(code_verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(&hash)
}

// Helper function to generate a random string
pub fn generate_random_string(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            charset
                .chars()
                .nth(rng.gen_range(0..charset.len()))
                .unwrap()
        })
        .collect()
}

pub fn generate_code_verifier() -> String {
    generate_random_string(43) // Between 43-128 characters
}

pub async fn generate_unique_account_slug(
    client: &Postgrest,
    base_slug: &str,
    account_id: &str,
) -> (String, String) {
    let mut slug = slugify!(base_slug, separator = "_").to_uppercase();

    println!("Base slug at start: {}", slug);
    let mut counter = 1;

    dotenv().ok();
    let supabase_service_role_api_key = match env::var("SUPABASE_SERVICE_ROLE_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error fetching SUPABASE_SERVICE_ROLE_API_KEY: {}", e);
            return (slug.clone(), base_slug.to_string());
        }
    };

    //never go over 100. just like sanity check.
    for _ in 0..100 {
        println!(
            "Attempting to fetch existing slugs for slug: {} and account_id: {}",
            slug, account_id
        );
        let response = match client
            .from("account_auth_provider_accounts")
            .auth(supabase_service_role_api_key.clone())
            .select("account_auth_provider_account_slug")
            .eq("account_auth_provider_account_slug", &slug)
            .eq("account_id", account_id)
            .execute()
            .await
        {
            Ok(response) => {
                println!("Received response for slug check: {:?}", response);
                response
            }
            Err(e) => {
                eprintln!("Error executing request to fetch slugs: {}", e);
                return (slug.clone(), base_slug.to_string());
            }
        };

        let body = match response.text().await {
            Ok(body) => {
                println!("Received body for slug check: {}", body);
                body
            }
            Err(e) => {
                eprintln!("Error reading response body: {}", e);
                return (slug.clone(), base_slug.to_string());
            }
        };

        let existing_slugs: Vec<Value> = match serde_json::from_str(&body) {
            Ok(items) => {
                println!("Parsed existing slugs: {:?}", items);
                items
            }
            Err(e) => {
                eprintln!("Error parsing JSON response: {}", e);
                return (slug.clone(), base_slug.to_string());
            }
        };

        if existing_slugs.is_empty() {
            println!("Using Unique slug generated: {}", slug);
            break;
        }

        slug = slugify!(
            format!("{}_{}", base_slug, counter).as_str(),
            separator = "_"
        )
        .to_uppercase();
        println!("Trying another slug: {}", slug);
        counter += 1;
    }

    let human_readable_slug = slug
        .replace('_', " ")
        .to_lowercase()
        .split_whitespace()
        .enumerate()
        .map(|(i, word)| {
            if i == 1 && word.chars().all(char::is_numeric) {
                word.to_string()
            } else {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!(
        "Final slug: {}, Human readable slug: {}",
        slug, human_readable_slug
    );

    (slug, human_readable_slug)
}

pub async fn insert_secret_to_vault(
    client: &Postgrest,
    secret_name: &str,
    secret_value: &str,
    description: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
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

    let secret_vault_id = body.trim_matches('"').to_string();

    Ok(secret_vault_id)
}
pub async fn update_secret_in_vault(
    client: &Postgrest,
    secret_id: &str,
    new_secret_value: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

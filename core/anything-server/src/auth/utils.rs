use postgrest::Postgrest;
use serde_json::Value;
use slugify::slugify;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use dotenv::dotenv;
use rand::Rng;

use sha2::{Digest, Sha256};
use std::env;


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

// use postgrest::Postgrest; // Removed - using SeaORM instead
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
    base_slug: &str,
    account_id: &str,
) -> (String, String) {
    let mut slug = slugify!(base_slug, separator = "_").to_uppercase();

    println!("Base slug at start: {}", slug);
    
    // TODO: Replace with SeaORM query to check for existing slugs
    // For now, just use the base slug
    println!("TODO: Check for existing slugs using SeaORM");

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

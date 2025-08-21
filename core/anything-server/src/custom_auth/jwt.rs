use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // Username
    pub session_id: String, // Session ID for revocation
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, session_id: Uuid) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours
        
        Self {
            sub: user_id.to_string(),
            username,
            session_id: session_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new() -> Result<Self> {
        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-very-secret-jwt-key-change-this-in-production".to_string());
        
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        
        Ok(Self {
            encoding_key,
            decoding_key,
        })
    }
    
    pub fn create_token(&self, claims: &Claims) -> Result<String> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to create JWT token: {}", e))
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| anyhow!("Failed to verify JWT token: {}", e))?;
        
        Ok(token_data.claims)
    }
}

impl Default for JwtManager {
    fn default() -> Self {
        Self::new().expect("Failed to create JWT manager")
    }
}

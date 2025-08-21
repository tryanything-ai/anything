use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::pgsodium_secrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyUser {
    pub account_id: String,
}

pub async fn api_key_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get the API key from the Authorization header
    let api_key = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(header) if header.starts_with("Bearer ") => header[7..].to_string(),
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Get the state from the request extensions
    let state = request
        .extensions()
        .get::<Arc<crate::AppState>>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Implement API key validation with pgsodium_secrets
    // For now, reject all API key requests until the secret validation is implemented
    println!("API key validation not implemented yet - rejecting request");
    Err(StatusCode::UNAUTHORIZED)
}

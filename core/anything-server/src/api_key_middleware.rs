use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::secrets;

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

    // Check if the API key exists and is valid in the database
    let secret = match secrets::get_secret_by_secret_value(state.clone(), api_key).await {
        Ok(secret) => secret,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify this is an API key secret
    if !secret.anything_api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Add the user info to request extensions
    let api_key_user = ApiKeyUser {
        account_id: secret.account_id,
    };
    request.extensions_mut().insert(api_key_user);

    Ok(next.run(request).await)
}

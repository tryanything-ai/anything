use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;
use sha2::{Sha256, Digest};

use crate::entities::{users, user_sessions, user_accounts};
use crate::AppState;
use super::{jwt::JwtManager, jwt::Claims};

/// Middleware to validate JWT tokens and ensure session is active
pub async fn jwt_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify JWT token
    let jwt_manager = JwtManager::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let claims = jwt_manager.verify_token(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if token is expired
    let now = chrono::Utc::now().timestamp();
    if claims.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Verify session exists in database and is not expired
    let session_id = Uuid::parse_str(&claims.session_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let session = user_sessions::Entity::find_by_id(session_id)
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if session is expired
    if session.expires_at < chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Verify token hash matches
    let token_hash = Sha256::digest(auth_header.as_bytes());
    if session.token_hash != hex::encode(token_hash) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Verify user exists and is active
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let user = users::Entity::find_by_id(user_id)
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !user.is_active {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create User for backward compatibility with existing endpoints
    // For now, we'll use the first account as the account_id context
    // TODO: Improve this to handle multiple accounts per user properly
    let user_accounts_data = user_accounts::Entity::find()
        .filter(user_accounts::Column::UserId.eq(user.user_id))
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let account_id = if let Some(user_account) = user_accounts_data {
        user_account.account_id.to_string()
    } else {
        // If no account relationship exists, we may need to create one or handle this case
        // For now, return unauthorized
        return Err(StatusCode::UNAUTHORIZED);
    };

    let compat_user = crate::custom_auth::User::from_auth_user_with_jwt(&user, account_id, auth_header.to_string());

    // Add claims and user info to request extensions for use in handlers
    request.extensions_mut().insert(claims);
    request.extensions_mut().insert(user);
    request.extensions_mut().insert(compat_user);

    Ok(next.run(request).await)
}

/// Extract claims from request extensions (for use in handlers)
pub fn extract_claims(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}

/// Extract user from request extensions (for use in handlers)
pub fn extract_user(request: &Request) -> Option<&users::Model> {
    request.extensions().get::<users::Model>()
}

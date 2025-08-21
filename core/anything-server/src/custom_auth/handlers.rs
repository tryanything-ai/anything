use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use chrono::{Duration, Utc};
use std::sync::Arc;
use sha2::{Sha256, Digest};

use crate::entities::{users, user_sessions, user_accounts};
use crate::AppState;
use super::{password, jwt::Claims, jwt::JwtManager, extractors::AuthClaims};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
    pub expires_in: i64, // Seconds until token expires
}

#[derive(Serialize)]
pub struct UserInfo {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub accounts: Vec<UserAccount>,
}

#[derive(Serialize)]
pub struct UserAccount {
    pub account_id: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Register a new user
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<ResponseJson<AuthResponse>, StatusCode> {
    // Validate input
    if request.username.trim().is_empty() || request.email.trim().is_empty() || request.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let existing_user = users::Entity::find()
        .filter(
            users::Column::Username.eq(&request.username)
                .or(users::Column::Email.eq(&request.email))
        )
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        return Err(StatusCode::CONFLICT); // User already exists
    }

    // Hash password
    let password_hash = password::hash_password(&request.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create user
    let user_id = Uuid::new_v4();
    let new_user = users::ActiveModel {
        user_id: Set(user_id),
        username: Set(request.username.clone()),
        email: Set(request.email.clone()),
        password_hash: Set(password_hash),
        is_active: Set(true),
        email_verified: Set(false), // Require email verification in production
        failed_login_attempts: Set(0),
        ..Default::default()
    };

    let user = new_user.insert(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create initial session
    let session_id = Uuid::new_v4();
    let token_expires = Utc::now() + Duration::hours(24);
    
    let claims = Claims::new(user.user_id, user.username.clone(), session_id);
    let jwt_manager = JwtManager::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let token = jwt_manager.create_token(&claims)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store session in database
    let token_hash = Sha256::digest(token.as_bytes());
    let new_session = user_sessions::ActiveModel {
        session_id: Set(session_id),
        user_id: Set(user.user_id),
        token_hash: Set(hex::encode(token_hash)),
        expires_at: Set(token_expires.into()),
        user_agent: Set(None), // TODO: Extract from headers
        ip_address: Set(None), // TODO: Extract from request
        ..Default::default()
    };

    new_session.insert(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get user accounts (empty for new user)
    let accounts = vec![];

    let response = AuthResponse {
        token,
        user: UserInfo {
            user_id: user.user_id.to_string(),
            username: user.username,
            email: user.email,
            accounts,
        },
        expires_in: 86400, // 24 hours in seconds
    };

    Ok(ResponseJson(response))
}

/// Login user
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<ResponseJson<AuthResponse>, StatusCode> {
    // Find user by username
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&request.username))
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if user is active and not locked
    if !user.is_active {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if let Some(locked_until) = user.locked_until {
        if locked_until > Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()) {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // Verify password
    let password_valid = password::verify_password(&request.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !password_valid {
        // Increment failed login attempts
        let mut user_update: users::ActiveModel = user.clone().into();
        user_update.failed_login_attempts = Set(user.failed_login_attempts + 1);
        
        // Lock account after 5 failed attempts
        if user.failed_login_attempts >= 4 {
            user_update.locked_until = Set(Some((Utc::now() + Duration::hours(1)).into()));
        }
        
        user_update.update(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Reset failed login attempts on successful login
    if user.failed_login_attempts > 0 {
        let mut user_update: users::ActiveModel = user.clone().into();
        user_update.failed_login_attempts = Set(0);
        user_update.locked_until = Set(None);
        user_update.last_login_at = Set(Some(Utc::now().into()));
        
        user_update.update(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Create session
    let session_id = Uuid::new_v4();
    let token_expires = Utc::now() + Duration::hours(24);
    
    let claims = Claims::new(user.user_id, user.username.clone(), session_id);
    let jwt_manager = JwtManager::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let token = jwt_manager.create_token(&claims)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store session in database
    let token_hash = Sha256::digest(token.as_bytes());
    let new_session = user_sessions::ActiveModel {
        session_id: Set(session_id),
        user_id: Set(user.user_id),
        token_hash: Set(hex::encode(token_hash)),
        expires_at: Set(token_expires.into()),
        user_agent: Set(None), // TODO: Extract from headers
        ip_address: Set(None), // TODO: Extract from request
        ..Default::default()
    };

    new_session.insert(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get user accounts
    let user_accounts_data = user_accounts::Entity::find()
        .filter(user_accounts::Column::UserId.eq(user.user_id))
        .all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let accounts: Vec<UserAccount> = user_accounts_data
        .into_iter()
        .map(|ua| UserAccount {
            account_id: ua.account_id.to_string(),
            role: ua.role,
        })
        .collect();

    let response = AuthResponse {
        token,
        user: UserInfo {
            user_id: user.user_id.to_string(),
            username: user.username,
            email: user.email,
            accounts,
        },
        expires_in: 86400, // 24 hours in seconds
    };

    Ok(ResponseJson(response))
}

/// Logout user (revoke session)
pub async fn logout(
    State(state): State<Arc<AppState>>,
    AuthClaims(claims): AuthClaims, // This would be extracted by middleware
) -> Result<ResponseJson<MessageResponse>, StatusCode> {
    let session_id = Uuid::parse_str(&claims.session_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Delete the session from database
    user_sessions::Entity::delete_by_id(session_id)
        .exec(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(MessageResponse {
        message: "Logged out successfully".to_string(),
    }))
}

/// Get current user info
pub async fn me(
    State(state): State<Arc<AppState>>,
    AuthClaims(claims): AuthClaims, // This would be extracted by middleware
) -> Result<ResponseJson<UserInfo>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let user = users::Entity::find_by_id(user_id)
        .one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Get user accounts
    let user_accounts_data = user_accounts::Entity::find()
        .filter(user_accounts::Column::UserId.eq(user.user_id))
        .all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let accounts: Vec<UserAccount> = user_accounts_data
        .into_iter()
        .map(|ua| UserAccount {
            account_id: ua.account_id.to_string(),
            role: ua.role,
        })
        .collect();

    let response = UserInfo {
        user_id: user.user_id.to_string(),
        username: user.username,
        email: user.email,
        accounts,
    };

    Ok(ResponseJson(response))
}

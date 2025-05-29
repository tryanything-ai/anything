use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::{supabase_jwt_middleware::User, AppState};

// Cache entry with expiration
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedAccess {
    has_access: bool,
    expires_at: SystemTime,
}

// Cache key combining user_id and account_id
#[derive(Hash, Eq, PartialEq, Clone)]
struct AccessCacheKey {
    user_id: String,
    account_id: String,
}

// Add this to your AppState struct in main.rs
pub struct AccountAccessCache {
    cache: DashMap<AccessCacheKey, CachedAccess>,
    ttl: Duration,
}

impl AccountAccessCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: DashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, user_id: &str, account_id: &str) -> Option<bool> {
        let key = AccessCacheKey {
            user_id: user_id.to_string(),
            account_id: account_id.to_string(),
        };
        let result = self.cache.get(&key).and_then(|entry| {
            if entry.expires_at > SystemTime::now() {
                Some(entry.has_access)
            } else {
                None
            }
        });

        result
    }

    pub fn set(&self, user_id: &str, account_id: &str, has_access: bool) {
        let key = AccessCacheKey {
            user_id: user_id.to_string(),
            account_id: account_id.to_string(),
        };
        let expires_at = SystemTime::now() + self.ttl;
        let entry = CachedAccess {
            has_access,
            expires_at,
        };
        self.cache.insert(key, entry);
    }

    // Cleanup expired entries
    pub fn cleanup(&self) {
        let before_count = self.cache.len();
        self.cache
            .retain(|_, entry| entry.expires_at > SystemTime::now());
        let after_count = self.cache.len();
    }
}

async fn verify_account_access(
    client: &postgrest::Postgrest,
    jwt: &str,
    user_id: &str,
    account_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let response = client
        .rpc(
            "get_account",
            serde_json::to_string(&json!({ "account_id": account_id })).unwrap(),
        )
        .auth(jwt)
        .execute()
        .await?;

    // If the user doesn't have access, get_account will return a 404/401
    // So if we get a successful response, the user has access
    let has_access = response.status().is_success();

    Ok(has_access)
}

pub async fn account_access_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract user_id from the existing auth middleware
    let user = request.extensions().get::<User>().ok_or_else(|| {
        StatusCode::UNAUTHORIZED
    })?;
    let user_id = &user.account_id;

    // Extract account_id from path parameters
    let account_id = request
        .uri()
        .path()
        .split('/')
        .nth(2) // "account" is at index 1, so account_id will be at index 2
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Check cache first
    let mut needs_db_check = true;
    let has_access = {
        if let Some(cached_access) = state.account_access_cache.get(user_id, account_id) {
            needs_db_check = false;
            cached_access
        } else {
            false
        }
    };

    if !needs_db_check {
        if !has_access {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        // Verify access in database
        let db_has_access =
            verify_account_access(&state.public_client, &user.jwt, user_id, account_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Update cache
        state
            .account_access_cache
            .set(user_id, account_id, db_has_access);

        if !db_has_access {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    Ok(next.run(request).await)
}

// Periodic cache cleanup task
pub async fn cleanup_account_access_cache(state: Arc<AppState>) {
    let cleanup_interval = Duration::from_secs(3600); // Run cleanup every hour
    loop {
        tokio::time::sleep(cleanup_interval).await;
        state.account_access_cache.cleanup();
    }
}

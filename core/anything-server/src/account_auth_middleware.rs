use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::info;

use crate::AppState;

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
    cache: HashMap<AccessCacheKey, CachedAccess>,
    ttl: Duration,
}

impl AccountAccessCache {
    pub fn new(ttl: Duration) -> Self {
        info!(
            "[ACCOUNT MIDDLEWARE] Creating new AccountAccessCache with TTL: {:?}",
            ttl
        );
        Self {
            cache: HashMap::new(),
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
        info!(
            "[ACCOUNT MIDDLEWARE] Cache lookup for user_id: {}, account_id: {} -> result: {:?}",
            user_id, account_id, result
        );
        result
    }

    pub fn set(&mut self, user_id: &str, account_id: &str, has_access: bool) {
        info!(
            "[ACCOUNT MIDDLEWARE] Setting cache for user_id: {}, account_id: {} -> has_access: {}",
            user_id, account_id, has_access
        );
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

    pub fn remove(&mut self, user_id: &str, account_id: &str) {
        info!(
            "[ACCOUNT MIDDLEWARE] Removing cache entry for user_id: {}, account_id: {}",
            user_id, account_id
        );
        let key = AccessCacheKey {
            user_id: user_id.to_string(),
            account_id: account_id.to_string(),
        };
        self.cache.remove(&key);
    }

    // Cleanup expired entries
    pub fn cleanup(&mut self) {
        info!("[ACCOUNT MIDDLEWARE] Starting cache cleanup");
        let before_count = self.cache.len();
        self.cache
            .retain(|_, entry| entry.expires_at > SystemTime::now());
        let after_count = self.cache.len();
        info!(
            "[ACCOUNT MIDDLEWARE] Cache cleanup complete - removed {} entries",
            before_count - after_count
        );
    }
}

async fn verify_account_access(
    client: &postgrest::Postgrest,
    user_id: &str,
    account_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    info!(
        "[ACCOUNT MIDDLEWARE] Verifying account access for user_id: {}, account_id: {}",
        user_id, account_id
    );
    let response = client
        .from("account_user")
        .select("*")
        .eq("user_id", user_id)
        .eq("account_id", account_id)
        .execute()
        .await?;

    let accounts: Vec<serde_json::Value> = response.json().await?;
    let has_access = !accounts.is_empty();
    info!(
        "[ACCOUNT MIDDLEWARE] Database access check result: {}",
        has_access
    );
    Ok(has_access)
}

pub async fn account_access_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    info!("[ACCOUNT MIDDLEWARE] Processing request");

    // Extract user_id from the existing auth middleware
    let user_id = request
        .extensions()
        .get::<String>()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    info!("[ACCOUNT MIDDLEWARE] Extracted user_id: {}", user_id);

    // Extract account_id from path parameters
    let account_id = request
        .uri()
        .path()
        .split('/')
        .find(|segment| segment.starts_with("account_"))
        .map(|s| s.trim_start_matches("account_"))
        .ok_or(StatusCode::BAD_REQUEST)?;
    info!("[ACCOUNT MIDDLEWARE] Extracted account_id: {}", account_id);

    // Check cache first
    let mut needs_db_check = true;
    let has_access = {
        let cache = state.account_access_cache.read().await;
        if let Some(cached_access) = cache.get(user_id, account_id) {
            needs_db_check = false;
            cached_access
        } else {
            false
        }
    };
    info!(
        "[ACCOUNT MIDDLEWARE] Cache check - needs_db_check: {}, has_access: {}",
        needs_db_check, has_access
    );

    if !needs_db_check {
        if !has_access {
            info!("[ACCOUNT MIDDLEWARE] Access denied from cache");
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        // Verify access in database
        let db_has_access = verify_account_access(&state.anything_client, user_id, account_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Update cache
        {
            let mut cache = state.account_access_cache.write().await;
            cache.set(user_id, account_id, db_has_access);
        }

        if !db_has_access {
            info!("[ACCOUNT MIDDLEWARE] Access denied from database check");
            return Err(StatusCode::FORBIDDEN);
        }
    }

    info!("[ACCOUNT MIDDLEWARE] Access granted, proceeding with request");
    Ok(next.run(request).await)
}

// Periodic cache cleanup task
pub async fn cleanup_account_access_cache(state: Arc<AppState>) {
    let cleanup_interval = Duration::from_secs(3600); // Run cleanup every hour
    info!(
        "[ACCOUNT MIDDLEWARE] Starting periodic cache cleanup task with interval: {:?}",
        cleanup_interval
    );
    loop {
        tokio::time::sleep(cleanup_interval).await;
        info!("[ACCOUNT MIDDLEWARE] Running scheduled cache cleanup");
        let mut cache = state.account_access_cache.write().await;
        cache.cleanup();
    }
}

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
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
    cache: HashMap<AccessCacheKey, CachedAccess>,
    ttl: Duration,
}

impl AccountAccessCache {
    pub fn new(ttl: Duration) -> Self {
        println!(
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
        println!(
            "[ACCOUNT MIDDLEWARE] Cache lookup for user_id: {}, account_id: {} -> result: {:?}",
            user_id, account_id, result
        );
        result
    }

    pub fn set(&mut self, user_id: &str, account_id: &str, has_access: bool) {
        println!(
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


    // Cleanup expired entries
    pub fn cleanup(&mut self) {
        println!("[ACCOUNT MIDDLEWARE] Starting cache cleanup");
        let before_count = self.cache.len();
        self.cache
            .retain(|_, entry| entry.expires_at > SystemTime::now());
        let after_count = self.cache.len();
        println!(
            "[ACCOUNT MIDDLEWARE] Cache cleanup complete - removed {} entries",
            before_count - after_count
        );
    }
}


async fn verify_account_access(
    client: &postgrest::Postgrest,
    jwt: &str,
    user_id: &str,
    account_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!(
        "[VERIFY_ACCOUNT_ACCESS] Starting verification for user_id: {}, account_id: {}",
        user_id, account_id
    );

    println!(
        "[VERIFY_ACCOUNT_ACCESS] Making RPC call to get_account with payload: {}",
        serde_json::to_string(&json!({ "account_id": account_id })).unwrap()
    );

    let response = client
        .rpc(
            "get_account",
            serde_json::to_string(&json!({ "account_id": account_id })).unwrap(),
        )
        .auth(jwt)
        .execute()
        .await?;

    println!(
        "[VERIFY_ACCOUNT_ACCESS] Response status: {}",
        response.status()
    );

    println!(
        "[VERIFY_ACCOUNT_ACCESS] Response headers: {:?}",
        response.headers()
    );

    // If the user doesn't have access, get_account will return a 404/401
    // So if we get a successful response, the user has access
    let has_access = response.status().is_success();

    println!(
        "[VERIFY_ACCOUNT_ACCESS] Access determination: {}",
        if has_access { "GRANTED" } else { "DENIED" }
    );

    Ok(has_access)
}

pub async fn account_access_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("[ACCOUNT MIDDLEWARE] Processing request");

    // Extract user_id from the existing auth middleware
    println!("[ACCOUNT MIDDLEWARE] Attempting to extract user from request extensions");
    let user = request.extensions().get::<User>().ok_or_else(|| {
        println!("[ACCOUNT MIDDLEWARE] No user found in request extensions");
        StatusCode::UNAUTHORIZED
    })?;
    println!("[ACCOUNT MIDDLEWARE] Successfully found user in request extensions");
    let user_id = &user.account_id;
    println!("[ACCOUNT MIDDLEWARE] Extracted user_id: {}", user_id);
    // println!("[ACCOUNT MIDDLEWARE] User email: {}", user.email.as_deref().unwrap_or("no email"));

    // Extract account_id from path parameters
    println!("[ACCOUNT MIDDLEWARE] Path: {}", request.uri().path());
    let account_id = request
        .uri()
        .path()
        .split('/')
        .nth(2) // "account" is at index 1, so account_id will be at index 2
        .ok_or(StatusCode::BAD_REQUEST)?;
    println!("[ACCOUNT MIDDLEWARE] Extracted account_id: {}", account_id);
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
    println!(
        "[ACCOUNT MIDDLEWARE] Cache check - needs_db_check: {}, has_access: {}",
        needs_db_check, has_access
    );

    if !needs_db_check {
        if !has_access {
            println!("[ACCOUNT MIDDLEWARE] Access denied from cache");
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        // Verify access in database
        let db_has_access =
            verify_account_access(&state.public_client, &user.jwt, user_id, account_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Update cache
        {
            let mut cache = state.account_access_cache.write().await;
            cache.set(user_id, account_id, db_has_access);
        }

        if !db_has_access {
            println!("[ACCOUNT MIDDLEWARE] Access denied from database check");
            return Err(StatusCode::FORBIDDEN);
        }
    }

    println!("[ACCOUNT MIDDLEWARE] Access granted, proceeding with request");
    Ok(next.run(request).await)
}

// Periodic cache cleanup task
pub async fn cleanup_account_access_cache(state: Arc<AppState>) {
    let cleanup_interval = Duration::from_secs(3600); // Run cleanup every hour
    println!(
        "[ACCOUNT MIDDLEWARE] Starting periodic cache cleanup task with interval: {:?}",
        cleanup_interval
    );
    loop {
        tokio::time::sleep(cleanup_interval).await;
        println!("[ACCOUNT MIDDLEWARE] Running scheduled cache cleanup");
        let mut cache = state.account_access_cache.write().await;
        cache.cleanup();
    }
}

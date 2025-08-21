use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use crate::{custom_auth::User, entities::user_accounts, AppState};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

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

// Account access cache
pub struct AccountAccessCache {
    cache: DashMap<AccessCacheKey, CachedAccess>,
    ttl: Duration,
}

impl AccountAccessCache {
    pub fn new(ttl: Duration) -> Self {
        println!(
            "[ACCOUNT MIDDLEWARE] Creating new AccountAccessCache with TTL: {:?}",
            ttl
        );
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

        if let Some(cached) = self.cache.get(&key) {
            if cached.expires_at > SystemTime::now() {
                println!("[ACCOUNT MIDDLEWARE] Cache hit for {}:{}", user_id, account_id);
                return Some(cached.has_access);
            } else {
                println!("[ACCOUNT MIDDLEWARE] Cache expired for {}:{}", user_id, account_id);
                self.cache.remove(&key);
            }
        }

        println!("[ACCOUNT MIDDLEWARE] Cache miss for {}:{}", user_id, account_id);
        None
    }

    pub fn set(&self, user_id: &str, account_id: &str, has_access: bool) {
        let key = AccessCacheKey {
            user_id: user_id.to_string(),
            account_id: account_id.to_string(),
        };

        let cached = CachedAccess {
            has_access,
            expires_at: SystemTime::now() + self.ttl,
        };

        println!(
            "[ACCOUNT MIDDLEWARE] Caching result for {}:{} = {}",
            user_id, account_id, has_access
        );
        self.cache.insert(key, cached);
    }

    pub fn cleanup_expired(&self) {
        let now = SystemTime::now();
        let expired_keys: Vec<_> = self
            .cache
            .iter()
            .filter_map(|entry| {
                if entry.value().expires_at <= now {
                    Some(entry.key().clone())
                } else {
                    None
                }
            })
            .collect();

        for key in expired_keys {
            self.cache.remove(&key);
        }

        println!("[ACCOUNT MIDDLEWARE] Cleaned up expired cache entries");
    }
}

// Verify account access using SeaORM
async fn verify_account_access_seaorm(
    state: &AppState,
    user_id: &str,
    account_id: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[VERIFY_ACCOUNT_ACCESS] Checking access for user {} to account {}",
        user_id, account_id
    );

    // Parse UUIDs
    let user_uuid = match Uuid::parse_str(user_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            println!("[VERIFY_ACCOUNT_ACCESS] Invalid user ID format");
            return Ok(false);
        }
    };

    let account_uuid = match Uuid::parse_str(account_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            println!("[VERIFY_ACCOUNT_ACCESS] Invalid account ID format");
            return Ok(false);
        }
    };

    // Check if user has access to this account using SeaORM
    let user_account = match user_accounts::Entity::find()
        .filter(user_accounts::Column::UserId.eq(user_uuid))
        .filter(user_accounts::Column::AccountId.eq(account_uuid))
        .filter(user_accounts::Column::Active.eq(true))
        .one(&*state.db)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            println!("[VERIFY_ACCOUNT_ACCESS] Database error: {:?}", err);
            return Err(Box::new(err));
        }
    };

    let has_access = user_account.is_some();

    println!(
        "[VERIFY_ACCOUNT_ACCESS] Access determination: {}",
        if has_access { "GRANTED" } else { "DENIED" }
    );

    Ok(has_access)
}

// Account access middleware using SeaORM
pub async fn account_access_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract user from request extensions (set by JWT middleware)
    let user = match req.extensions().get::<User>() {
        Some(user) => user.clone(),
        None => {
            println!("[ACCOUNT MIDDLEWARE] No user found in request extensions");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Extract account_id from path - this assumes the URL pattern has :account_id
    let path = req.uri().path();
    let account_id = extract_account_id_from_path(path)?;

    println!(
        "[ACCOUNT MIDDLEWARE] Checking access for user {} to account {}",
        user.id, account_id
    );

    // Check cache first
    if let Some(cached_access) = state
        .account_access_cache
        .get(&user.id.to_string(), &account_id)
    {
        if cached_access {
            println!("[ACCOUNT MIDDLEWARE] Access granted from cache");
            return Ok(next.run(req).await);
        } else {
            println!("[ACCOUNT MIDDLEWARE] Access denied from cache");
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Not in cache, verify access using database
    match verify_account_access_seaorm(&state, &user.id.to_string(), &account_id).await {
        Ok(has_access) => {
            // Cache the result
            state
                .account_access_cache
                .set(&user.id.to_string(), &account_id, has_access);

            if has_access {
                println!("[ACCOUNT MIDDLEWARE] Access granted from database");
                Ok(next.run(req).await)
            } else {
                println!("[ACCOUNT MIDDLEWARE] Access denied from database");
                Err(StatusCode::FORBIDDEN)
            }
        }
        Err(err) => {
            println!("[ACCOUNT MIDDLEWARE] Error verifying access: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Extract account_id from URL path
fn extract_account_id_from_path(path: &str) -> Result<String, StatusCode> {
    // Split path and look for account_id after "/account/"
    let parts: Vec<&str> = path.split('/').collect();
    
    for (i, part) in parts.iter().enumerate() {
        if *part == "account" && i + 1 < parts.len() {
            let account_id = parts[i + 1];
            if !account_id.is_empty() {
                return Ok(account_id.to_string());
            }
        }
    }

    println!("[ACCOUNT MIDDLEWARE] Could not extract account_id from path: {}", path);
    Err(StatusCode::BAD_REQUEST)
}

// Cleanup task for expired cache entries
pub async fn cleanup_account_access_cache(state: Arc<AppState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes

    loop {
        interval.tick().await;
        println!("[ACCOUNT MIDDLEWARE] Running cache cleanup task");
        state.account_access_cache.cleanup_expired();
    }
}

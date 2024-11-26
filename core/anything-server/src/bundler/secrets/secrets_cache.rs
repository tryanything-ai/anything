use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::debug;

use crate::bundler::secrets::DecryptedSecret;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedSecret {
    secret: DecryptedSecret,
    expires_at: SystemTime,
}

pub struct SecretsCache {
    cache: HashMap<String, Vec<CachedSecret>>, // account_id -> secrets
    ttl: Duration,
}

impl SecretsCache {
    pub fn new(ttl: Duration) -> Self {
        debug!("[BUNDLER] Creating new SecretsCache with TTL: {:?}", ttl);
        Self {
            cache: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, account_id: &str) -> Option<Vec<DecryptedSecret>> {
        self.cache.get(account_id).and_then(|entries| {
            let now = SystemTime::now();
            if entries.iter().all(|entry| entry.expires_at > now) {
                Some(entries.iter().map(|e| e.secret.clone()).collect())
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, account_id: &str, secrets: Vec<DecryptedSecret>) {
        debug!(
            "[BUNDLER] Setting secrets cache for account_id: {}",
            account_id
        );
        let expires_at = SystemTime::now() + self.ttl;
        let cached_secrets = secrets
            .into_iter()
            .map(|secret| CachedSecret { secret, expires_at })
            .collect();
        self.cache.insert(account_id.to_string(), cached_secrets);
    }

    pub fn invalidate(&mut self, account_id: &str) {
        debug!(
            "[BUNDLER] Invalidating secrets cache for account_id: {}",
            account_id
        );
        self.cache.remove(account_id);
    }

    pub fn cleanup(&mut self) {
        debug!("[BUNDLER] Starting secrets cache cleanup");
        let now = SystemTime::now();
        self.cache
            .retain(|_, entries| entries.iter().any(|entry| entry.expires_at > now));
    }
}

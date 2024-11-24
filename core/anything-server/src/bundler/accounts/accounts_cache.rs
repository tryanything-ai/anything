use crate::auth::init::AccountAuthProviderAccount;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedAccount {
    account: AccountAuthProviderAccount,
    expires_at: SystemTime,
}

pub struct AccountsCache {
    cache: HashMap<String, Vec<CachedAccount>>, // account_id -> accounts
    ttl: Duration,
}

impl AccountsCache {
    pub fn new(ttl: Duration) -> Self {
        debug!("[BUNDLER] Creating new AccountsCache with TTL: {:?}", ttl);
        Self {
            cache: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, account_id: &str) -> Option<Vec<AccountAuthProviderAccount>> {
        self.cache.get(account_id).and_then(|entries| {
            let now = SystemTime::now();
            if entries.iter().all(|entry| entry.expires_at > now) {
                Some(entries.iter().map(|e| e.account.clone()).collect())
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, account_id: &str, accounts: Vec<AccountAuthProviderAccount>) {
        debug!(
            "[BUNDLER] Setting accounts cache for account_id: {}",
            account_id
        );
        let expires_at = SystemTime::now() + self.ttl;
        let cached_accounts = accounts
            .into_iter()
            .map(|account| CachedAccount {
                account,
                expires_at,
            })
            .collect();
        self.cache.insert(account_id.to_string(), cached_accounts);
    }

    pub fn cleanup(&mut self) {
        debug!("[BUNDLER] Starting accounts cache cleanup");
        let now = SystemTime::now();
        self.cache
            .retain(|_, entries| entries.iter().any(|entry| entry.expires_at > now));
    }
}

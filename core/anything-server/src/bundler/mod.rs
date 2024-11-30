pub mod accounts;
pub mod bundler;
pub mod secrets;

#[cfg(test)]
pub mod bundler_test;

use std::{sync::Arc, time::Duration};

pub use bundler::*;

use crate::AppState;

pub async fn cleanup_bundler_caches(state: Arc<AppState>) {
    let cleanup_interval = Duration::from_secs(86400); // Run cleanup every day
    println!("[BUNDLER] Starting periodic cache cleanup task");
    loop {
        tokio::time::sleep(cleanup_interval).await;
        println!("[BUNDLER] Running scheduled cache cleanup");
        {
            let mut secrets_cache = state.bundler_secrets_cache.write().await;
            secrets_cache.cleanup();
        }
        {
            let mut accounts_cache = state.bundler_accounts_cache.write().await;
            accounts_cache.cleanup();
        }
    }
}

pub mod accounts;
pub mod bundler;
pub mod secrets;

use std::{sync::Arc, time::Duration};

pub use bundler::*;

use crate::AppState;

pub async fn cleanup_bundler_caches(state: Arc<AppState>) {
    let cleanup_interval = Duration::from_secs(86400); // Run cleanup every day
    println!("[BUNDLER] Starting periodic cache cleanup task");
    loop {
        tokio::time::sleep(cleanup_interval).await;
        println!("[BUNDLER] Running scheduled cache cleanup");

        // Cleanup secrets caches for all accounts
        for cache_entry in state.bundler_secrets_cache.iter_mut() {
            cache_entry.cleanup();
        }

        // Cleanup accounts caches for all accounts
        for cache_entry in state.bundler_accounts_cache.iter_mut() {
            cache_entry.cleanup();
        }
    }
}

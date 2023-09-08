use std::fs;

use anything_core::{error::AnythingResult, tracing::setup_tracing, AnythingConfig};
use tracing::info;

pub async fn bootstrap(config: &AnythingConfig) -> AnythingResult<()> {
    info!("Bootstrapping Eventurous");
    // Bootstrap database directory
    let root_dir = config.root_dir.clone();
    let db_dir = root_dir.join("database");

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    setup_tracing(tracing_subscriber::registry(), config);

    Ok(())
}

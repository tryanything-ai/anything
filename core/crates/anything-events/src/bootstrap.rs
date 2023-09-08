use std::fs;

use tracing::info;

use crate::{config::Config, utils::tracing::setup_tracing, EvtResult};

pub async fn bootstrap(config: &Config) -> EvtResult<()> {
    info!("Bootstrapping Eventurous");
    // Bootstrap database directory
    let root_dir = config.root_dir.clone();
    let db_dir = root_dir.join("database");

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    setup_tracing(tracing_subscriber::registry(), "events-service");

    Ok(())
}

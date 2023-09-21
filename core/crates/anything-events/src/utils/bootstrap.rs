use std::fs;

use tracing::info;

use crate::{
    config::AnythingEventsConfig, context::Context, errors::EventsResult,
    models::system_handler::SystemHandler, utils::tracing::setup_tracing,
};

pub async fn bootstrap<'a>(config: &'a AnythingEventsConfig) -> EventsResult<Context> {
    info!("Bootstrapping anything");
    color_eyre::install()?;

    bootstrap_directory(config)?;
    setup_tracing("anything".to_string(), &config);
    setup_system(config).await?;

    // Create context
    let context = Context::new(config.clone()).await?;

    Ok(context)
}

// -----------------------------------------------------------------
// Bootstrap systems
// -----------------------------------------------------------------
async fn setup_system<'a>(_config: &'a AnythingEventsConfig) -> EventsResult<()> {
    SystemHandler::setup(_config).await?;
    Ok(())
}

// -----------------------------------------------------------------
// Bootstrap directory
// -----------------------------------------------------------------
fn bootstrap_directory<'a>(config: &'a AnythingEventsConfig) -> EventsResult<()> {
    // Bootstrap database directory
    let root_dir = config.root_dir.clone();

    let directories = vec![
        "database", "logs", "config", "nodes", "settings", "assets", "flows",
    ];

    directories.into_iter().for_each(|dir| {
        let dir = root_dir.join(dir);
        if !dir.exists() {
            fs::create_dir_all(dir).unwrap();
        }
    });

    // // If the parent directory does not exist, create it.
    // if !db_dir.exists() {
    //     fs::create_dir_all(db_dir).unwrap();
    // }
    Ok(())
}

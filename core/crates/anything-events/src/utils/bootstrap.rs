use std::fs;

use tracing::info;

use crate::{
    config::AnythingEventsConfig, context::Context, errors::EventsResult,
    models::system_handler::SystemHandler, utils::tracing::setup_tracing,
};

pub async fn bootstrap<'a>(config: &'a AnythingEventsConfig) -> EventsResult<Context> {
    setup_tracing("anything".to_string(), &config);
    info!("Bootstrapping anything");
    bootstrap_directory(config)?;
    info!("Root Dir {:?}", config.root_dir);
    // Create context
    let context = Context::new(config.clone()).await?;
    setup_system(context.clone()).await?;

    Ok(context)
}

// -----------------------------------------------------------------
// Bootstrap systems
// -----------------------------------------------------------------
async fn setup_system<'a>(context: Context) -> EventsResult<()> {
    SystemHandler::setup(context).await?;
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
    Ok(())
}

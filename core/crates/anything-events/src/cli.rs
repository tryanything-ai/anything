use std::path::PathBuf;

use anything_core::error::AnythingResult;
use clap::{Parser, Subcommand};
use tracing::{debug, info};

use crate::{
    config::AnythingEventsConfig,
    context::Context,
    errors::{EventsError, EventsResult},
    server::server::Server,
    utils::bootstrap,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Provide a specific config path
    #[arg(short, long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Server {
        #[arg(short, long)]
        database_uri: Option<String>,
    },
}

pub async fn start() -> EventsResult<()> {
    let cli = Cli::parse();

    let config_path = cli.config_path;
    // let mut config = crate::config::load(config_path.as_ref())?;
    // let mut config = anything_core::config::load(config_path.as_ref())?;
    let mut config = crate::config::load(config_path.as_ref())?;

    // logging::setup(&config)?;
    bootstrap::bootstrap(&config).await?;

    match cli.command {
        Some(Commands::Server { database_uri }) => {
            // TODO: add a sexier way to handle this configuration
            if let Some(database_uri) = database_uri {
                config.database.uri = database_uri;
            }
            let ctx = Context::new(config).await?;
            debug!("Context: {:?}", ctx);
            debug!("Building server...");
            let server = Server::new(ctx).await?;
            server.run_server().await?;
        }
        None => {
            println!("No command specified");
        }
    }

    Ok(())
}

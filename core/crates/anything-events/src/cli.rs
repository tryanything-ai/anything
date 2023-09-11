use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::debug;

use crate::{errors::EventsResult, server::server::Server, utils::bootstrap};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Provide a specific config path
    #[arg(short, long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,

    #[arg(short, long)]
    pub database_uri: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Run the server
    Server {},
    /// Post a new event to a running server
    PostEvent {},
}

pub async fn start() -> EventsResult<()> {
    let cli = Cli::parse();

    let config_path = cli.config_path;
    // let mut config = crate::config::load(config_path.as_ref())?;
    // let mut config = anything_core::config::load(config_path.as_ref())?;
    let config = crate::config::load(config_path.as_ref())?;

    // logging::setup(&config)?;
    let context = bootstrap::bootstrap(&config).await?;

    match cli.command {
        Some(Commands::Server {}) => {
            debug!("Building server...");
            let server = Server::new(context).await?;
            server.run_server().await?;
        }
        Some(Commands::PostEvent {}) => {
            println!("post an event");
        }
        None => {
            println!("No command specified");
        }
    }

    Ok(())
}

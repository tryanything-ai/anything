use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{bootstrap, context::Context, server::server::Server, EvtResult};

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

pub async fn start() -> EvtResult<()> {
    let cli = Cli::parse();

    let config_path = cli.config_path;
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
            let server = Server::new(ctx).await?;
            server.run_server().await?;
        }
        None => {
            println!("No command specified");
        }
    }

    Ok(())
}

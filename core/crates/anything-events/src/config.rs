use anyhow::{Context, Result};
use serde::{de, Deserialize, Deserializer};
use std::{fmt::Formatter, path::PathBuf};
struct DurationError(humantime::DurationError);
use config::{builder::DefaultState, ConfigBuilder, Environment, File, FileFormat};

pub const GIT_VERSION: &str = git_version::git_version!();

impl de::Expected for DurationError {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a human duration string ({})", self.0)
    }
}

fn serde_human_time<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<u64, D::Error> {
    let raw: String = Deserialize::deserialize(d)?;
    let secs = humantime::parse_duration(&raw)
        .map_err(|err| de::Error::invalid_value(de::Unexpected::Str(&raw), &DurationError(err)))?
        .as_secs() as u64;
    Ok(secs)
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// The name of the server
    pub host: Option<String>,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub uri: String,

    pub max_connections: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TracingConfig {
    /// The service name given for the tracing
    pub service_name: Option<String>,
    /// The endpoint of the otel collector
    pub otel_endpoint: Option<String>,

    pub json_log: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnythingEventsConfig {
    pub root_dir: PathBuf,
    pub json_log: bool,
    pub log: String,
    // pub task_engine: TaskEngine,
    #[serde(deserialize_with = "serde_human_time")]
    pub log_retention: u64,

    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub tracing: TracingConfig,
}

impl Default for AnythingEventsConfig {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("./.eventurous"),
            json_log: false,
            log: "info".to_string(),
            log_retention: 86400,
            server: ServerConfig {
                host: None,
                port: 8080,
            },
            database: DatabaseConfig {
                uri: "sqlite://:memory:".to_string(),
                max_connections: None,
            },
            tracing: TracingConfig {
                service_name: None,
                otel_endpoint: None,
                json_log: None,
            },
        }
    }
}

pub fn loader(file: Option<&PathBuf>) -> ConfigBuilder<DefaultState> {
    let mut builder = config::Config::builder();

    builder = builder.add_source(File::from_str(
        include_str!("../config/default_config.toml"),
        FileFormat::Toml,
    ));

    if let Some(file) = file {
        let file = file.as_path();
        builder = builder.add_source(File::from(file));
    }

    builder.add_source(
        Environment::with_prefix("EVENTUROUS")
            .list_separator(",")
            .try_parsing(true)
            .with_list_parse_key("cluster_seed_nodes"),
    )
}

pub fn load(file: Option<&PathBuf>) -> Result<AnythingEventsConfig> {
    let config = loader(file)
        .build()?
        .try_deserialize()
        .context("mandatory configuration value not set")?;

    Ok(config)
}


// use opentelemetry_otlp::WithExportConfig;
// use tracing::Subscriber;
// use tracing_subscriber::registry::LookupSpan;
// use tracing_subscriber::EnvFilter;
// use tracing_subscriber::{fmt, prelude::*};

// use tracing::info;

// use crate::{config::AnythingEventsConfig, context::Context, errors::EventsResult};

// pub async fn bootstrap<'a>(config: &'a AnythingEventsConfig) -> EventsResult<Context> {
  
//     info!("Bootstrapping Anything File Structure");
//     // Bootstrap database directory
//     let root_dir = config.root_dir.clone();
//     let db_dir = root_dir.join("database");

//     // If the parent directory does not exist, create it.
//     if !db_dir.exists() {
//         fs::create_dir_all(db_dir).unwrap();
//     }

//     setup_tracing(tracing_subscriber::registry(), &config);

//     // Create context
//     let context = Context::new(config.clone()).await?;

//     Ok(context)
// }
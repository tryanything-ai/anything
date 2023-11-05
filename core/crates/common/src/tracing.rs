const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

pub fn setup_tracing(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::sdk::trace::Config;
    use tracing_subscriber::{prelude::*, EnvFilter};

    if std::env::var_os("RUST_LOG").is_none() {
        let level = std::env::var_os("LOG_LEVEL").unwrap_or_else(|| "info".into());
        let level_str = level.as_os_str().to_str().unwrap_or("info");
        let var = vec![
            format!("{CRATE_NAME}={level_str}"),
            format!("{service_name}={level_str}"),
        ];

        std::env::set_var("RUST_LOG", var.join(","));
    }

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .with_trace_config(Config::default())
        .install_simple()?;
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .with(EnvFilter::from_default_env())
        .try_init()?;

    Ok(())
}

pub use tracing::*;

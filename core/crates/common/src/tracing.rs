const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

pub fn setup_tracing(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::sdk::trace::Config;
    use tracing_subscriber::{prelude::*, EnvFilter};

    if std::env::var_os("RUST_LOG").is_none() {
        let level: String = std::env::var("LOG_LEVEL").unwrap_or("info".into());
        let level_str = level.as_str();
        let var = vec![
            format!("{CRATE_NAME}={level_str}"),
            format!("{service_name}={level_str}"),
        ];

        std::env::set_var("RUST_LOG", var.join(","));
    }

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_level(true)
        .with_target(true);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .with_trace_config(Config::default())
        .install_simple()?;
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    // tracing_subscriber::registry()
    //     .with(telemetry)
    //     .with(EnvFilter::from_default_env())
    //     .try_init()?;

    Ok(())
}

pub use tracing::*;

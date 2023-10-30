pub fn setup_tracing(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::sdk::trace::Config;
    // use opentelemetry::sdk::Resource;
    // use opentelemetry::KeyValue;
    use tracing_subscriber::{prelude::*, EnvFilter};

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .with_trace_config(Config::default())
        .install_simple()?;
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(telemetry)
        .try_init()?;

    Ok(())
}

pub use tracing::*;

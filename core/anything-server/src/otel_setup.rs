use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use opentelemetry_semantic_conventions::resource;
use std::env;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

/// Initializes OpenTelemetry tracing and exports data to a gRPC OTLP endpoint.
///
/// The OTLP endpoint is read from the `OTEL_EXPORTER_OTLP_ENDPOINT` environment variable.
pub fn init_otel_grpc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .map_err(|e| format!("Failed to get OTEL_EXPORTER_OTLP_ENDPOINT: {}", e))?;

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint);

    // Determine the deployment environment. Default to "development" if not set.
    let deployment_environment =
        env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    let resource = Resource::new(vec![
        opentelemetry::KeyValue::new(resource::SERVICE_NAME, "anything-server"),
        opentelemetry::KeyValue::new(resource::DEPLOYMENT_ENVIRONMENT, deployment_environment),
        // You can add more common attributes here, for example:
        // opentelemetry::KeyValue::new(resource::SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        // opentelemetry::KeyValue::new(resource::HOST_NAME, hostname::get().unwrap_or_default().to_string_lossy().into_owned()),
    ]);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            sdktrace::config()
                .with_sampler(sdktrace::Sampler::AlwaysOn)
                .with_resource(resource),
        )
        .install_batch(runtime::Tokio)?;

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("off,anything_server=trace"));

    let subscriber = Registry::default().with(env_filter).with(telemetry_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

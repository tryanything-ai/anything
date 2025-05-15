use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace};
use std::env;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

/// Initializes OpenTelemetry tracing and exports data to a gRPC OTLP endpoint.
///
/// The OTLP endpoint is read from the `OTEL_EXPORTER_OTLP_ENDPOINT` environment variable.
pub fn init_otel_grpc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .map_err(|e| format!("Failed to get OTEL_EXPORTER_OTLP_ENDPOINT: {}", e))?;

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(sdktrace::config().with_sampler(sdktrace::Sampler::AlwaysOn))
        .install_batch(runtime::Tokio)?;

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default().with(telemetry_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

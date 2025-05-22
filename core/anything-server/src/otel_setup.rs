use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use opentelemetry_semantic_conventions::resource;
use std::env;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

// Added for metrics
use opentelemetry::global as otel_global;
use opentelemetry_sdk::metrics as sdkmetrics;

/// Initializes OpenTelemetry tracing and metrics exporting to a gRPC OTLP endpoint.
///
/// The OTLP endpoint is read from the `OTEL_EXPORTER_OTLP_ENDPOINT` environment variable.
pub fn init_otel_grpc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .map_err(|e| format!("Failed to get OTEL_EXPORTER_OTLP_ENDPOINT: {}", e))?;

    // Determine the deployment environment based on build profile.
    let deployment_environment = if cfg!(debug_assertions) {
        "development"
    } else {
        "production"
    };

    let resource = Resource::new(vec![
        opentelemetry::KeyValue::new(resource::SERVICE_NAME, "anything-server"),
        opentelemetry::KeyValue::new(resource::DEPLOYMENT_ENVIRONMENT, deployment_environment),
        // You can add more common attributes here, for example:
        opentelemetry::KeyValue::new(resource::SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        // opentelemetry::KeyValue::new(resource::HOST_NAME, hostname::get().unwrap_or_default().to_string_lossy().into_owned()),
    ]);

    // Setup Tracing
    let trace_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint.clone()); // Clone the endpoint string for the trace exporter
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(trace_exporter)
        .with_trace_config(
            sdktrace::config()
                .with_sampler(sdktrace::Sampler::AlwaysOn)
                .with_resource(resource.clone()), // Clone resource for tracer
        )
        .install_batch(runtime::Tokio)?;

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Setup Metrics
    let metrics_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint); // Use the original endpoint string for the metrics exporter
    let meter_provider = opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio) // Specify the runtime for metrics
        .with_exporter(metrics_exporter)
        .with_resource(resource) // Use the same resource for metrics
        .build()?;

    otel_global::set_meter_provider(meter_provider);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("off,anything_server=trace"));

    let subscriber = Registry::default().with(env_filter).with(telemetry_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

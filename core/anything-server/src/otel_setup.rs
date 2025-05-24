use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use opentelemetry_semantic_conventions::resource;
use std::env;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

// Added for metrics
use opentelemetry::global as otel_global;
use opentelemetry_sdk::metrics as sdkmetrics;

/// Initializes OpenTelemetry tracing and metrics exporting to a gRPC OTLP endpoint.
/// If OTLP endpoint is not available, falls back to console-only logging.
pub fn init_otel_grpc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Try to set up OTLP, but don't fail if it's not available
    match env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        Ok(otlp_endpoint) => {
            println!(
                "Setting up OpenTelemetry with OTLP endpoint: {}",
                otlp_endpoint
            );

            // Determine the deployment environment based on build profile.
            let deployment_environment = if cfg!(debug_assertions) {
                "development"
            } else {
                "production"
            };

            let resource = Resource::new(vec![
                opentelemetry::KeyValue::new(resource::SERVICE_NAME, "anything-server"),
                opentelemetry::KeyValue::new(
                    resource::DEPLOYMENT_ENVIRONMENT,
                    deployment_environment,
                ),
                opentelemetry::KeyValue::new(resource::SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            ]);

            // Setup Tracing
            let trace_exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint.clone());
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(trace_exporter)
                .with_trace_config(
                    sdktrace::config()
                        .with_sampler(sdktrace::Sampler::AlwaysOn)
                        .with_resource(resource.clone()),
                )
                .install_batch(runtime::Tokio)?;

            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

            // Setup Metrics
            let metrics_exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint);
            let meter_provider = opentelemetry_otlp::new_pipeline()
                .metrics(runtime::Tokio)
                .with_exporter(metrics_exporter)
                .with_resource(resource)
                .build()?;

            otel_global::set_meter_provider(meter_provider);

            // Initialize the metrics registry
            let _ = &crate::metrics::METRICS;

            // Add console/fmt layer for local logging
            let fmt_layer = fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_line_number(true);

            // Set up subscriber with both OTLP and console
            let subscriber = Registry::default()
                .with(env_filter)
                .with(telemetry_layer) // Exports to OTLP
                .with(fmt_layer); // Prints to console

            tracing::subscriber::set_global_default(subscriber)?;
            println!("✅ OpenTelemetry initialized successfully with OTLP export");
        }
        Err(_) => {
            println!("⚠️  OTEL_EXPORTER_OTLP_ENDPOINT not found, using console-only logging");

            // Set up console-only logging with fresh layer
            let console_fmt_layer = fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_line_number(true);

            let subscriber = Registry::default().with(env_filter).with(console_fmt_layer);

            tracing::subscriber::set_global_default(subscriber)?;
            println!("✅ Console logging initialized successfully");
        }
    }

    Ok(())
}

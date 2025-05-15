use tracing::{error, info, span, warn, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};

use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    runtime::Tokio,
    trace::{
        self as sdktrace, Config as TraceConfig, RandomIdGenerator, Sampler, SdkTracerProvider,
    },
    Resource,
};
use opentelemetry_semantic_conventions::{
    resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};

// Helper to build a Resource for the service, adapted from the example
fn otel_resource() -> Resource {
    println!("[OTEL_SETUP DEBUG] Creating Otel Resource...");
    let service_name = env!("CARGO_PKG_NAME").to_string();
    let service_version = env!("CARGO_PKG_VERSION").to_string();
    let deployment_env =
        std::env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    println!("[OTEL_SETUP DEBUG]   Service Name: {}", service_name);
    println!("[OTEL_SETUP DEBUG]   Service Version: {}", service_version);
    println!(
        "[OTEL_SETUP DEBUG]   Deployment Environment: {}",
        deployment_env
    );

    Resource::builder()
        .with_attributes([
            KeyValue::new(SERVICE_NAME, service_name),
            KeyValue::new(SERVICE_VERSION, service_version),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, deployment_env),
        ])
        .build()
}

// Set up the OpenTelemetry tracer provider, adapted from the example
fn init_tracer_provider() -> Result<SdkTracerProvider, opentelemetry::trace::TraceError> {
    println!("[OTEL_SETUP DEBUG] Initializing Tracer Provider...");
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://otel-collector.railway.internal:4317".to_string());
    println!(
        "[OTEL_SETUP DEBUG]   OTEL Exporter Endpoint: {}",
        otel_endpoint
    );

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otel_endpoint)
        .with_timeout(std::time::Duration::from_secs(3));

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(TraceConfig::default().with_resource(otel_resource()))
        .install_batch(Tokio)?;

    println!("[OTEL_SETUP DEBUG]   SdkTracerProvider created.");
    Ok(tracer)
}

// Initialize tracing-subscriber with OpenTelemetry
pub fn init_tracing() {
    println!("[OTEL_SETUP DEBUG] Initializing Tracing...");
    let tracer_provider =
        init_tracer_provider().expect("Failed to initialize OpenTelemetry tracer provider");
    let tracer = opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "anything-server");

    // Set the SdkTracerProvider as the global tracer provider
    global::set_tracer_provider(tracer_provider);
    println!("[OTEL_SETUP DEBUG]   Global TracerProvider set.");

    Registry::default()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
    println!("[OTEL_SETUP DEBUG] Tracing initialized and set as global default.");
}

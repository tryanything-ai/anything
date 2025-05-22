use once_cell::sync::Lazy;
use opentelemetry::{
    global as otel_global,
    metrics::{Counter, Histogram, UpDownCounter},
    KeyValue,
};

/// A registry containing all metrics for the application.
/// This centralizes metric definitions and reduces boilerplate.
pub struct MetricsRegistry {
    meter: opentelemetry::metrics::Meter,

    // Processor metrics
    pub processor_messages_received: Counter<u64>,
    pub processor_active_workflows: UpDownCounter<i64>,
    pub processor_workflow_duration: Histogram<f64>,
}

/// Global static instance of the metrics registry
pub static METRICS: Lazy<MetricsRegistry> = Lazy::new(|| MetricsRegistry::new());

impl MetricsRegistry {
    fn new() -> Self {
        let meter = otel_global::meter("anything_server");

        Self {
            // Processor metrics
            processor_messages_received: meter
                .u64_counter("anything_processor_messages_received_total")
                .with_description("Total number of messages received by the processor.")
                .init(),

            processor_active_workflows: meter
                .i64_up_down_counter("anything_processor_active_workflows")
                .with_description("Number of workflows currently being processed.")
                .init(),

            processor_workflow_duration: meter
                .f64_histogram("anything_workflow_processing_duration_seconds")
                .with_description("Duration of workflow processing in seconds.")
                .init(),

            meter,
        }
    }
}

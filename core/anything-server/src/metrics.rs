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

    // Trigger engine metrics
    pub trigger_hydration_duration: Histogram<f64>,
    pub triggers_loaded_total: Counter<u64>,
    pub triggers_active: UpDownCounter<i64>,
    pub trigger_executions_total: Counter<u64>,
    pub trigger_execution_duration: Histogram<f64>,
    pub trigger_failures_total: Counter<u64>,
    pub trigger_updates_total: Counter<u64>,
    pub trigger_update_duration: Histogram<f64>,
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

            // Trigger engine metrics
            trigger_hydration_duration: meter
                .f64_histogram("anything_trigger_hydration_duration_seconds")
                .with_description("Duration of trigger hydration from database in seconds.")
                .init(),

            triggers_loaded_total: meter
                .u64_counter("anything_triggers_loaded_total")
                .with_description("Total number of triggers loaded from database.")
                .init(),

            triggers_active: meter
                .i64_up_down_counter("anything_triggers_active")
                .with_description("Number of triggers currently active in memory.")
                .init(),

            trigger_executions_total: meter
                .u64_counter("anything_trigger_executions_total")
                .with_description("Total number of trigger executions.")
                .init(),

            trigger_execution_duration: meter
                .f64_histogram("anything_trigger_execution_duration_seconds")
                .with_description("Duration of trigger execution in seconds.")
                .init(),

            trigger_failures_total: meter
                .u64_counter("anything_trigger_failures_total")
                .with_description("Total number of trigger execution failures.")
                .init(),

            trigger_updates_total: meter
                .u64_counter("anything_trigger_updates_total")
                .with_description("Total number of trigger updates.")
                .init(),

            trigger_update_duration: meter
                .f64_histogram("anything_trigger_update_duration_seconds")
                .with_description("Duration of trigger updates in seconds.")
                .init(),

            meter,
        }
    }
}

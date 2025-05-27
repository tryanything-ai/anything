use once_cell::sync::Lazy;
use opentelemetry::{
    global as otel_global,
    metrics::{Counter, Histogram, UpDownCounter},
    KeyValue,
};
use std::time::Duration;

/// A registry containing all metrics for the application.
/// This centralizes metric definitions and reduces boilerplate.
pub struct MetricsRegistry {
    meter: opentelemetry::metrics::Meter,

    // Processor metrics
    pub processor_messages_received: Counter<u64>,
    pub processor_active_workflows: UpDownCounter<i64>,
    pub processor_workflow_duration: Histogram<f64>,
    pub processor_semaphore_wait_time: Histogram<f64>,
    pub processor_workflow_errors: Counter<u64>,

    // Status updater metrics (adding the ones referenced in enhanced_processor.rs)
    pub status_updater_operations_total: Counter<u64>,
    pub status_updater_operations_success: Counter<u64>,
    pub status_updater_operations_failed: Counter<u64>,
    pub status_updater_operation_duration: Histogram<f64>,
    pub status_updater_retries_total: Counter<u64>,
    pub status_updater_max_retries_exceeded: Counter<u64>,
    pub status_updater_database_errors: Counter<u64>,
    pub status_updater_timeout_errors: Counter<u64>,
    pub status_updater_channel_closed_errors: Counter<u64>,
    pub status_updater_queue_wait_time: Histogram<f64>,
    pub status_updater_connection_pool_errors: Counter<u64>,
    pub status_updater_network_errors: Counter<u64>,
    pub status_updater_serialization_errors: Counter<u64>,
    pub status_updater_constraint_errors: Counter<u64>,
    pub status_updater_unknown_errors: Counter<u64>,

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

            processor_semaphore_wait_time: meter
                .f64_histogram("anything_processor_semaphore_wait_time_seconds")
                .with_description("Time spent waiting for semaphore permits in seconds.")
                .init(),

            processor_workflow_errors: meter
                .u64_counter("anything_processor_workflow_errors_total")
                .with_description("Total number of workflow processing errors.")
                .init(),

            // Status updater metrics
            status_updater_operations_total: meter
                .u64_counter("anything_status_updater_operations_total")
                .with_description("Total number of status update operations.")
                .init(),

            status_updater_operations_success: meter
                .u64_counter("anything_status_updater_operations_success_total")
                .with_description("Total number of successful status update operations.")
                .init(),

            status_updater_operations_failed: meter
                .u64_counter("anything_status_updater_operations_failed_total")
                .with_description("Total number of failed status update operations.")
                .init(),

            status_updater_operation_duration: meter
                .f64_histogram("anything_status_updater_operation_duration_seconds")
                .with_description("Duration of status update operations in seconds.")
                .init(),

            status_updater_retries_total: meter
                .u64_counter("anything_status_updater_retries_total")
                .with_description("Total number of status update operation retries.")
                .init(),

            status_updater_max_retries_exceeded: meter
                .u64_counter("anything_status_updater_max_retries_exceeded_total")
                .with_description("Total number of operations that exceeded max retries.")
                .init(),

            status_updater_database_errors: meter
                .u64_counter("anything_status_updater_database_errors_total")
                .with_description("Total number of database errors in status updater.")
                .init(),

            status_updater_timeout_errors: meter
                .u64_counter("anything_status_updater_timeout_errors_total")
                .with_description("Total number of timeout errors in status updater.")
                .init(),

            status_updater_channel_closed_errors: meter
                .u64_counter("anything_status_updater_channel_closed_errors_total")
                .with_description("Total number of channel closed errors in status updater.")
                .init(),

            status_updater_queue_wait_time: meter
                .f64_histogram("anything_status_updater_queue_wait_time_seconds")
                .with_description("Time spent waiting in status update queue in seconds.")
                .init(),

            status_updater_connection_pool_errors: meter
                .u64_counter("anything_status_updater_connection_pool_errors_total")
                .with_description("Total number of connection pool errors in status updater.")
                .init(),

            status_updater_network_errors: meter
                .u64_counter("anything_status_updater_network_errors_total")
                .with_description("Total number of network errors in status updater.")
                .init(),

            status_updater_serialization_errors: meter
                .u64_counter("anything_status_updater_serialization_errors_total")
                .with_description("Total number of serialization errors in status updater.")
                .init(),

            status_updater_constraint_errors: meter
                .u64_counter("anything_status_updater_constraint_errors_total")
                .with_description("Total number of database constraint errors in status updater.")
                .init(),

            status_updater_unknown_errors: meter
                .u64_counter("anything_status_updater_unknown_errors_total")
                .with_description("Total number of unknown errors in status updater.")
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

    // ===== PROCESSOR METRICS METHODS =====

    pub fn record_message_received(&self, labels: &[KeyValue]) {
        self.processor_messages_received.add(1, labels);
    }

    pub fn record_workflow_started(&self, labels: &[KeyValue]) {
        self.processor_active_workflows.add(1, labels);
    }

    pub fn record_workflow_completed(&self, duration: Duration, labels: &[KeyValue]) {
        self.processor_workflow_duration
            .record(duration.as_secs_f64(), labels);
        self.processor_active_workflows.add(-1, labels);
    }

    pub fn record_workflow_error(&self, error_type: &str) {
        self.processor_workflow_errors
            .add(1, &[KeyValue::new("error_type", error_type.to_string())]);
    }

    pub fn record_semaphore_wait_time(&self, wait_duration: Duration, labels: &[KeyValue]) {
        self.processor_semaphore_wait_time
            .record(wait_duration.as_secs_f64(), labels);
    }

    // ===== STATUS UPDATER METRICS METHODS =====

    pub fn record_status_operation_start(&self, operation_type: &str) {
        self.status_updater_operations_total.add(
            1,
            &[KeyValue::new("operation_type", operation_type.to_string())],
        );
    }

    pub fn record_status_operation_success(&self, duration_ms: u64, operation_type: &str) {
        self.status_updater_operations_success.add(
            1,
            &[KeyValue::new("operation_type", operation_type.to_string())],
        );

        self.status_updater_operation_duration.record(
            duration_ms as f64 / 1000.0, // Convert to seconds
            &[KeyValue::new("operation_type", operation_type.to_string())],
        );
    }

    pub fn record_status_operation_failure(&self, error: &str, operation_type: &str) {
        let error_category = self.categorize_error(error);

        self.status_updater_operations_failed.add(
            1,
            &[
                KeyValue::new("operation_type", operation_type.to_string()),
                KeyValue::new("error_type", error_category.to_string()),
            ],
        );

        // Record specific error type counters
        let error_lower = error.to_lowercase();
        if error_lower.contains("connection") || error_lower.contains("pool") {
            self.status_updater_connection_pool_errors.add(1, &[]);
        } else if error_lower.contains("network")
            || error_lower.contains("tcp")
            || error_lower.contains("io")
        {
            self.status_updater_network_errors.add(1, &[]);
        } else if error_lower.contains("serialize")
            || error_lower.contains("json")
            || error_lower.contains("parse")
        {
            self.status_updater_serialization_errors.add(1, &[]);
        } else if error_lower.contains("constraint")
            || error_lower.contains("unique")
            || error_lower.contains("foreign")
        {
            self.status_updater_constraint_errors.add(1, &[]);
        } else {
            self.status_updater_unknown_errors.add(1, &[]);
        }
    }

    pub fn record_status_retry(&self, operation_type: &str) {
        self.status_updater_retries_total.add(
            1,
            &[KeyValue::new("operation_type", operation_type.to_string())],
        );
    }

    pub fn record_status_max_retries_exceeded(&self, operation_type: &str) {
        self.status_updater_max_retries_exceeded.add(
            1,
            &[KeyValue::new("operation_type", operation_type.to_string())],
        );
    }

    pub fn record_status_database_error(&self) {
        self.status_updater_database_errors.add(1, &[]);
    }

    pub fn record_status_timeout_error(&self) {
        self.status_updater_timeout_errors.add(1, &[]);
    }

    pub fn record_status_channel_closed(&self) {
        self.status_updater_channel_closed_errors.add(1, &[]);
    }

    pub fn record_status_queue_wait_time(&self, wait_time_ms: u64) {
        self.status_updater_queue_wait_time.record(
            wait_time_ms as f64 / 1000.0, // Convert to seconds
            &[],
        );
    }

    fn categorize_error(&self, error: &str) -> &'static str {
        let error_lower = error.to_lowercase();

        if error_lower.contains("connection") || error_lower.contains("pool") {
            "connection_pool"
        } else if error_lower.contains("network")
            || error_lower.contains("tcp")
            || error_lower.contains("io")
        {
            "network"
        } else if error_lower.contains("serialize")
            || error_lower.contains("json")
            || error_lower.contains("parse")
        {
            "serialization"
        } else if error_lower.contains("constraint")
            || error_lower.contains("unique")
            || error_lower.contains("foreign")
        {
            "constraint"
        } else {
            "unknown"
        }
    }
}

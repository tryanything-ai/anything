use once_cell::sync::Lazy;
use opentelemetry::{
    global as otel_global,
    metrics::{Counter, Histogram, ObservableGauge, Observer, UpDownCounter},
    KeyValue,
};
use std::sync::Arc;

/// A registry containing all metrics for the application.
/// This centralizes metric definitions and reduces boilerplate.
pub struct MetricsRegistry {
    meter: opentelemetry::metrics::Meter,

    // Processor metrics
    pub processor_messages_received: Counter<u64>,
    pub processor_active_workflows: UpDownCounter<i64>,
    pub processor_workflow_duration: Histogram<f64>,
    pub processor_semaphore_permits_available: ObservableGauge<u64>,
    pub processor_semaphore_permits_used: ObservableGauge<u64>,
    // Add other metrics categories here as needed
}

/// Global static instance of the metrics registry
pub static METRICS: Lazy<MetricsRegistry> = Lazy::new(|| MetricsRegistry::new());

impl MetricsRegistry {
    fn new() -> Self {
        let meter = otel_global::meter("anything_server");

        Self {
            // Processor metrics
            processor_messages_received: meter
                .u64_counter("processor_messages_received_total")
                .with_description("Total number of messages received by the processor.")
                .init(),

            processor_active_workflows: meter
                .i64_up_down_counter("processor_active_workflows")
                .with_description("Number of workflows currently being processed.")
                .init(),

            processor_workflow_duration: meter
                .f64_histogram("workflow_processing_duration_seconds")
                .with_description("Duration of workflow processing in seconds.")
                .init(),

            processor_semaphore_permits_available: meter
                .u64_observable_gauge("semaphore_permits_available")
                .with_description("Number of available semaphore permits for workflow processing.")
                .init(),

            processor_semaphore_permits_used: meter
                .u64_observable_gauge("semaphore_permits_used")
                .with_description("Number of used semaphore permits for workflow processing.")
                .init(),

            meter,
        }
    }

    // Registers a callback function to update semaphore metrics based on the provided app state
    // pub fn register_semaphore_metrics<F>(
    //     &self,
    //     get_total_permits: F,
    //     app_state: Arc<impl HasSemaphore>,
    // ) where
    //     F: Fn(&Arc<impl HasSemaphore>) -> u64 + Send + Sync + 'static,
    // {
    //     let state_clone = app_state.clone();

    //     if let Err(err) = self.meter.register_callback(
    //         &[
    //             self.processor_semaphore_permits_available.as_any(),
    //             self.processor_semaphore_permits_used.as_any(),
    //         ],
    //         move |observer: &dyn Observer| {
    //             let permits_available = state_clone.get_semaphore().available_permits() as u64;
    //             observer.observe_u64(
    //                 &*METRICS.processor_semaphore_permits_available,
    //                 permits_available,
    //                 &[],
    //             );

    //             let total_permits = get_total_permits(&state_clone);
    //             let permits_used = total_permits.saturating_sub(permits_available);
    //             observer.observe_u64(
    //                 &*METRICS.processor_semaphore_permits_used,
    //                 permits_used,
    //                 &[],
    //             );
    //         },
    //     ) {
    //         tracing::error!("Failed to register metrics callback: {}", err);
    //     }
    // }
}

// Trait to abstract access to a semaphore
// pub trait HasSemaphore {
//     fn get_semaphore(&self) -> &tokio::sync::Semaphore;
// }

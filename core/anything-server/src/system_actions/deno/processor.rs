use std::thread;
use tokio::sync::mpsc::{Receiver, Sender};
use serde_json::Value;

use super::process_deno_js_task;

pub struct DenoTask {
    pub code: String,
    pub context: Value,
    pub response_channel: Sender<Result<Option<Value>, String>>,
}

pub fn run_deno_processor(mut rx: Receiver<DenoTask>) {
    // Spawn a dedicated OS thread for Deno
    thread::spawn(move || {
        // Create a new tokio runtime for this thread
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async move {
            while let Some(task) = rx.recv().await {
                let result = process_deno_js_task(&task.context)
                    .await
                    .map_err(|e| e.to_string());
                
                let _ = task.response_channel.send(result).await;
            }
        });
    });
}
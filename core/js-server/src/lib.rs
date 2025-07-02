pub mod javascript_engine;
pub use javascript_engine::JavaScriptEngine;

// Re-export for tests
pub mod js_executor {
    tonic::include_proto!("js_executor");
}

pub use js_executor::*;

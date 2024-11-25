use serde_json::Value;

pub fn process_trigger_task(
    _bundled_context: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS TRIGGER TASK] Processing trigger task");

    Ok(serde_json::json!({
        "message": "successfully triggered",
        "time": chrono::Utc::now().to_rfc3339()
    }))
}

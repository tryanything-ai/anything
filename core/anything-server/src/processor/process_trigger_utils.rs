use serde_json::Value;

pub fn process_trigger_task(
    _bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS TRIGGER TASK] Processing trigger task");

    //Don't return anything for trigger tasks.
    Ok(None)
}

use serde_json::Value;

//This is meant to be used for function calls if we do agents and voice call type thing
//And to be how we do reusable flows or sublfows

pub fn process_input_task(
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[INPUT] Starting input task processing");
    println!("[INPUT] Bundled context: {:?}", bundled_context);

    return Ok(Some(bundled_context.clone()));
}

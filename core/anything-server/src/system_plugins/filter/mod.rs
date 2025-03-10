use serde_json::{json, Value};

//This is meant to be used for function calls if we do agents and voice call type thing
//And to be how we do reusable flows or sublfows
//TODO: maybe just make this expect JS, and we just let it always be JS for determining truth
pub fn process_filter_task(
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    // println!("[CONDITION] Starting condition task processing");
    // println!("[CONDITION] Bundled context: {:?}", bundled_context);

    // return Ok(Some(bundled_context.clone()));
    // Extract condition from the config
    //TODO: we will just do this with JS I think to start
    let condition = bundled_context
        .get("condition")
        .and_then(|c| c.as_bool())
        .unwrap_or(false);

    // Return the condition as a result
    Ok(Some(json!({
        "condition": condition,
        "should_continue": condition
    })))
}

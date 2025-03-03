use crate::AppState;
use serde_json::Value;
use std::sync::Arc;

pub async fn process_outbound_call_task(
    state: Arc<AppState>,
    bundled_context: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[OUTBOUND_CALL] Bundled context: {:?}", bundled_context);

    // Extract phone number from bundled context
    let destination_phone_number = bundled_context
        .get("destination_phone_number")
        .and_then(Value::as_str)
        .ok_or("destination_phone_number is required in bundled context")?;

    let agent_id = bundled_context
        .get("agent_id")
        .and_then(Value::as_str)
        .ok_or("agent_id is required in bundled context")?;

    let account_phone_number_id = bundled_context
        .get("account_phone_number_id")
        .and_then(Value::as_str)
        .ok_or("account_phone_number_id is required in bundled context")?;

    // Call the start_outbound_call function with the phone number
    match crate::agents::vapi::start_outbound_call(
        state,
        agent_id.to_string(),
        account_phone_number_id.to_string(),
        destination_phone_number.to_string(),
    )
    .await
    {
        Ok(response) => {
            println!(
                "[OUTBOUND_CALL] Outbound call initiated successfully: {:?}",
                response
            );
            Ok(Some(response))
        }
        Err(e) => {
            println!("[OUTBOUND_CALL] Error initiating outbound call: {:?}", e);
            Err(format!("Failed to initiate outbound call: {}", e).into())
        }
    }
}

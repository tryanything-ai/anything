use anything_pdk::{Action, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[host_fn]
extern "ExtismHost" {
    fn create_event(event: String) -> Json<Event>;
}

#[plugin_fn]
pub fn execute(config: Value) -> FnResult<Value> {
    //TODO: Determine what events need to be created. Create Them
    //TODO: host function for creating a real event not just stubbed string
    let message = "Creating an event".to_string();
    let _res = unsafe { create_event(message.clone())? };
    Ok(config)
}

#[plugin_fn]
pub fn register() -> FnResult<Action> {
    //Used to let UI and users know how to configure actions
    let action: Action = Action::builder()
        .trigger(true)
        .action_name("example_cron_trigger".to_string())
        .action_label("Example Cron Trigger".to_string())
        .icon("<svg></svg>".to_string())
        .description("Example Of A Cron Trigger".to_string())
        .variables(vec![])
        .input(serde_json::json!({
            "pattern": "0 */5 * * * *", //Every 5 minutes
        }))
        .input_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                },
            },
            "required": ["pattern"],
            "additionalProperties": false
        }))
        .output_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string"
                    "enum": ["success", "error"]
                },
                "output": {
                    "type": "object"
                },
                "error": {
                    "type": "object"
                }
            },
            "required": ["status"],
            "additionalProperties": false
        }))
        .plugin_id("example_cron_extension".to_string())
        .build();

    Ok(action)
}

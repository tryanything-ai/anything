use anything_pdk::{AnythingPlugin, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[host_fn]
extern "ExtismHost" {
    fn create_event(event: Json<Event>) -> String;
}

#[plugin_fn]
pub fn execute(config: Value) -> FnResult<Value> {
    //TODO: Determine what events need to be created. Create Them
    //TODO: host function for creating a real event not just stubbed string
    let event = Event {
        id: "1".to_string(),
        name: "Test Event".to_string(),
        description: "This is a test event".to_string(),
        timestamp: "2021-01-01T00:00:00Z".to_string(),
    };

    //Call Create Event
    let _res = unsafe { create_event(Json(event))? };

    let res = serde_json::json!({
        "status": "success",
        "output": {},
        "error": {},
    });

    Ok(res)
}

#[plugin_fn]
pub fn register() -> FnResult<AnythingPlugin> {
    //Used to let UI and users know how to configure actions
    let plugin: AnythingPlugin = AnythingPlugin::builder()
        .trigger(true)
        .label("Example Cron Trigger".to_string())
        .icon("<svg></svg>".to_string())
        .description("Example Of A Cron Trigger".to_string())
        .variables(vec![])
        .input(serde_json::json!({
            "cron_expression": "0 */5 * * * *", //Every 5 minutes
        }))
        .input_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "cron_expression": {
                    "type": "string", //Extensiosn of type trigger are required to have a cron_expression property
                },
            },
            "required": ["cron_expression"],
            "additionalProperties": false
        }))
        .output_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
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
        .plugin_id("example_cron_plugin".to_string())
        .build();

    Ok(plugin)
}

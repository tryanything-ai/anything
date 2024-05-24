use anything_pdk::{AnythingPlugin, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[host_fn]
extern "ExtismHost" {
    fn host_log(log: String) -> Json<Log>;
    fn create_event(event: String) -> Json<Event>;
}

#[plugin_fn]
pub fn execute(config: Value) -> FnResult<Value> {
    // let _res = unsafe { host_log(message.clone())? };
    Ok(config)
}

#[plugin_fn]
pub fn register() -> FnResult<AnythingPlugin> {
    //Used to let UI and users know how to configure
    let plugin: AnythingPlugin = AnythingPlugin::builder()
        .trigger(false)
        .label("Example Plugin".to_string())
        .icon("<svg></svg>".to_string())
        .description("This is an example plugin".to_string())
        .variables(vec![])
        .input(serde_json::json!({
            "method": "GET",
            "url": "http://example.com",
            "headers": {},
            "body": ""
        }))
        .input_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"]
                },
                "url": {
                    "type": "string"
                },
                "headers": {
                    "type": "object"
                },
                "body": {
                    "type": "string"
                }
            },
            "required": ["method", "url"],
            "additionalProperties": false
        }))
        .plugin_id("example_plugin".to_string())
        .build();

    Ok(plugin)
}

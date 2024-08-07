use anything_pdk::{AnythingPlugin, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[host_fn]
extern "ExtismHost" {
    fn host_log(log: String) -> Json<Log>;
}

#[plugin_fn]
pub fn execute(config: Value) -> FnResult<Value> {
    Ok(config)
}

#[plugin_fn]
pub fn register() -> FnResult<AnythingPlugin> {
    //Used to let UI and users know how to configure
    let plugin: AnythingPlugin = AnythingPlugin::builder()
        .trigger(false)
        .plugin_id("example_plugin".to_string())
        .label("Example Plugin".to_string())
        .description("This is an example plugin".to_string())
        .icon("<svg></svg>".to_string())
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
        .build();

    Ok(plugin)
}

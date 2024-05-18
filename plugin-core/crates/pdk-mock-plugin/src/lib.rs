use anything_pdk::{Action, Event, Handle, Log};
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
pub fn register() -> FnResult<Action> {
    //Used to let UI and users know how to configure actions
    let action: Action = Action::builder()
        .trigger(false)
        .node_name("example_node".to_string())
        .node_label("Example Node".to_string())
        .icon("<svg></svg>".to_string())
        .description("This is an example action".to_string())
        .variables(vec![])
        .config(serde_json::json!({
            "method": "GET",
            "url": "http://example.com",
            "headers": {},
            "body": ""
        }))
        .extension_id("example_extension".to_string())
        .build();

    Ok(action)
}

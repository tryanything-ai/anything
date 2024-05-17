use anything_pdk::{Action, Log, Event};
use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[host_fn]
extern "ExtismHost" {
    fn host_log(log: String) -> Json<Log>;
    fn create_event(event: String) -> Json<Event>;
}

#[plugin_fn]
pub fn execute(message: String) -> FnResult<String> {
    let _res = unsafe { host_log(message.clone())? };
    Ok(message)
}

#[plugin_fn]
pub fn register() -> FnResult<Action> {
    let action = Action {
        id: "1".to_string(),
        name: "register".to_string(),
        description: "register a new user".to_string(),
        timestamp: "2021-09-01".to_string(),
    };

    Ok(action)
}

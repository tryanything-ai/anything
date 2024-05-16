use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Log {
    pub time: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: String,
}

#[derive(serde::Serialize, ToBytes)]
#[encoding(Json)]
struct Action {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: String,
}

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

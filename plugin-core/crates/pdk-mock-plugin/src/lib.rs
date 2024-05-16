use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Log {
    pub time: String,
    pub message: String,
}

#[host_fn]
extern "ExtismHost" {
    fn host_log(log: String) -> Json<Log>;
}

#[plugin_fn]
pub fn execute(message: String) -> FnResult<String> {
    let _res = unsafe { host_log(message.clone())? };
    Ok(message)
}

use anything_pdk::{Action, Event, Handle, Log};
use extism_pdk::*;
use serde::Deserialize;

#[derive(serde::Serialize, ToBytes)]
#[encoding(Json)]
struct Action {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: String,
}

#[plugin_fn]
pub fn execute(message: String) -> FnResult<String> {
    // let _res = unsafe { host_log(message.clone())? };
    Ok(message)
}
// #[plugin_fn]
// pub fn execute(Json(req): Json<HttpRequest>) -> FnResult<Vec<u8>> {
//     let res = http::request::<()>(&req, None)?;
//     Ok(res.body())
// }

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

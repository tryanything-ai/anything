use anything_pdk::*;
use extism_pdk::*;
use serde::Deserialize;

#[host_fn]
extern "ExtismHost" {
    fn host_log(input: Json<HostLogInput>);
}

#[plugin_fn]
pub fn execute(Json(req): Json<HttpRequest>) -> FnResult<Vec<u8>> {
    let res = http::request::<()>(&req, None)?;
    Ok(res.body())
}

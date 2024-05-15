use extism_pdk::*;

#[plugin_fn]
pub fn execute(message: String) -> FnResult<String> {
    Ok(message)
}

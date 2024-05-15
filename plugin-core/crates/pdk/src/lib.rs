use extism::{CurrentPlugin, Error, Function, UserData, Val, ValType};
use serde_json::Value;

//How Proto calls this https://github.com/moonrepo/proto/blob/d423b236948211aa644c4a8389f1f92343936260/crates/core/src/tool.rs
pub fn create_host_functions() -> Vec<Function> {
    vec![Function::new(
        "host_log",
        [ValType::I64],
        [],
        UserData::new(()),
        host_log,
    )]
}

fn host_log(
    plugin: &mut CurrentPlugin,
    inputs: &[Val],
    _outputs: &mut [Val],
    _user_data: UserData<()>,
) -> Result<(), Error> {
    let input: Value = serde_json::from_str(plugin.memory_get_val(&inputs[0])?)?;
    let message = input["message"].as_str().unwrap_or("No message provided");

    println!("Host Function Logged: {}", message);

    Ok(())
}
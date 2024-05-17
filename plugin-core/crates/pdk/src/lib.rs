// use extism::{CurrentPlugin, Error, Function, Json, UserData, Val, ValType};
// use serde_json::Value;

use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
//export a type used for register function
// #[derive(serde::Serialize, ToBytes)]
// #[encoding(Json)]
// pub struct Action {
//     pub id: String,
//     pub name: String,
//     pub description: String,
//     pub timestamp: String,
// }

// Define the Handle struct
#[derive(Serialize, Deserialize, ToBytes)]
#[encoding(Json)]
pub struct Handle {
    pub id: String,
    pub position: String,
    pub r#type: String,
}

// Define the main Action struct
#[derive(Serialize, Deserialize, ToBytes)]
#[encoding(Json)]
pub struct Action {
    pub trigger: bool,
    pub node_name: String,
    pub node_label: String,
    pub icon: String,
    pub description: String,
    pub handles: Vec<Handle>,
    pub variables: Vec<Value>, // Assuming variables is a list of arbitrary JSON values. Adjust if needed.
    pub config: Value,         // Config as arbitrary JSON
    pub extension_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub time: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: String,
}

//How Proto calls this https://github.com/moonrepo/proto/blob/d423b236948211aa644c4a8389f1f92343936260/crates/core/src/tool.rs
// pub fn create_host_functions() -> Vec<Function> {
//     vec![Function::new(
//         "host_log",
//         [ValType::I64],
//         [],
//         UserData::new(()),
//         host_log,
//     )]
// }

// fn host_log(
//     plugin: &mut CurrentPlugin,
//     inputs: &[Val],
//     _outputs: &mut [Val],
//     _user_data: UserData<()>,
// ) -> Result<(), Error> {
//     let input: Value = serde_json::from_str(plugin.memory_get_val(&inputs[0])?)?;
//     let message = input["message"].as_str().unwrap_or("No message provided");

//     println!("Host Function Logged: {}", message);

//     Ok(())
// }

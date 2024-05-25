use anything_pdk::{AnythingPlugin, Event}; 
use extism::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use crate::convert::Json; 

// Define the Event struct
// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Event {
//     pub id: String,
//     pub name: String,
//     pub description: String,
//     pub timestamp: String,
// }

// Define a simple counter to track the number of events
type EventCounter = usize; 
// Implement the create_event host function
host_fn!(create_event(user_data: EventCounter; event: Json<Event>) -> String {
    // let counter = user_data.get()?;
    // let mut counter = counter.lock().unwrap();
    // *counter += 1;
    Ok("Success".to_string())
});

// #[no_mangle]
// pub extern "C" fn create_event(event_ptr: i64) -> i64 {
//     // Find the memory at the given pointer
//     let event_mem = Memory::find(event_ptr as u64).expect("can't find event memory");

//     // Convert memory to a string
//     let event_data = event_mem.to_string().expect("bad data?");

//     // Deserialize the event data to the Event struct
//     let event: Event = serde_json::from_str(&event_data).expect("deserialization failed");

//     // Here you can perform any operations you need with the event
//     // For example, log the event details or store them somewhere
//     // println!("Received event: {:?}", event);

//     // Serialize the event back to JSON
//     let output = serde_json::to_vec(&event).expect("serialization failed");

//     // Create new memory for the output
//     let output_mem = Memory::from_bytes(&output);

//     // Return the offset of the new memory as an i64
//     return output_mem.expect("cant return offset").offset() as i64;
// }

fn main() {
    println!("Run `cargo test` to execute the tests.");
}

// When a first argument separated with a semicolon is provided to `host_fn` it is used as the
// variable name and type for the `UserData` parameter
// host_fn!(pub hello_world (a: String) -> String { Ok(a) });
//TODO: add create_event host function
// Define a simple KV store to hold the events
// type EventStore = Arc<Mutex<BTreeMap<String, Event>>>;

// // Implement the create_event host function
// host_fn!(create_event(user_data: EventStore; id: String, name: String, description: String, timestamp: String) {
//     let mut store = user_data.lock().unwrap();
//     let event = Event {
//         id: id.clone(),
//         name,
//         description,
//         timestamp,
//     };
//     store.insert(id, event);
//     Ok(())
// });

//Generally this is ran via the Included Justfile.
//Need to build all the related packages to wasm as instructed in Justfile for this test to run.
//This is kind of like an integration test because we need a non wasm environment for jsonschema validation to run currently.
#[cfg(test)]
mod tests {
    use super::*;
    use jsonschema::is_valid;
    use serde_json::{json, Value};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_json_schemas() {

        //Hold Events for testing creation of events from triggers
          // Initialize the event counter
        let event_count = UserData::new(EventCounter::default());

        //load a http plugin as a test action
        let http_plugin_file = Wasm::file("../plugins/anything-http-plugin/target/wasm32-unknown-unknown/release/anything_http_plugin.wasm");
        //load a cron plugin as a test trigger
        let cron_plugin_trigger_file = Wasm::file("../plugins/anything-cron-plugin/target/wasm32-unknown-unknown/release/anything_cron_plugin.wasm");

        //Create Http Plugin
        let http_manifest = Manifest::new([http_plugin_file]);
        println!("{:?}", http_manifest);
        let mut http_plugin = Plugin::new(&http_manifest, [], false).unwrap();

        //Create Cron Plugin
        let cron_manifest = Manifest::new([cron_plugin_trigger_file]);
        println!("{:?}", cron_manifest);
        // let mut cron_plugin = Plugin::new(&cron_manifest, [], false).unwrap();
        let mut cron_plugin = PluginBuilder::new(cron_manifest)
        .with_wasi(false)
        .with_function(
            "create_event",
            [ValType::I64],
            [ValType::I64],
            event_count.clone(), //Resources liek sql that need to be piped into implementations
            create_event, //the host function
        )
        .build()
        .unwrap();

        //Call Register on HTTP Plugin
        let http_register_response = http_plugin.call::<&str, &str>("register", "").unwrap();
        println!("{:?}", http_register_response);
        //Call Register on Cron Plugin
        let cron_register_response = cron_plugin.call::<&str, &str>("register", "").unwrap();
        println!("{:?}", cron_register_response);

        //Parse HTTP Response
        let http_plugin_registration_object: AnythingPlugin =
            serde_json::from_str(&http_register_response)
                .expect("Failed to deserialize Plugin Registration JSON");

        let http_input_schema = http_plugin_registration_object.input_schema;
        println!("http_input_schema {:?}\n", http_input_schema);
        let http_input = http_plugin_registration_object.input;

        //Parse Cron Response
        let cron_plugin_registration_object: AnythingPlugin =
            serde_json::from_str(&cron_register_response)
                .expect("Failed to deserialize Plugin Registration JSON");

        let cron_input_schema = cron_plugin_registration_object.input_schema;
        println!("cron_input_schema {:?}\n", cron_input_schema);
        let cron_input = cron_plugin_registration_object.input;

        //Asserts HTTP Response Is Good
        assert!(
            is_valid(&http_input_schema, &http_input),
            "HTTP Input does not match the expected schema"
        );

        //Asserts Cron Response Is Good
        assert!(
            is_valid(&cron_input_schema, &cron_input),
            "Cron Input does not match the expected schema"
        );

        //Execute HTTP Plugin
        let http_execute_res = http_plugin
            .call::<&str, &str>("execute", &http_input.to_string())
            .unwrap();

        println!("http_execute_res {:?}", http_execute_res);

        //Execute Cron Plugin
        let cron_execute_res = cron_plugin
            .call::<&str, &str>("execute", &cron_input.to_string())
            .unwrap();

        println!("cron_execute_res {:?}", cron_execute_res);

        //Base Anything Schema
        let anything_output_schema = json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["success", "error"]
                },
                "output": {
                    "type": "object"
                },
                "error": {
                    "type": "object"
                }
            },
            "required": ["status"]
        });

        //Get Http Plugin Output Schema
        let http_output_schema = http_plugin_registration_object.output_schema;
        println!("http_output_schema {:?}\n", http_output_schema);

        //Get Cron Plugin Output Schema
        let cron_output_schema = cron_plugin_registration_object.output_schema;
        println!("cron_output_schema {:?}\n", cron_output_schema);

        //Parse HTTP Execute Response
        let http_exec_res_json = serde_json::from_str(&http_execute_res)
            .expect("Failed to deserialize JSON Execute Result");

        println!("http_exec_res_json {:?}", http_exec_res_json);

        //Parse Cron Execute Response
        let cron_exec_res_json = serde_json::from_str(&cron_execute_res)
            .expect("Failed to deserialize JSON Execute Result");

        println!("cron_exec_res_json {:?}", cron_exec_res_json);

        //Check if http exec output works in anything schema
        assert!(
            is_valid(&anything_output_schema, &http_exec_res_json),
            "Output does not match the expected schema"
        );

        //Check if cron exec output works in anything schema
        assert!(
            is_valid(&anything_output_schema, &cron_exec_res_json),
            "Output does not match the expected schema"
        );

        //Check output works in plugin defined schema ( always compatable )
        //Plugin schema just more specific
        //Check for HTTP
        assert!(
            is_valid(&http_output_schema, &http_exec_res_json),
            "Output does not match the expected schema"
        );

        //Check for Cron
        assert!(
            is_valid(&cron_output_schema, &cron_exec_res_json),
            "Output does not match the expected schema"
        );

        let cron_test_input = serde_json::json!({
            "pattern": "*/2 * * * * * *", //Every 2 seconds
        });

        //TODO: test how to make this plugin really work and really test itd
         // Schedule Cron Plugin Execution
         let expected_minimum_triggers = 2; 

        //TODO: need to use the plugin builder and have host functions to create events
        //https://github.com/extism/extism/tree/main/runtime#host-functions
         for _ in 0..5 {
            let cron_execute_res = cron_plugin
                .call::<&str, &str>("execute", &cron_test_input.to_string())
                .unwrap();
            println!("Scheduled cron_execute_res {:?}", cron_execute_res);

            sleep(Duration::from_secs(2)).await;
        }

        let binding = event_count.get().unwrap();
        let counter = binding.lock().unwrap();

        println!("Counter: {:?}", *counter);
        // Check if the cron plugin was triggered at least twice
        // let counter = event_count.get().unwrap().lock().unwrap();
        // assert!(
        //     *counter >= expected_minimum_triggers,
        //     "Cron Plugin was not triggered the expected number of times"
        // );

        println!("Cron Plugin Test Complete");
    }
}

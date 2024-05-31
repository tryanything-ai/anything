use anything_pdk::{AnythingPlugin, Event}; 
use extism::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use crate::convert::Json; 

// Define a simple counter to track the number of events
type EventCounter = usize; 
// Implement the create_event host function
host_fn!(create_event(user_data: EventCounter; event: Json<Event>) -> String {
    let counter = user_data.get()?;
    let mut counter = counter.lock().unwrap();
    *counter += 1;
    println!("Event created: {:?}", counter);
    Ok("Success".to_string())
});


fn main() {
    println!("Run `cargo test` to execute the tests.");
}

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

        //Test if the input schema of triggers includes cron_expression atleast
         //Base Trigger Input Schema
         let anything_trigger_input_schema = json!({
            "type": "object",
            "properties": {
                "cron_expression": {
                    "type": "string",
                },
            },
            "required": ["cron_expression"],
        });

        assert!(
            is_valid(&anything_trigger_input_schema, &cron_input),
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

        //This will be one more then expected triggers because it is called once above to validate schemas
         let expected_triggers = 3;

        //TODO: need to use the plugin builder and have host functions to create events
        //https://github.com/extism/extism/tree/main/runtime#host-functions
         for _ in 0..expected_triggers - 1  {
            let cron_execute_res = cron_plugin
                .call::<&str, &str>("execute", &cron_test_input.to_string())
                .unwrap();
            println!("Scheduled cron_execute_res {:?}", cron_execute_res);

            sleep(Duration::from_secs(2)).await;
        }

        let binding = event_count.get().unwrap();
        let counter = binding.lock().unwrap();

        println!("Counter: {:?}", *counter);

        assert!(
            *counter == expected_triggers,
            "Cron Plugin was not triggered the expected number of times"
        );

        println!("Cron Plugin Test Complete");
    }
}

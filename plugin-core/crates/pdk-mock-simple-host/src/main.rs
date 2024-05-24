use anything_pdk::AnythingPlugin;
use extism::*;

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

    #[test]
    fn test_json_schemas() {
        //load a local plugin
        let http_plugin_file = Wasm::file("../plugins/anything-http-plugin/target/wasm32-unknown-unknown/release/anything_http_plugin.wasm");
        let cron_plugin_trigger_file = Wasm::file("../plugins/anything-cron-plugin/target/wasm32-unknown-unknown/release/anything_cron_plugin.wasm");

        //Create Http Plugin
        let http_manifest = Manifest::new([http_plugin_file]);
        println!("{:?}", http_manifest);
        let mut http_plugin = Plugin::new(&http_manifest, [], false).unwrap();

        //Create Cron Plugin
        let cron_manifest = Manifest::new([cron_plugin_trigger_file]);
        println!("{:?}", cron_manifest);
        let mut cron_plugin = Plugin::new(&cron_manifest, [], false).unwrap();

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
    }
}

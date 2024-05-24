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
        let file = Wasm::file("../plugins/anything-http-plugin/target/wasm32-unknown-unknown/release/anything_http_plugin.wasm");

        let manifest = Manifest::new([file]);
        println!("{:?}", manifest);
        let mut plugin = Plugin::new(&manifest, [], false).unwrap();
        //TODO: call register, then execute, then evaluate the output and inputes etc against schemas
        let register_res = plugin.call::<&str, &str>("register", "").unwrap();

        println!("{:?}", register_res);

        let plugin_registration_object: AnythingPlugin =
            serde_json::from_str(&register_res).expect("Failed to deserialize Plugin Registration JSON");

        let input_schema = plugin_registration_object.input_schema;
        println!("input_schema {:?}\n", input_schema);
        let input = plugin_registration_object.input;

        assert!(
            is_valid(&input_schema, &input),
            "Input does not match the expected schema"
        );

        let execute_res = plugin
            .call::<&str, &str>("execute", &input.to_string())
            .unwrap();

        println!("{:?}", execute_res);

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

        let output_schema = plugin_registration_object.output_schema;
        println!("output_schema {:?}\n", output_schema);

        let exec_res_json =
            serde_json::from_str(&execute_res).expect("Failed to deserialize JSON Execute Result");

        println!("{:?}", exec_res_json);
        //Check output works in anything schema
        assert!(
            is_valid(&anything_output_schema, &exec_res_json),
            "Output does not match the expected schema"
        );

        //Check output works in plugin defined schema ( always compatable )
        //Plugin schema just more specific
        assert!(
            is_valid(&output_schema, &exec_res_json),
            "Output does not match the expected schema"
        );
    }
}

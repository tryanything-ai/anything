// create a main
// have it load plugin from file
// have it call register on them
// have it call execute on them

// use anything_pdk::*;
// use extism::*;

// fn main() {
// let url =
//     Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");
// let manifest = Manifest::new([url]);
// let mut plugin = Plugin::new(&manifest, [], true).unwrap();
// let res = plugin
//     .call::<&str, &str>("count_vowels", "Hello, world!")
//     .unwrap();
// println!("{}", res);
// }

use anything_pdk::Action;
use extism::*;
// use jsonschema::{Draft, JSONSchema};
// use serde_json::json;

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

        let action: Action =
            serde_json::from_str(&register_res).expect("Failed to deserialize JSON Action");

        let input_schema = action.input_schema;
        println!("input_schema {:?}\n", input_schema);
        let input = action.input;

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

        let output_schema = action.output_schema;
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

        //check output schema follows anything schema output schema
        // assert!(
        //     is_valid(&anything_output_schema, &output_schema),
        //     // compiled_output_schema.is_valid(&anything_output_schema),
        //     "Output schema does not match the expected schema"
        // );

        // let output = serde_json::from_str::<Value>(&execute_res).expect("Invalid JSON");

        // let compiled_output = JSONSchema::compile(&output).expect("An invalid schema");
        // //check that the executions return matches the defined output schema
        // assert!(
        //     compiled_output.is_valid(&compiled_output_schema),
        //     "Output does not match the expected schema"
        // );
    }
}

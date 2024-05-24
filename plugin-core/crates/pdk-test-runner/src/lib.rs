use anything_pdk::{AnythingPlugin, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use xtp_test;

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {
    let Json(plugin): Json<AnythingPlugin> = xtp_test::call("register", "")?;

    // Testing basic top level info on trigger is returned.
    // xtp_test::assert!("plugin trigger is false", plugin.trigger == false);

    xtp_test::assert!(
        "plugin plugin_id is non-empty",
        !plugin.plugin_id.is_empty()
    );  


    xtp_test::assert!("label is non-empty", !plugin.label.is_empty());
    xtp_test::assert!("plugin icon is non-empty", !plugin.icon.is_empty());
    xtp_test::assert!(
        "plugin description is non-empty",
        !plugin.description.is_empty()
    );

    // Validate the config field
    xtp_test::assert!(
        "input is valid JSON and has keys",
        validate_schema(&plugin.input)
    );

    // Validate that the config schema is real
    // Validate the config field
    xtp_test::assert!(
        "input_schema is valid JSON and has keys",
        validate_schema(&plugin.input_schema)
    );
    Ok(())
}

//TODO: run integration tests with jsonschema rs in the "simple mock host"
// Function to validate the config field
fn validate_schema(config: &Value) -> bool {
    // Check if config is an object and contains at least one key
    if let Some(config_obj) = config.as_object() {
        return !config_obj.is_empty();
    }
    false
}

use anything_pdk::{Action, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use xtp_test;

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {

    let Json(action): Json<Action> = xtp_test::call("register", "")?;

    // Testing basic top level info on trigger is returned.
    xtp_test::assert!("action trigger is false", action.trigger == false);

    xtp_test::assert!(
        "action plugin_id is non-empty",
        !action.plugin_id.is_empty()
    );

    xtp_test::assert!(
        "action action_name is non-empty",
        !action.action_name.is_empty()
    );
    xtp_test::assert!("action_label is non-empty", !action.action_label.is_empty());
    xtp_test::assert!("action icon is non-empty", !action.icon.is_empty());
    xtp_test::assert!(
        "action description is non-empty",
        !action.description.is_empty()
    );

    // Validate the config field
    xtp_test::assert!(
        "input is valid JSON and has keys",
        validate_schema(&action.input)
    );

    // Validate that the config schema is real
    // Validate the config field
    xtp_test::assert!(
        "input_schema is valid JSON and has keys",
        validate_schema(&action.input_schema)
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

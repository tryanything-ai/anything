use anything_pdk::{Action, Event, Handle, Log};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use xtp_test;

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {
    // call a function from some Extism plugin (you'll link these up in the CLI command to run the test),
    // passing in some data and getting back a string (`callString` is a helper for string output)
    let config = serde_json::json!({
        "method": "GET",
        "url": "http://example.com",
        "headers": {},
        "body": ""
    });

    let res: String = xtp_test::call("execute", config.clone())?;
    // assert the count of the vowels is correct, giving the test case a name (which will be shown in the CLI output)
    // using the macro version here will also capture filename and line number
    xtp_test::assert_eq!("response is as expected", res, config.to_string());

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
        "config is valid JSON and has keys",
        validate_config(&action.config)
    );

    // Validate that the config schema is real
    // Validate the config field
    xtp_test::assert!(
        "config_schema is valid JSON and has keys",
        validate_config(&action.config_schema)
    );
    Ok(())
}

//TODO: run integration tests with jsonschema rs in the "simple mock host"
// Function to validate the config field
fn validate_config(config: &Value) -> bool {
    // Check if config is an object and contains at least one key
    if let Some(config_obj) = config.as_object() {
        return !config_obj.is_empty();
    }
    false
}

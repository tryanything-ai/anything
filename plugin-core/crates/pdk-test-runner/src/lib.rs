use extism_pdk::*;
use serde::{Deserialize, Serialize};
use xtp_test;

#[derive(Serialize, Deserialize)]
struct Action {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: String,
}

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {
    // call a function from some Extism plugin (you'll link these up in the CLI command to run the test),
    // passing in some data and getting back a string (`callString` is a helper for string output)
    let res: String = xtp_test::call("execute", "carls test input")?;
    // assert the count of the vowels is correct, giving the test case a name (which will be shown in the CLI output)
    // using the macro version here will also capture filename and line number
    xtp_test::assert_eq!("response is as expected", res, "carls test input");

    let Json(action): Json<Action> = xtp_test::call("register", "")?;

    xtp_test::assert_eq!("action id is 1", action.id, "1");

    Ok(())
}

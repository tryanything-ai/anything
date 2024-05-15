use extism_pdk::*;
use xtp_test;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Count {
    count: usize,
    total: usize,
    vowels: String,
}

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {
    // call a function from some Extism plugin (you'll link these up in the CLI command to run the test),
    // passing in some data and getting back a string (`callString` is a helper for string output)
    let Json(res): Json<Count> = xtp_test::call("count_vowels", "some input")?;
    // assert the count of the vowels is correct, giving the test case a name (which will be shown in the CLI output)
    // using the macro version here will also capture filename and line number
    xtp_test::assert_eq!("count_vowels of 'some input'", res.count, 4);

    // create a group of tests, which will be run together and reset after the group is complete
    xtp_test::group("count_vowels maintains state", || {
        let mut accum_total = 0;
        let expected_final_total = 12;
        for i in 0..3 {
            let Json(res): Json<Count> = xtp_test::call("count_vowels", "this is a test")?;
            accum_total += res.count;
            xtp_test::assert_eq("total count increased", accum_total, 4 * (i + 1));
        }

        xtp_test::assert_eq(
            "expected total at and of test",
            accum_total,
            expected_final_total,
        );
        Ok(())
    })?;

    Ok(())
}

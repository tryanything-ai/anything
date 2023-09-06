#[test]
fn nodes() {
    let t = trybuild::TestCases::new();
    t.pass("tests/nodes/01-simple.rs");
    t.compile_fail("tests/nodes/02-no-input-or-output.rs");
    t.compile_fail("tests/nodes/03-no-run-fn.rs");
    t.pass("tests/nodes/04-aggregate.rs");
}

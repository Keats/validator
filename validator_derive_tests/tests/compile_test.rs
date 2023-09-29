#[cfg(feature = "test_ui")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/**/*.rs");
    t.pass("tests/run-pass/**/*.rs");
}

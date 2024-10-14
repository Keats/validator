#[cfg(feature = "test_ui")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    #[cfg(not(feature = "nightly"))]
    t.compile_fail("tests/compile-fail/**/*.rs");
    t.pass("tests/run-pass/**/*.rs");
}

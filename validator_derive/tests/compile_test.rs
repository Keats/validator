extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();
    let cfg_mode = mode.parse().expect("Invalid mode");

    config.target_rustcflags = Some("-L target/debug/ -L target/debug/deps/".to_string());
    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    compiletest::run_tests(&config);
}

#[test]
fn test_compile_fail() {
    run_mode("compile-fail");
}

#[test]
fn test_run_pass() {
    run_mode("run-pass");
}

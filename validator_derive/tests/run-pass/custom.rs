#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(custom = "validate_something")]
    s: String,
}

fn validate_something(s: &str) -> Option<String> {
    Some(s.to_string())
}

fn main() {}

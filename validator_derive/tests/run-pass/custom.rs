#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::{Validate, ValidationError};

#[derive(Validate)]
struct Test {
    #[validate(custom = "validate_something")]
    s: String,
}

#[derive(Validate)]
struct TestPath {
    #[validate(custom = "::validate_something")]
    s: String,
}

fn validate_something(s: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn main() {}

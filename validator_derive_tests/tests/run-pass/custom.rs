use std::borrow::Cow;
use validator::{Validate, ValidationError};

#[derive(Validate)]
struct Test {
    #[validate(custom(function = crate::validate_something))]
    s: String,
}

#[derive(Validate)]
struct TestPath {
    #[validate(custom(function = "crate::validate_something"))]
    s: String,
    #[validate(custom(function = "crate::validate_number"))]
    n: u8,
    #[validate(custom(function = "crate::validate_something"))]
    r: Cow<'static, str>
}

fn validate_something(_s: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn validate_number(_s: u8) -> Result<(), ValidationError> {
    Ok(())
}

fn main() {}

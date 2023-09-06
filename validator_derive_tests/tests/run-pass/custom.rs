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
}

fn validate_something(_s: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn main() {}

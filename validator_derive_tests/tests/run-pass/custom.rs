use validator::{Validate, ValidationError};

#[derive(Validate)]
struct Test {
    #[validate(custom = "crate::validate_something")]
    s: String,
}

#[derive(Validate)]
struct TestPath {
    #[validate(custom = "crate::validate_something")]
    s: String,
}

fn validate_something(_s: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn main() {}

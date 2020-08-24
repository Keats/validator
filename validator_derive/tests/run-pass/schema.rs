use validator::{Validate, ValidationError};

#[derive(Validate)]
#[validate(schema(function = "hey"))]
struct Test {
    s: String,
}

fn hey(_: &Test) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Validate)]
#[validate(schema(function = "hey2", skip_on_field_errors = false))]
struct Test2 {
    s: String,
}

fn hey2(_: &Test2) -> Result<(), ValidationError> {
    Ok(())
}

fn main() {}

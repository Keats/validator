use validator::{Validate, ValidationError};

fn hello_world(_: &str, _arg: &mut String) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Validate)]
struct Test {
    #[validate(custom(function = "hello_world", arg = "&'a mut String"))]
    s: String,
}

fn main() {}

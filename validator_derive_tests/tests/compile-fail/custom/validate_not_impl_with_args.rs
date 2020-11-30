use validator::{Validate, ValidationError};

fn hello_world(_: &str, _arg: i32) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Validate)]
struct Test {
    #[validate(custom(function = "hello_world", arg = "i32"))]
    s: String,
}

fn main() {
    let test = Test{s: "Test".to_string()};
    test.validate();
}

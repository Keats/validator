use validator::{Validate, ValidationError};

fn hello_world(_: &str, _arg: &Arg) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Validate)]
#[validate(context = "Arg<'a>")]
struct Test {
    #[validate(custom(function = "hello_world", use_context))]
    s: String,
}

struct Arg<'a> {
    arg: &'a str,
}

fn main() {}

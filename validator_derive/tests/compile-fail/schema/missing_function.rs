#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid schema level validation: `function` is required
#[validate(schema())]
struct Test {
    s: i32,
}

fn hey(_: &Test) -> Option<(String, String)> {
    None
}


fn main() {}

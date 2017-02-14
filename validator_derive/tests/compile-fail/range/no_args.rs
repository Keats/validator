#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: Validator `range` requires 2 arguments: `min` and `max`
struct Test {
    #[validate(range())]
    s: i32,
}

fn main() {}

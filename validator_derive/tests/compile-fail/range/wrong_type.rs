#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: Validator `range` can only be used on number types but found `String`
struct Test {
    #[validate(range(min = 10.0, max = 12.0))]
    s: String,
}

fn main() {}

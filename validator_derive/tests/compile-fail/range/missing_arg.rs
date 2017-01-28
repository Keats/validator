#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: Validator `range` requires 2 arguments: `min` and `max`
struct Test {
    #[validate(range(min = 2.0))]
    s: i32,
}

fn main() {}

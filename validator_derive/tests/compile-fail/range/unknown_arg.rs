#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: unknown argument `mi` for validator `range` (it only has `min`, `max`)
struct Test {
    #[validate(range(mi = 2, max = 3))]
    s: i32,
}

fn main() {}

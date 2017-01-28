#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: invalid argument for `custom` validator: only strings are allowed
struct Test {
    #[validate(custom = 2)]
    s: String,
}

fn main() {}

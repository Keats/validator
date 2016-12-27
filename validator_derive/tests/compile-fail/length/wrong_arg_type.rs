#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: invalid argument type for `min` of `length` validator: only integers are allowed
struct Test {
    #[validate(length(min = "2"))]
    s: String,
}

fn main() {}

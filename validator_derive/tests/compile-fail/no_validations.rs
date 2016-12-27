#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: it needs at least one validator
struct Test {
    #[validate()]
    s: String,
}

fn main() {}

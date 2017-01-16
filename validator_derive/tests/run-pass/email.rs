#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(email)]
    s: String,
}

fn main() {}

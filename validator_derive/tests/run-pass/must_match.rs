#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(must_match = "s2")]
    s: String,
    s2: String,

    #[validate(must_match = "s4")]
    s3: usize,
    s4: usize,
}

fn main() {}

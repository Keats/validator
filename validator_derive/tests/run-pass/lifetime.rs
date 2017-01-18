#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
struct Test<'a> {
    #[validate(length(min = 1))]
    s: &'a str,
    #[validate(length(min = 1, max = 2))]
    s2: &'a str,
    #[validate(length(equal = 1))]
    s3: &'a str,
    #[validate(length(max = 1))]
    s4: &'a str,
}

fn main() {}

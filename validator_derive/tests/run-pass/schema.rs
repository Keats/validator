#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
#[validate(schema(function = "hey"))]
struct Test {
    s: String,
}

fn hey(_: &Test) -> Option<(String, String)> {
    None
}

#[derive(Validate)]
#[validate(schema(function = "hey2", skip_on_field_errors = false))]
struct Test2 {
    s: String,
}

fn hey2(_: &Test2) -> Option<(String, String)> {
    None
}

fn main() {}

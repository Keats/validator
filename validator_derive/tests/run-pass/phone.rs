#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[cfg(feature = "phone")]
#[derive(Validate)]
struct Test {
    #[validate(phone)]
    s: String,
}

fn main() {}

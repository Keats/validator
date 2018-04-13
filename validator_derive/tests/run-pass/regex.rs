extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;
use regex::Regex;

lazy_static! {
    static ref RE2: Regex = Regex::new(r"^[a-z]{2}$").unwrap();
}

#[derive(Validate)]
struct Test {
    #[validate(regex = "RE2")]
    s: String,
}

#[derive(Validate)]
struct TestPath {
    #[validate(regex = "::RE2")]
    s: String,
}

fn main() {}

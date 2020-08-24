use lazy_static::lazy_static;
use regex::Regex;
use validator::Validate;

lazy_static! {
    static ref RE2: Regex = Regex::new(r"^[a-z]{2}$").unwrap();
}

#[derive(Validate)]
struct Test {
    #[validate(regex = "crate::RE2")]
    s: String,
}

#[derive(Validate)]
struct TestPath {
    #[validate(regex = "crate::RE2")]
    s: String,
}

fn main() {}

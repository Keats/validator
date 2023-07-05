use lazy_regex::lazy_regex;
use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;

static RE2: Lazy<Regex> = lazy_regex!(r"^[a-z]{2}$");

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

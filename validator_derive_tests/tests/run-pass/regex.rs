use regex::Regex;
use std::sync::LazyLock;
use validator::Validate;

static RE2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z]{2}$").unwrap());

#[derive(Validate)]
struct Test {
    #[validate(regex(path = "*crate::RE2"))]
    s: String,
}

#[derive(Validate)]
struct TestPath {
    #[validate(regex(path = *crate::RE2))]
    s: String,
}

fn main() {}

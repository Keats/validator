
use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;

static RE2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z]{2}$").unwrap()
});


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

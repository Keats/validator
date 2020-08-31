use validator::Validate;

#[derive(Validate)]
struct Test<'a> {
    #[validate(length(min = 1))]
    s: &'a str,
    #[validate(length(min = 1))]
    s2: Option<&'a str>,
}

fn main() {}

use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(length(min = 1, equal = 2))]
    s: String,
}

fn main() {}

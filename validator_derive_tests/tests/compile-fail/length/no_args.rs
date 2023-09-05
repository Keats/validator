use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(range(min = 5, max = 10))]
    #[validate(length())]
    s: String,
}

fn main() {}

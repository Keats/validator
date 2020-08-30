use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(range(min = 10.0, max = 12.0))]
    s: String,
}

fn main() {}

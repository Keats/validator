use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(custom(function = 2))]
    s: String,
}

fn main() {}

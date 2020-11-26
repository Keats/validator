use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(custom = 2)]
    s: String,
}

fn main() {}

use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(custom(arg = "i64"))]
    s: String,
}

fn main() {}

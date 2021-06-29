use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(custom(function = "hello_world", arg = "String"))]
    s: String,
}

fn main() {}

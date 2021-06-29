use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(custom(function = "hello_world", arg = "dyn FnOnce(&str) -> usize"))]
    s: String,
}

fn main() {}

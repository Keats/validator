use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(range())]
    s: i32,
}

fn main() {}

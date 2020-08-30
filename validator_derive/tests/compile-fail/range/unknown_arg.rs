use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(range(mi = 2, max = 3))]
    s: i32,
}

fn main() {}

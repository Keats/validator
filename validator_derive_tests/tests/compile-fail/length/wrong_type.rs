use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(length(min = 5))]
    s: usize,
}

fn main() {}

use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(length(eq = 2))]
    s: String,
}

fn main() {}

use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(email)]
    s: String,
}

fn main() {}

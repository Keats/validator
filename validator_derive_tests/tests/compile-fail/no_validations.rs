use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate()]
    s: String,
}

fn main() {}

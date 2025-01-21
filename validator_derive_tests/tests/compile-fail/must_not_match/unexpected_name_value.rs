use validator::Validate;

#[derive(Validate)]
struct Email {
    #[validate(not_a(other = "validator"))]
    email: String,
}

fn main() {}

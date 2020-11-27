use validator::Validate;

#[derive(Validate)]
struct Email {
    #[validate(not_a = "validator")]
    email: String,
}

fn main() {}

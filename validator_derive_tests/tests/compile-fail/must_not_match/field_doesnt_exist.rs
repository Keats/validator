use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(must_not_match = "password2")]
    password: String,
}

fn main() {}

use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(must_not_match(other = "password2"))]
    password: String,
    password2: i32,
}

fn main() {}

use validator::Validate;

#[derive(Validate)]
struct PII {
    #[validate(not_a_list(a, b, c))]
    email: String,
}

fn main() {}

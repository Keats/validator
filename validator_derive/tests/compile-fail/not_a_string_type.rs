use validator::Validate;

#[derive(Validate)]
struct Register {
    #[validate(email)]
    email: Vec<u8>,
}

fn main() {}

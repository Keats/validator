use validator::Validate;

#[cfg(feature = "phone")]
#[derive(Validate)]
struct Test {
    #[validate(phone)]
    s: String,
}

fn main() {}

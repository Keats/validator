use validator::Validate;

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    Validate,
)]
pub struct Message {
    #[validate(length(min = 1i64, max = 2048i64))]
    pub text: String,
}

fn main() {}

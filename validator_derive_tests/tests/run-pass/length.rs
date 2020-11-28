use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(length(min = 1))]
    s: String,
    #[validate(length(min = 1, max = 2))]
    s2: String,
    #[validate(length(equal = 1))]
    s3: String,
    #[validate(length(max = 1))]
    s4: String,

    #[validate(length(min = 1))]
    s5: Vec<String>,
    #[validate(length(min = 1, max = 2))]
    s6: Vec<String>,
    #[validate(length(equal = 1))]
    s7: Vec<String>,
    #[validate(length(max = 1))]
    s8: Vec<String>,
}

fn main() {}

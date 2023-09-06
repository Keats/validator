use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(nested)]
    nested: Nested,
}

struct Nested {
    value: String,
}

fn main() {}

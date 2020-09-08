use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate]
    nested: Nested,
}

struct Nested {
    value: String,
}

fn main() {}

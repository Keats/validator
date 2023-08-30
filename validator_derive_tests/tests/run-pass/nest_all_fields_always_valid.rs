use validator::Validate;

#[derive(Validate)]
#[validate(nest_all_fields)]
struct Test {
    #[validate(always_valid)]
    nested: Nested,
}

struct Nested {
    value: String,
}

fn main() {}

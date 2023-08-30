use validator::Validate;

#[derive(Validate)]
#[validate(nest_all_fields)]
struct Test {
    nested: Nested,
}

struct Nested {
    value: String,
}

fn main() {}

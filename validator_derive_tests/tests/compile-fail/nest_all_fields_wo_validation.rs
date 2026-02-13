use validator::Validate;

#[derive(Validate)]
#[validate(nest_all_fields)]
struct Test {
    nested: Nested,
    unchecked: u8,
}

#[derive(Validate)]
struct Nested {
    value: String,
}

fn main() {}

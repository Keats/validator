#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: no method named `nested_validate` found for type `Nested` in the current scope [E0599]
//~^^ HELP: items from traits can only be used if the trait is implemented and in scope
struct Test {
    #[validate]
    nested: Nested,
}

struct Nested {
    value: String
}

fn main() {}

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `password`: invalid argument for `must_match` validator: field doesn't exist in struct
struct Test {
    #[validate(must_match = "password2")]
    password: String,
}

fn main() {}

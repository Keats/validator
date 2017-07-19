#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid argument for `must_match` validator of field `password`: types of field can't match
struct Test {
    #[validate(must_match = "password2")]
    password: String,
    password2: i32,
}

fn main() {}

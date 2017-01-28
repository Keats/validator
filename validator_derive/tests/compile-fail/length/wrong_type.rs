#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: Validator `length` can only be used on types `String`, `&str` or `Vec` but found `usize`
struct Test {
    #[validate(length())]
    s: usize,
}

fn main() {}

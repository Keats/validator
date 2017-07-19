#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Validator `length` can only be used on types `String`, `&str` or `Vec` but found `usize`
struct Test {
    #[validate(length())]
    s: usize,
}

fn main() {}

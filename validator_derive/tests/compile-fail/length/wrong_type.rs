#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: "Validator `length` can only be used on types `String`, `&str`, Cow<'_,str> or `Vec` but found `{}` for field `{}`"
struct Test {
    #[validate(length())]
    s: usize,
}

fn main() {}

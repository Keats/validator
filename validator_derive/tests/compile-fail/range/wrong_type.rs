use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Validator `range` can only be used on number types but found `String`
struct Test {
    #[validate(range(min = 10.0, max = 12.0))]
    s: String,
}

fn main() {}

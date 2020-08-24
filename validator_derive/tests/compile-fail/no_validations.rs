use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: it needs at least one validator
struct Test {
    #[validate()]
    s: String,
}

fn main() {}

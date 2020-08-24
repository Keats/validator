use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: unknown argument `eq` for validator `length` (it only has `min`, `max`, `equal`)
struct Test {
    #[validate(length(eq = 2))]
    s: String,
}

fn main() {}

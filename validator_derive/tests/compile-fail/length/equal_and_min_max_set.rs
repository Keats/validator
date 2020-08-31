use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: both `equal` and `min` or `max` have been set in `length` validator: probably a mistake
struct Test {
    #[validate(length(min = 1, equal = 2))]
    s: String,
}

fn main() {}

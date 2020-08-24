use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: unknown argument `mi` for validator `range` (it only has `min`, `max`)
struct Test {
    #[validate(range(mi = 2, max = 3))]
    s: i32,
}

fn main() {}

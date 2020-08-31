use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: invalid argument for `custom` validator: only strings are allowed
struct Test {
    #[validate(custom = 2)]
    s: String,
}

fn main() {}

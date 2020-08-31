use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid argument for `must_match` validator of field `password`: the other field doesn't exist in struct
struct Test {
    #[validate(must_match = "password2")]
    password: String,
}

fn main() {}

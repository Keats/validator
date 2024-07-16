use validator as validator_renamed;

mod inner {
    use super::validator_renamed;

    mod validator {}

    #[derive(validator_renamed::Validate)]
    #[validate(crate = "validator_renamed")]
    struct Test {
        #[validate(url)]
        val: String,
    }
}

fn main() {}

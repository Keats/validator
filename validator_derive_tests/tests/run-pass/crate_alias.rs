use validator as validator_renamed;

mod inner {
    use super::validator_renamed;

    mod validator {}

    fn validate_fn(_: &str) -> Result<(), validator_renamed::ValidationError> {
        Ok(())
    }

    #[derive(validator_renamed::Validate)]
    #[validate(crate = "validator_renamed")]
    struct Test {
        #[validate(url)]
        url: String,
        #[validate(email)]
        email: String,
        #[validate(length(min = 1, max = 10))]
        length: String,
        #[validate(range(min = 1, max = 10))]
        range: i32,
        #[validate(custom(function = "validate_fn"))]
        custom: String,
    }
}

fn main() {}

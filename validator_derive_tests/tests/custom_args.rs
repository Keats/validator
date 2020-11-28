use validator::{Validate, ValidateArgs, ValidationError};

fn valid_generic_custom_fn<T>(_: &T) -> Result<(), ValidationError> {
    Ok(())
}

fn valid_custom_fn(_: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn invalid_custom_fn(_: &str) -> Result<(), ValidationError> {
    Err(ValidationError::new("meh"))
}

#[derive(Debug, Validate)]
struct TestGenericStruct<'a, T: serde::ser::Serialize> {
    #[validate(custom = "valid_generic_custom_fn")]
    generic: &'a T,

    #[validate(custom = "valid_custom_fn")]
    val: String,
}

impl<'v_a, 'a, T: serde::ser::Serialize> ValidateArgs<'v_a> for TestGenericStruct<'a, T> {
    type Args = ();

    fn validate_args(&self, _args: Self::Args) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

#[test]
fn validate_generic_struct_custom_fn_ok() {
    let generic = 746460_i128;

    let s = TestGenericStruct { generic: &generic, val: "hello".to_string() };

    assert!(s.validate().is_ok());
    assert!(s.validate_args(()).is_ok());
}

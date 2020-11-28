use validator::{Validate, ValidationError};

fn valid_custom_fn(_: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn invalid_custom_fn(_: &str) -> Result<(), ValidationError> {
    Err(ValidationError::new("meh"))
}

#[derive(Debug, Validate)]
struct TestGenericStruct<'a, T> {
    generic: &'a T,

    #[validate(custom = "valid_custom_fn")]
    val: String,
}

#[test]
fn validate_generic_struct_custom_fn_ok() {
    let generic = 746460_i128;
    
    let s = TestGenericStruct { 
        generic: &generic,
        val: "hello".to_string()
    };

    assert!(s.validate().is_ok());
}